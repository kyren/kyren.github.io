use std::u32;

use failure::{err_msg, Error};
use js_sys::Float32Array;
use web_sys::{WebGlBuffer, WebGlProgram, WebGlRenderingContext};

use glutil::{compile_shader, link_program};

pub struct BlockLighting {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    work_array: F32Array,

    block_count: (u32, u32),
    block_size: (f32, f32),
    block_state: Vec<bool>,
    block_buffer: Option<(WebGlBuffer, u32)>,
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

        Ok(BlockLighting {
            context,
            program,
            work_array: F32Array::new(),
            block_buffer: None,
            block_count: (blocks_horizontal, blocks_vertical),
            block_size: (1.0 / blocks_horizontal as f32, 1.0 / blocks_vertical as f32),
            block_state: vec![false; blocks_horizontal as usize * blocks_vertical as usize],
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

struct F32Array {
    array: Float32Array,
    len: u32,
}

impl F32Array {
    fn new() -> F32Array {
        F32Array {
            array: Float32Array::new_with_length(0),
            len: 0,
        }
    }

    fn clear(&mut self) {
        self.resize(0, 0.0);
    }

    fn resize(&mut self, new_len: u32, fill: f32) {
        if new_len > self.array.length() {
            let new_capacity =
                new_len.max((self.array.length()).checked_mul(2).unwrap_or(u32::MAX));
            let new_array = Float32Array::new_with_length(new_capacity);

            self.array.for_each(&mut |v, i, _| {
                new_array.fill(v, i, i + 1);
            });
            new_array.fill(fill, self.len, new_len);

            self.array = new_array;
            self.len = new_len;
        } else {
            self.array.fill(fill, self.len, new_len);
            self.len = new_len;
        }
    }

    fn len(&self) -> u32 {
        self.len
    }

    fn set(&mut self, i: u32, v: f32) {
        self.array.fill(v, i, i + 1);
    }

    fn push(&mut self, v: f32) {
        let i = self.len();
        self.resize(i + 1, 0.0);
        self.set(i, v);
    }

    fn visit<R, F: FnOnce(&Float32Array) -> R>(&self, f: F) -> R {
        f(&self.array.subarray(0, self.len))
    }
}

const VERTEX_SHADER: &str = r#"
    precision mediump float;

    attribute vec2 a_position;
    attribute vec3 a_color;

    varying vec3 v_color;

    void main() {
        v_color = a_color;
        gl_Position = vec4(a_position * 2.0 - 1.0, 0.0, 1.0);
    }
"#;

const FRAGMENT_SHADER: &str = r#"
    precision mediump float;

    varying vec3 v_color;

    void main() {
        gl_FragColor = vec4(v_color, 1.0);
    }
"#;
