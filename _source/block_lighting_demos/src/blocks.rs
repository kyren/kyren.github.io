use std::u32;

use failure::{err_msg, Error};
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlTexture};

use f32_array::F32Array;
use glutil::{compile_shader, link_program};
use util::js_err;

pub struct BlockLighting {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    work_array: F32Array,

    block_count: (u32, u32),
    block_size: (f32, f32),
    block_state: Vec<bool>,
    block_buffer: Option<(WebGlBuffer, u32)>,

    lighting_texture: WebGlTexture,
}

impl BlockLighting {
    pub fn new(
        context: WebGlRenderingContext,
        blocks_horizontal: u32,
        blocks_vertical: u32,
    ) -> Result<BlockLighting, Error> {
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

        let lighting_texture = context
            .create_texture()
            .ok_or_else(|| err_msg("cannot create lighting texture"))?;
        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&lighting_texture));

        let mut pixels = vec![
            0, 0, 255, 255, 0, 255, 0, 255, 255, 0, 0, 255, 255, 255, 255, 255,
        ];
        context
            .tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGBA as i32,
                2,
                2,
                0,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                Some(&mut pixels),
            )
            .map_err(js_err)?;

        Ok(BlockLighting {
            context,
            program,
            work_array: F32Array::new(),
            block_buffer: None,
            block_count: (blocks_horizontal, blocks_vertical),
            block_size: (1.0 / blocks_horizontal as f32, 1.0 / blocks_vertical as f32),
            block_state: vec![false; blocks_horizontal as usize * blocks_vertical as usize],
            lighting_texture,
        })
    }

    pub fn block_count(&self) -> (u32, u32) {
        self.block_count
    }

    pub fn get_block_state(&mut self, x: u32, y: u32) -> bool {
        self.block_state[block_index(self.block_count.0, x, y)]
    }

    pub fn set_block_state(&mut self, x: u32, y: u32, block_state: bool) -> bool {
        let bi = block_index(self.block_count.0, x, y);
        if self.block_state[bi] != block_state {
            self.block_buffer = None;
            self.block_state[bi] = block_state;
            true
        } else {
            false
        }
    }

    pub fn draw(&mut self) -> Result<(), Error> {
        if self.block_buffer.is_none() {
            self.block_buffer = Some(self.create_block_buffer()?);
        }
        let block_buffer = self.block_buffer.as_ref().unwrap();

        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&block_buffer.0));

        self.context.vertex_attrib_pointer_with_i32(
            0,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            6 * 4,
            0 * 4,
        );
        self.context.enable_vertex_attrib_array(0);

        self.context.vertex_attrib_pointer_with_i32(
            1,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            6 * 4,
            2 * 4,
        );
        self.context.enable_vertex_attrib_array(1);

        self.context.use_program(Some(&self.program));

        let texture_uniform_locaiton = self
            .context
            .get_uniform_location(&self.program, "lighting")
            .ok_or_else(|| err_msg("cannot find 'lighting' unifomr"))?;

        self.context.tex_parameteri(
            WebGlRenderingContext::TEXTURE_2D,
            WebGlRenderingContext::TEXTURE_MIN_FILTER,
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

        self.context.bind_texture(
            WebGlRenderingContext::TEXTURE_2D,
            Some(&self.lighting_texture),
        );
        self.context
            .uniform1iv_with_i32_array(Some(&texture_uniform_locaiton), &mut [0]);

        self.context
            .draw_arrays(WebGlRenderingContext::TRIANGLES, 0, block_buffer.1 as i32);

        Ok(())
    }

    fn create_block_buffer(&mut self) -> Result<(WebGlBuffer, u32), Error> {
        const BLOCK_COLOR: (f32, f32, f32, f32) = (1.0, 1.0, 1.0, 1.0);

        self.work_array.clear();

        fn push_vertex(array: &mut F32Array, x: f32, y: f32) {
            array.push(x);
            array.push(y);
            array.push(BLOCK_COLOR.0);
            array.push(BLOCK_COLOR.1);
            array.push(BLOCK_COLOR.2);
            array.push(BLOCK_COLOR.3);
        };

        for y in 0..self.block_count.1 {
            for x in 0..self.block_count.0 {
                if self.block_state[block_index(self.block_count.0, x, y)] {
                    let x = x as f32 * self.block_size.0;
                    let y = y as f32 * self.block_size.1;
                    let w = self.block_size.0;
                    let h = self.block_size.1;

                    push_vertex(&mut self.work_array, x, y);
                    push_vertex(&mut self.work_array, x + w, y);
                    push_vertex(&mut self.work_array, x + w, y + h);

                    push_vertex(&mut self.work_array, x, y);
                    push_vertex(&mut self.work_array, x + w, y + h);
                    push_vertex(&mut self.work_array, x, y + h);
                }
            }
        }

        let buffer = self
            .context
            .create_buffer()
            .ok_or_else(|| err_msg("could not create buffer"))?;

        self.context
            .bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        self.work_array.visit(|array| {
            self.context.buffer_data_with_opt_array_buffer(
                WebGlRenderingContext::ARRAY_BUFFER,
                Some(&array.buffer()),
                WebGlRenderingContext::STATIC_DRAW,
            );
        });

        Ok((buffer, self.work_array.len() / 6))
    }
}

fn block_index(width: u32, x: u32, y: u32) -> usize {
    y as usize * width as usize + x as usize
}

const VERTEX_SHADER: &str = r#"
    precision mediump float;

    attribute vec2 a_position;
    attribute vec3 a_color;

    varying vec3 v_color;
    varying vec2 v_tex;

    void main() {
        v_color = a_color;
        v_tex = a_position;
        gl_Position = vec4(a_position * 2.0 - 1.0, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    uniform sampler2D lighting;

    varying vec3 v_color;
    varying vec2 v_tex;

    void main() {
        gl_FragColor = vec4(v_color, 1.0) * texture2D(lighting, v_tex);
    }
"#;
