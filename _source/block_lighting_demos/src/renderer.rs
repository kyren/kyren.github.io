use std::u32;

use failure::{ensure, err_msg, Error};
use web_sys::{
    HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlShader, WebGlTexture,
};

use crate::f32_array::F32Array;
use crate::util::js_err;

pub struct Renderer {
    context: WebGlRenderingContext,
    program: WebGlProgram,

    main_texture: WebGlTexture,
    lighting_texture: WebGlTexture,

    work_array: F32Array,
    draw_buffer: WebGlBuffer,
}

impl Renderer {
    pub fn new(
        context: WebGlRenderingContext,
        main_image: HtmlImageElement,
    ) -> Result<Renderer, Error> {
        let vert_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            VERTEX_SHADER,
        )?;
        let frag_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            FRAGMENT_SHADER,
        )?;
        let program = link_program(&context, [vert_shader, frag_shader].iter())?;

        let main_texture = context
            .create_texture()
            .ok_or_else(|| err_msg("cannot create main texture"))?;

        let lighting_texture = context
            .create_texture()
            .ok_or_else(|| err_msg("cannot create lighting texture"))?;

        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&main_texture));
        context
            .tex_image_2d_with_u32_and_u32_and_image(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGBA as i32,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                &main_image,
            )
            .map_err(js_err)?;

        let work_array = F32Array::new();
        let draw_buffer = context
            .create_buffer()
            .ok_or_else(|| err_msg("could not create buffer"))?;

        Ok(Renderer {
            context,
            program,
            main_texture,
            lighting_texture,
            work_array,
            draw_buffer,
        })
    }

    pub fn start(
        &mut self,
        lighting_width: u32,
        lighting_height: u32,
        light: &[u8],
    ) -> Result<(), Error> {
        self.work_array.clear();
        self.context.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.lighting_texture),
        );
        self.context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGB as i32,
                lighting_width as i32,
                lighting_height as i32,
                0,
                WebGlRenderingContext::RGB,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(light),
            )
            .map_err(js_err)
    }

    pub fn draw(
        &mut self,
        screen: (f32, f32, f32, f32),
        texture: (f32, f32, f32, f32),
        color: (f32, f32, f32, f32),
    ) {
        let mut push_vertex = |x: f32, y: f32, u: f32, v: f32, c: (f32, f32, f32, f32)| {
            self.work_array.push(x);
            self.work_array.push(y);
            self.work_array.push(u);
            self.work_array.push(v);
            self.work_array.push(c.0);
            self.work_array.push(c.1);
            self.work_array.push(c.2);
            self.work_array.push(c.3);
        };

        push_vertex(screen.0, screen.1, texture.0, texture.1, color);
        push_vertex(screen.2, screen.1, texture.2, texture.1, color);
        push_vertex(screen.2, screen.3, texture.2, texture.3, color);

        push_vertex(screen.0, screen.1, texture.0, texture.1, color);
        push_vertex(screen.2, screen.3, texture.2, texture.3, color);
        push_vertex(screen.0, screen.3, texture.0, texture.3, color);
    }

    pub fn finish(&self) -> Result<(), Error> {
        const VERT_FLOATS: i32 = 8;

        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&self.draw_buffer));
        self.work_array.visit(|array| {
            self.context.buffer_data_with_opt_array_buffer(
                WebGlRenderingContext::ARRAY_BUFFER,
                Some(&array.buffer()),
                WebGlRenderingContext::DYNAMIC_DRAW,
            );
        });

        let pos_attrib = self
            .context
            .get_attrib_location(&self.program, "a_position");
        let main_tex_attrib = self
            .context
            .get_attrib_location(&self.program, "a_main_tex");
        let color_attrib = self.context.get_attrib_location(&self.program, "a_color");
        ensure!(
            pos_attrib >= 0 && main_tex_attrib >= 0 && color_attrib >= 0,
            "could not find vertex attribute"
        );

        self.context.vertex_attrib_pointer_with_i32(
            pos_attrib as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            VERT_FLOATS * 4,
            0 * 4,
        );
        self.context.enable_vertex_attrib_array(0);

        self.context.vertex_attrib_pointer_with_i32(
            main_tex_attrib as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            VERT_FLOATS * 4,
            2 * 4,
        );
        self.context.enable_vertex_attrib_array(1);

        self.context.vertex_attrib_pointer_with_i32(
            color_attrib as u32,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            VERT_FLOATS * 4,
            4 * 4,
        );
        self.context.enable_vertex_attrib_array(2);

        self.context.use_program(Some(&self.program));

        self.context
            .active_texture(WebGlRenderingContext::TEXTURE0 + 0);
        self.context.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.lighting_texture),
        );
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::LINEAR as i32,
        );
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        let light_tex_uniform_location = self
            .context
            .get_uniform_location(&self.program, "light_tex")
            .ok_or_else(|| err_msg("cannot find 'light_tex' uniform"))?;
        self.context
            .uniform1iv_with_i32_array(Some(&light_tex_uniform_location), &mut [0]);

        self.context
            .active_texture(WebGlRenderingContext::TEXTURE0 + 1);
        self.context
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.main_texture));
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MAG_FILTER,
            WebGlRenderingContext::NEAREST as i32,
        );
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_S,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_WRAP_T,
            WebGlRenderingContext::CLAMP_TO_EDGE as i32,
        );
        let main_tex_uniform_location = self
            .context
            .get_uniform_location(&self.program, "main_tex")
            .ok_or_else(|| err_msg("cannot find 'main_tex' uniform"))?;
        self.context
            .uniform1iv_with_i32_array(Some(&main_tex_uniform_location), &mut [1]);

        self.context.draw_arrays(
            WebGlRenderingContext::TRIANGLES,
            0,
            self.work_array.len() as i32 / VERT_FLOATS,
        );

        Ok(())
    }
}

const VERTEX_SHADER: &str = r#"
    precision mediump float;

    attribute vec2 a_position;
    attribute vec2 a_main_tex;
    attribute vec4 a_color;

    varying vec4 v_color;
    varying vec2 v_main_tex;
    varying vec2 v_light_tex;

    void main() {
        v_light_tex = a_position;
        v_main_tex = a_main_tex;
        v_color = a_color;
        gl_Position = vec4(a_position * 2.0 - 1.0, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    uniform sampler2D light_tex;
    uniform sampler2D main_tex;

    varying vec4 v_color;
    varying vec2 v_main_tex;
    varying vec2 v_light_tex;

    void main() {
        gl_FragColor = v_color * texture2D(light_tex, v_light_tex) * texture2D(main_tex, v_main_tex);
    }
"#;

fn compile_shader(
    context: &WebGlRenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, Error> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| err_msg("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .map(err_msg)
            .unwrap_or_else(|| err_msg("Unknown error creating shader"))
            .into())
    }
}

fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(
    context: &WebGlRenderingContext,
    shaders: T,
) -> Result<WebGlProgram, Error> {
    let program = context
        .create_program()
        .ok_or_else(|| err_msg("Unable to create shader object"))?;
    for shader in shaders {
        context.attach_shader(&program, shader)
    }
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .map(err_msg)
            .unwrap_or_else(|| err_msg("Unknown error creating program object"))
            .into())
    }
}
