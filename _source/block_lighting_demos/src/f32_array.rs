use std::u32;

use js_sys::Float32Array;

pub struct F32Array {
    array: Float32Array,
    len: u32,
}

impl F32Array {
    pub fn new() -> F32Array {
        F32Array {
            array: Float32Array::new_with_length(0),
            len: 0,
        }
    }

    pub fn clear(&mut self) {
        self.resize(0, 0.0);
    }

    pub fn resize(&mut self, new_len: u32, fill: f32) {
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

    pub fn len(&self) -> u32 {
        self.len
    }

    pub fn set(&mut self, i: u32, v: f32) {
        self.array.fill(v, i, i + 1);
    }

    pub fn push(&mut self, v: f32) {
        let i = self.len();
        self.resize(i + 1, 0.0);
        self.set(i, v);
    }

    pub fn visit<R, F: FnOnce(&Float32Array) -> R>(&self, f: F) -> R {
        f(&self.array.subarray(0, self.len))
    }
}
