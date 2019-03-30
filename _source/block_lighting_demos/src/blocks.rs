use std::hash::Hasher;
use std::u32;

use failure::{ensure, err_msg, Error};
use twox_hash::XxHash;
use web_sys::{HtmlImageElement, WebGlBuffer, WebGlProgram, WebGlRenderingContext, WebGlTexture};

use crate::f32_array::F32Array;
use crate::glutil::{compile_shader, link_program};
use crate::util::js_err;

const BLOCK_TEX_COORDS: [(f32, f32, f32, f32); 4] = [
    (0.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0, 0.0 / 4.0),
    (1.0 / 4.0, 1.0 / 4.0, 2.0 / 4.0, 0.0 / 4.0),
    (0.0 / 4.0, 2.0 / 4.0, 1.0 / 4.0, 1.0 / 4.0),
    (1.0 / 4.0, 2.0 / 4.0, 2.0 / 4.0, 1.0 / 4.0),
];

const GRASS_TEX_COORDS: [(f32, f32, f32, f32); 4] = [
    (2.0 / 4.0, 1.0 / 4.0, 3.0 / 4.0, 0.0 / 4.0),
    (3.0 / 4.0, 1.0 / 4.0, 4.0 / 4.0, 0.0 / 4.0),
    (2.0 / 4.0, 2.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0),
    (3.0 / 4.0, 2.0 / 4.0, 4.0 / 4.0, 1.0 / 4.0),
];

const LIGHT_TEX_COORDS: [(f32, f32, f32, f32); 4] = [
    (0.0 / 4.0, 3.0 / 4.0, 1.0 / 4.0, 2.0 / 4.0),
    (1.0 / 4.0, 3.0 / 4.0, 2.0 / 4.0, 2.0 / 4.0),
    (0.0 / 4.0, 4.0 / 4.0, 1.0 / 4.0, 3.0 / 4.0),
    (1.0 / 4.0, 4.0 / 4.0, 2.0 / 4.0, 3.0 / 4.0),
];

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockState {
    Empty,
    Foreground,
    Background,
    Light,
}

pub struct BlockLighting {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    work_array: F32Array,

    block_count: (u32, u32),
    block_size: (f32, f32),
    block_state: Vec<BlockState>,
    block_buffer: Option<(WebGlBuffer, u32)>,

    lighting_texture: WebGlTexture,
    tile_texture: WebGlTexture,
}

