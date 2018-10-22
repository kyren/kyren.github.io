(function() {
    var wasm;
    const __exports = {};


    const __widl_f_log_1__target = console.log;

    const stack = [];

    const slab = [{ obj: undefined }, { obj: null }, { obj: true }, { obj: false }];

    function getObject(idx) {
        if ((idx & 1) === 1) {
            return stack[idx >> 1];
        } else {
            const val = slab[idx >> 1];

            return val.obj;

        }
    }

    __exports.__widl_f_log_1_ = function(arg0) {
        __widl_f_log_1__target(getObject(arg0));
    };

    const __widl_f_get_element_by_id_Document_target = Document.prototype.getElementById || function() {
        throw new Error(`wasm-bindgen: Document.prototype.getElementById does not exist`);
    };

    let cachedDecoder = new TextDecoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function getStringFromWasm(ptr, len) {
        return cachedDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    let slab_next = slab.length;

    function addHeapObject(obj) {
        if (slab_next === slab.length) slab.push(slab.length + 1);
        const idx = slab_next;
        const next = slab[idx];

        slab_next = next;

        slab[idx] = { obj, cnt: 1 };
        return idx << 1;
    }

    function isLikeNone(x) {
        return x === undefined || x === null;
    }

    __exports.__widl_f_get_element_by_id_Document = function(arg0, arg1, arg2) {
        let varg1 = getStringFromWasm(arg1, arg2);

        const val = __widl_f_get_element_by_id_Document_target.call(getObject(arg0), varg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    __exports.__widl_instanceof_HTMLCanvasElement = function(idx) {
        return getObject(idx) instanceof HTMLCanvasElement ? 1 : 0;
    };

    const __widl_f_get_context_HTMLCanvasElement_target = HTMLCanvasElement.prototype.getContext || function() {
        throw new Error(`wasm-bindgen: HTMLCanvasElement.prototype.getContext does not exist`);
    };

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
    }

    __exports.__widl_f_get_context_HTMLCanvasElement = function(arg0, arg1, arg2, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        try {

            const val = __widl_f_get_context_HTMLCanvasElement_target.call(getObject(arg0), varg1);
            return isLikeNone(val) ? 0 : addHeapObject(val);

        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    function GetOwnOrInheritedPropertyDescriptor(obj, id) {
        while (obj) {
            let desc = Object.getOwnPropertyDescriptor(obj, id);
            if (desc) return desc;
            obj = Object.getPrototypeOf(obj);
        }
        throw new Error(`descriptor for id='${id}' not found`);
    }

    const __widl_f_set_width_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLCanvasElement.prototype, 'width').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLCanvasElement.prototype, 'width').set does not exist`);
    };

    __exports.__widl_f_set_width_HTMLCanvasElement = function(arg0, arg1) {
        __widl_f_set_width_HTMLCanvasElement_target.call(getObject(arg0), arg1);
    };

    const __widl_f_set_height_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLCanvasElement.prototype, 'height').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLCanvasElement.prototype, 'height').set does not exist`);
    };

    __exports.__widl_f_set_height_HTMLCanvasElement = function(arg0, arg1) {
        __widl_f_set_height_HTMLCanvasElement_target.call(getObject(arg0), arg1);
    };

    __exports.__widl_instanceof_HTMLElement = function(idx) {
        return getObject(idx) instanceof HTMLElement ? 1 : 0;
    };

    const __widl_f_offset_width_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'offsetWidth').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'offsetWidth').get does not exist`);
    };

    __exports.__widl_f_offset_width_HTMLElement = function(arg0) {
        return __widl_f_offset_width_HTMLElement_target.call(getObject(arg0));
    };

    const __widl_f_offset_height_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'offsetHeight').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'offsetHeight').get does not exist`);
    };

    __exports.__widl_f_offset_height_HTMLElement = function(arg0) {
        return __widl_f_offset_height_HTMLElement_target.call(getObject(arg0));
    };

    const __widl_f_set_onchange_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'onchange').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'onchange').set does not exist`);
    };

    __exports.__widl_f_set_onchange_HTMLElement = function(arg0, arg1) {
        __widl_f_set_onchange_HTMLElement_target.call(getObject(arg0), getObject(arg1));
    };

    const __widl_f_set_onclick_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'onclick').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'onclick').set does not exist`);
    };

    __exports.__widl_f_set_onclick_HTMLElement = function(arg0, arg1) {
        __widl_f_set_onclick_HTMLElement_target.call(getObject(arg0), getObject(arg1));
    };

    const __widl_f_set_onmousemove_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'onmousemove').set || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLElement.prototype, 'onmousemove').set does not exist`);
    };

    __exports.__widl_f_set_onmousemove_HTMLElement = function(arg0, arg1) {
        __widl_f_set_onmousemove_HTMLElement_target.call(getObject(arg0), getObject(arg1));
    };

    __exports.__widl_instanceof_HTMLInputElement = function(idx) {
        return getObject(idx) instanceof HTMLInputElement ? 1 : 0;
    };

    const __widl_f_value_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(HTMLInputElement.prototype, 'value').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(HTMLInputElement.prototype, 'value').get does not exist`);
    };

    let cachedEncoder = new TextEncoder('utf-8');

    function passStringToWasm(arg) {

        const buf = cachedEncoder.encode(arg);
        const ptr = wasm.__wbindgen_malloc(buf.length);
        getUint8Memory().set(buf, ptr);
        return [ptr, buf.length];
    }

    __exports.__widl_f_value_HTMLInputElement = function(ret, arg0) {

        const [retptr, retlen] = passStringToWasm(__widl_f_value_HTMLInputElement_target.call(getObject(arg0)));
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    __exports.__widl_instanceof_WebGLRenderingContext = function(idx) {
        return getObject(idx) instanceof WebGLRenderingContext ? 1 : 0;
    };

    const __widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext_target = WebGLRenderingContext.prototype.bufferData || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.bufferData does not exist`);
    };

    __exports.__widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
        __widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext_target.call(getObject(arg0), arg1, getObject(arg2), arg3);
    };

    const __widl_f_attach_shader_WebGLRenderingContext_target = WebGLRenderingContext.prototype.attachShader || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.attachShader does not exist`);
    };

    __exports.__widl_f_attach_shader_WebGLRenderingContext = function(arg0, arg1, arg2) {
        __widl_f_attach_shader_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), getObject(arg2));
    };

    const __widl_f_bind_buffer_WebGLRenderingContext_target = WebGLRenderingContext.prototype.bindBuffer || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.bindBuffer does not exist`);
    };

    __exports.__widl_f_bind_buffer_WebGLRenderingContext = function(arg0, arg1, arg2) {
        __widl_f_bind_buffer_WebGLRenderingContext_target.call(getObject(arg0), arg1, getObject(arg2));
    };

    const __widl_f_clear_WebGLRenderingContext_target = WebGLRenderingContext.prototype.clear || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.clear does not exist`);
    };

    __exports.__widl_f_clear_WebGLRenderingContext = function(arg0, arg1) {
        __widl_f_clear_WebGLRenderingContext_target.call(getObject(arg0), arg1);
    };

    const __widl_f_clear_color_WebGLRenderingContext_target = WebGLRenderingContext.prototype.clearColor || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.clearColor does not exist`);
    };

    __exports.__widl_f_clear_color_WebGLRenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
        __widl_f_clear_color_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3, arg4);
    };

    const __widl_f_compile_shader_WebGLRenderingContext_target = WebGLRenderingContext.prototype.compileShader || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.compileShader does not exist`);
    };

    __exports.__widl_f_compile_shader_WebGLRenderingContext = function(arg0, arg1) {
        __widl_f_compile_shader_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
    };

    const __widl_f_create_buffer_WebGLRenderingContext_target = WebGLRenderingContext.prototype.createBuffer || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createBuffer does not exist`);
    };

    __exports.__widl_f_create_buffer_WebGLRenderingContext = function(arg0) {

        const val = __widl_f_create_buffer_WebGLRenderingContext_target.call(getObject(arg0));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    const __widl_f_create_program_WebGLRenderingContext_target = WebGLRenderingContext.prototype.createProgram || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createProgram does not exist`);
    };

    __exports.__widl_f_create_program_WebGLRenderingContext = function(arg0) {

        const val = __widl_f_create_program_WebGLRenderingContext_target.call(getObject(arg0));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    const __widl_f_create_shader_WebGLRenderingContext_target = WebGLRenderingContext.prototype.createShader || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.createShader does not exist`);
    };

    __exports.__widl_f_create_shader_WebGLRenderingContext = function(arg0, arg1) {

        const val = __widl_f_create_shader_WebGLRenderingContext_target.call(getObject(arg0), arg1);
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    const __widl_f_draw_arrays_WebGLRenderingContext_target = WebGLRenderingContext.prototype.drawArrays || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.drawArrays does not exist`);
    };

    __exports.__widl_f_draw_arrays_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
        __widl_f_draw_arrays_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3);
    };

    const __widl_f_enable_vertex_attrib_array_WebGLRenderingContext_target = WebGLRenderingContext.prototype.enableVertexAttribArray || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.enableVertexAttribArray does not exist`);
    };

    __exports.__widl_f_enable_vertex_attrib_array_WebGLRenderingContext = function(arg0, arg1) {
        __widl_f_enable_vertex_attrib_array_WebGLRenderingContext_target.call(getObject(arg0), arg1);
    };

    const __widl_f_get_program_info_log_WebGLRenderingContext_target = WebGLRenderingContext.prototype.getProgramInfoLog || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.getProgramInfoLog does not exist`);
    };

    __exports.__widl_f_get_program_info_log_WebGLRenderingContext = function(ret, arg0, arg1) {
        const val = __widl_f_get_program_info_log_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
        const [retptr, retlen] = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_get_program_parameter_WebGLRenderingContext_target = WebGLRenderingContext.prototype.getProgramParameter || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.getProgramParameter does not exist`);
    };

    __exports.__widl_f_get_program_parameter_WebGLRenderingContext = function(arg0, arg1, arg2) {
        return addHeapObject(__widl_f_get_program_parameter_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), arg2));
    };

    const __widl_f_get_shader_info_log_WebGLRenderingContext_target = WebGLRenderingContext.prototype.getShaderInfoLog || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.getShaderInfoLog does not exist`);
    };

    __exports.__widl_f_get_shader_info_log_WebGLRenderingContext = function(ret, arg0, arg1) {
        const val = __widl_f_get_shader_info_log_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
        const [retptr, retlen] = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
        const mem = getUint32Memory();
        mem[ret / 4] = retptr;
        mem[ret / 4 + 1] = retlen;

    };

    const __widl_f_get_shader_parameter_WebGLRenderingContext_target = WebGLRenderingContext.prototype.getShaderParameter || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.getShaderParameter does not exist`);
    };

    __exports.__widl_f_get_shader_parameter_WebGLRenderingContext = function(arg0, arg1, arg2) {
        return addHeapObject(__widl_f_get_shader_parameter_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), arg2));
    };

    const __widl_f_link_program_WebGLRenderingContext_target = WebGLRenderingContext.prototype.linkProgram || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.linkProgram does not exist`);
    };

    __exports.__widl_f_link_program_WebGLRenderingContext = function(arg0, arg1) {
        __widl_f_link_program_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
    };

    const __widl_f_shader_source_WebGLRenderingContext_target = WebGLRenderingContext.prototype.shaderSource || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.shaderSource does not exist`);
    };

    __exports.__widl_f_shader_source_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
        let varg2 = getStringFromWasm(arg2, arg3);
        __widl_f_shader_source_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), varg2);
    };

    const __widl_f_use_program_WebGLRenderingContext_target = WebGLRenderingContext.prototype.useProgram || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.useProgram does not exist`);
    };

    __exports.__widl_f_use_program_WebGLRenderingContext = function(arg0, arg1) {
        __widl_f_use_program_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
    };

    const __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext_target = WebGLRenderingContext.prototype.vertexAttribPointer || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.vertexAttribPointer does not exist`);
    };

    __exports.__widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
        __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3, arg4 !== 0, arg5, arg6);
    };

    const __widl_f_viewport_WebGLRenderingContext_target = WebGLRenderingContext.prototype.viewport || function() {
        throw new Error(`wasm-bindgen: WebGLRenderingContext.prototype.viewport does not exist`);
    };

    __exports.__widl_f_viewport_WebGLRenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
        __widl_f_viewport_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3, arg4);
    };

    __exports.__widl_instanceof_Window = function(idx) {
        return getObject(idx) instanceof Window ? 1 : 0;
    };

    const __widl_f_document_Window_target = function() {
        return this.document;
    };

    __exports.__widl_f_document_Window = function(arg0) {

        const val = __widl_f_document_Window_target.call(getObject(arg0));
        return isLikeNone(val) ? 0 : addHeapObject(val);

    };

    __exports.__wbg_new_b0f6b20c16090c90 = function(arg0) {
        return addHeapObject(new Float32Array(getObject(arg0)));
    };

    const __wbg_fill_9f0d9c86387a5bbe_target = Float32Array.prototype.fill || function() {
        throw new Error(`wasm-bindgen: Float32Array.prototype.fill does not exist`);
    };

    __exports.__wbg_fill_9f0d9c86387a5bbe = function(arg0, arg1, arg2, arg3) {
        return addHeapObject(__wbg_fill_9f0d9c86387a5bbe_target.call(getObject(arg0), arg1, arg2, arg3));
    };

    const __wbg_buffer_1034a5b5085bbbb5_target = GetOwnOrInheritedPropertyDescriptor(Float32Array.prototype, 'buffer').get || function() {
        throw new Error(`wasm-bindgen: GetOwnOrInheritedPropertyDescriptor(Float32Array.prototype, 'buffer').get does not exist`);
    };

    __exports.__wbg_buffer_1034a5b5085bbbb5 = function(arg0) {
        return addHeapObject(__wbg_buffer_1034a5b5085bbbb5_target.call(getObject(arg0)));
    };

    __exports.__wbg_newnoargs_b1f726fad978f5a3 = function(arg0, arg1) {
        let varg0 = getStringFromWasm(arg0, arg1);
        return addHeapObject(new Function(varg0));
    };

    const __wbg_call_fa7f0da29d7b9250_target = Function.prototype.call || function() {
        throw new Error(`wasm-bindgen: Function.prototype.call does not exist`);
    };

    __exports.__wbg_call_fa7f0da29d7b9250 = function(arg0, arg1, exnptr) {
        try {
            return addHeapObject(__wbg_call_fa7f0da29d7b9250_target.call(getObject(arg0), getObject(arg1)));
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };
    /**
    * @returns {void}
    */
    __exports.demo_init = function() {
        return wasm.demo_init();
    };

    __exports.__wbindgen_object_clone_ref = function(idx) {
        // If this object is on the stack promote it to the heap.
        if ((idx & 1) === 1) return addHeapObject(getObject(idx));

        // Otherwise if the object is on the heap just bump the
        // refcount and move on
        const val = slab[idx >> 1];
        val.cnt += 1;
        return idx;
    };

    function dropRef(idx) {

        idx = idx >> 1;
        if (idx < 4) return;
        let obj = slab[idx];

        obj.cnt -= 1;
        if (obj.cnt > 0) return;

        // If we hit 0 then free up our space in the slab
        slab[idx] = slab_next;
        slab_next = idx;
    }

    __exports.__wbindgen_object_drop_ref = function(i) {
        dropRef(i);
    };

    __exports.__wbindgen_string_new = function(p, l) {
        return addHeapObject(getStringFromWasm(p, l));
    };

    __exports.__wbindgen_number_new = function(i) {
        return addHeapObject(i);
    };

    __exports.__wbindgen_boolean_get = function(i) {
        let v = getObject(i);
        if (typeof(v) === 'boolean') {
            return v ? 1 : 0;
        } else {
            return 2;
        }
    };

    __exports.__wbindgen_string_get = function(i, len_ptr) {
        let obj = getObject(i);
        if (typeof(obj) !== 'string') return 0;
        const [ptr, len] = passStringToWasm(obj);
        getUint32Memory()[len_ptr / 4] = len;
        return ptr;
    };

    __exports.__wbindgen_cb_forget = function(i) {
        dropRef(i);
    };

    __exports.__wbindgen_closure_wrapper420 = function(ptr, f, _ignored) {
        let cb = function() {
            let a = this.a;
            this.a = 0;
            try {
                return this.f(a);

            } finally {
                this.a = a;

            }

        };
        cb.f = wasm.__wbg_function_table.get(f);
        cb.a = ptr;
        let real = cb.bind(cb);
        real.original = cb;
        return addHeapObject(real);
    };

    __exports.__wbindgen_closure_wrapper422 = function(ptr, f, _ignored) {
        let cb = function(arg0) {
            let a = this.a;
            this.a = 0;
            try {
                return this.f(a, addHeapObject(arg0));

            } finally {
                this.a = a;

            }

        };
        cb.f = wasm.__wbg_function_table.get(f);
        cb.a = ptr;
        let real = cb.bind(cb);
        real.original = cb;
        return addHeapObject(real);
    };

    __exports.__wbindgen_throw = function(ptr, len) {
        throw new Error(getStringFromWasm(ptr, len));
    };

    function init(wasm_path) {
        const fetchPromise = fetch(wasm_path);
        let resultPromise;
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            resultPromise = WebAssembly.instantiateStreaming(fetchPromise, { './block_lighting_demos': __exports });
        } else {
            resultPromise = fetchPromise
            .then(response => response.arrayBuffer())
            .then(buffer => WebAssembly.instantiate(buffer, { './block_lighting_demos': __exports }));
        }
        return resultPromise.then(({instance}) => {
            wasm = init.wasm = instance.exports;
            return;
        });
    };
    self.wasm_bindgen = Object.assign(init, __exports);
})();