impl BlockLighting {
    pub fn new(
        context: WebGlRenderingContext,
        tile_image: HtmlImageElement,
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
            255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
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

        let tile_texture = context
            .create_texture()
            .ok_or_else(|| err_msg("cannot create tile texture"))?;
        context.bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&tile_texture));

        context
            .tex_image_2d_with_u32_and_u32_and_image(
                WebGlRenderingContext::TEXTURE_2D,
                0,
                WebGlRenderingContext::RGBA as i32,
                WebGlRenderingContext::RGBA,
                WebGlRenderingContext::UNSIGNED_BYTE,
                &tile_image,
            )
            .map_err(js_err)?;

        Ok(BlockLighting {
            context,
            program,
            work_array: F32Array::new(),
            block_buffer: None,
            block_count: (blocks_horizontal, blocks_vertical),
            block_size: (1.0 / blocks_horizontal as f32, 1.0 / blocks_vertical as f32),
            block_state: vec![
                BlockState::Empty;
                blocks_horizontal as usize * blocks_vertical as usize
            ],
            lighting_texture,
            tile_texture,
        })
    }

    pub fn block_count(&self) -> (u32, u32) {
        self.block_count
    }

    pub fn get_block_state(&mut self, x: u32, y: u32) -> BlockState {
        self.block_state[block_index(self.block_count.0, x, y)]
    }

    pub fn set_block_state(&mut self, x: u32, y: u32, block_state: BlockState) -> bool {
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

        let pos_attrib = self
            .context
            .get_attrib_location(&self.program, "a_position");
        let tile_tex_attrib = self
            .context
            .get_attrib_location(&self.program, "a_tile_tex");
        let color_attrib = self.context.get_attrib_location(&self.program, "a_color");
        ensure!(
            pos_attrib >= 0 && tile_tex_attrib >= 0 && color_attrib >= 0,
            "could not find vertex attribute"
        );

        self.context.vertex_attrib_pointer_with_i32(
            pos_attrib as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            8 * 4,
            0 * 4,
        );
        self.context.enable_vertex_attrib_array(0);

        self.context.vertex_attrib_pointer_with_i32(
            tile_tex_attrib as u32,
            2,
            WebGlRenderingContext::FLOAT,
            false,
            8 * 4,
            2 * 4,
        );
        self.context.enable_vertex_attrib_array(1);

        self.context.vertex_attrib_pointer_with_i32(
            color_attrib as u32,
            4,
            WebGlRenderingContext::FLOAT,
            false,
            8 * 4,
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
        let lighting_uniform_location = self
            .context
            .get_uniform_location(&self.program, "lighting")
            .ok_or_else(|| err_msg("cannot find 'lighting' uniform"))?;
        self.context
            .uniform1iv_with_i32_array(Some(&lighting_uniform_location), &mut [0]);

        self.context
            .active_texture(WebGlRenderingContext::TEXTURE0 + 1);
        self.context
            .bind_texture(WebGlRenderingContext::TEXTURE_2D, Some(&self.tile_texture));
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
        let tiles_uniform_location = self
            .context
            .get_uniform_location(&self.program, "tiles")
            .ok_or_else(|| err_msg("cannot find 'tiles' uniform"))?;
        self.context
            .uniform1iv_with_i32_array(Some(&tiles_uniform_location), &mut [1]);

        self.context
            .draw_arrays(WebGlRenderingContext::TRIANGLES, 0, block_buffer.1 as i32);

        Ok(())
    }

    fn create_block_buffer(&mut self) -> Result<(WebGlBuffer, u32), Error> {
        self.work_array.clear();

        fn push_vertex(
            array: &mut F32Array,
            x: f32,
            y: f32,
            u: f32,
            v: f32,
            c: (f32, f32, f32, f32),
        ) {
            array.push(x);
            array.push(y);
            array.push(u);
            array.push(v);
            array.push(c.0);
            array.push(c.1);
            array.push(c.2);
            array.push(c.3);
        };

        fn push_block(
            array: &mut F32Array,
            screen: (f32, f32, f32, f32),
            tex: (f32, f32, f32, f32),
            color: (f32, f32, f32, f32),
        ) {
            push_vertex(array, screen.0, screen.1, tex.0, tex.1, color);
            push_vertex(array, screen.2, screen.1, tex.2, tex.1, color);
            push_vertex(array, screen.2, screen.3, tex.2, tex.3, color);

            push_vertex(array, screen.0, screen.1, tex.0, tex.1, color);
            push_vertex(array, screen.2, screen.3, tex.2, tex.3, color);
            push_vertex(array, screen.0, screen.3, tex.0, tex.3, color);
        };

        let w = self.block_size.0;
        let h = self.block_size.1;
        for y in 0..self.block_count.1 {
            for x in 0..self.block_count.0 {
                let xc = x as f32 * self.block_size.0;
                let yc = y as f32 * self.block_size.1;

                match self.block_state[block_index(self.block_count.0, x, y)] {
                    BlockState::Foreground => {
                        let mut xxhash = XxHash::with_seed(17);
                        xxhash.write_u32(x);
                        xxhash.write_u32(y);
                        let block_tex = BLOCK_TEX_COORDS[(xxhash.finish() % 4) as usize];

                        push_block(
                            &mut self.work_array,
                            (xc, yc, xc + w, yc + h),
                            block_tex,
                            (1.0, 1.0, 1.0, 1.0),
                        );

                        if y == self.block_count.1 - 1
                            || self.block_state[block_index(self.block_count.0, x, y + 1)]
                                == BlockState::Empty
                        {
                            let mut xxhash = XxHash::with_seed(23);
                            xxhash.write_u32(x);
                            xxhash.write_u32(y);
                            let grass_tex = GRASS_TEX_COORDS[(xxhash.finish() % 4) as usize];
                            push_block(
                                &mut self.work_array,
                                (xc, yc, xc + w, yc + h),
                                grass_tex,
                                (1.0, 1.0, 1.0, 1.0),
                            );
                        }
                    }
                    BlockState::Background => {
                        let mut xxhash = XxHash::with_seed(11);
                        xxhash.write_u32(x);
                        xxhash.write_u32(y);
                        let block_tex = BLOCK_TEX_COORDS[(xxhash.finish() % 4) as usize];

                        push_block(
                            &mut self.work_array,
                            (xc, yc, xc + w, yc + h),
                            block_tex,
                            (0.5, 0.5, 0.5, 1.0),
                        );
                    }
                    BlockState::Light => {
                        let mut xxhash = XxHash::with_seed(29);
                        xxhash.write_u32(x);
                        xxhash.write_u32(y);
                        let block_tex = LIGHT_TEX_COORDS[(xxhash.finish() % 4) as usize];

                        push_block(
                            &mut self.work_array,
                            (xc, yc, xc + w, yc + h),
                            block_tex,
                            (1.0, 1.0, 1.0, 1.0),
                        );
                    }
                    BlockState::Empty => {}
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

        Ok((buffer, self.work_array.len() / 8))
    }
}

fn block_index(width: u32, x: u32, y: u32) -> usize {
    y as usize * width as usize + x as usize
}

const VERTEX_SHADER: &str = r#"
    precision mediump float;

    attribute vec2 a_position;
    attribute vec2 a_tile_tex;
    attribute vec4 a_color;

    varying vec4 v_color;
    varying vec2 v_tile_tex;
    varying vec2 v_light_tex;

    void main() {
        v_light_tex = a_position;
        v_tile_tex = a_tile_tex;
        v_color = a_color;
        gl_Position = vec4(a_position * 2.0 - 1.0, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    uniform sampler2D lighting;
    uniform sampler2D tiles;

    varying vec4 v_color;
    varying vec2 v_tile_tex;
    varying vec2 v_light_tex;

    void main() {
        gl_FragColor = v_color * texture2D(lighting, v_light_tex) * texture2D(tiles, v_tile_tex);
    }
"#;
