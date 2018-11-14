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

    const __widl_f_set_property_CSSStyleDeclaration_target = typeof CSSStyleDeclaration === 'undefined' ? null : CSSStyleDeclaration.prototype.setProperty || function() {
        throw new Error(`wasm-bindgen: CSSStyleDeclaration.setProperty does not exist`);
    };

    let cachegetUint32Memory = null;
    function getUint32Memory() {
        if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
            cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
        }
        return cachegetUint32Memory;
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

    let cachedTextDecoder = new TextDecoder('utf-8');

    let cachegetUint8Memory = null;
    function getUint8Memory() {
        if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
            cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
        }
        return cachegetUint8Memory;
    }

    function getStringFromWasm(ptr, len) {
        return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
    }

    __exports.__widl_f_set_property_CSSStyleDeclaration = function(arg0, arg1, arg2, arg3, arg4, exnptr) {
        let varg1 = getStringFromWasm(arg1, arg2);
        let varg3 = getStringFromWasm(arg3, arg4);
        try {
            __widl_f_set_property_CSSStyleDeclaration_target.call(getObject(arg0), varg1, varg3);
        } catch (e) {
            const view = getUint32Memory();
            view[exnptr / 4] = 1;
            view[exnptr / 4 + 1] = addHeapObject(e);

        }
    };

    const __widl_f_get_element_by_id_Document_target = typeof Document === 'undefined' ? null : Document.prototype.getElementById || function() {
        throw new Error(`wasm-bindgen: Document.getElementById does not exist`);
    };

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

    const __widl_f_get_context_HTMLCanvasElement_target = typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype.getContext || function() {
        throw new Error(`wasm-bindgen: HTMLCanvasElement.getContext does not exist`);
    };

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
    return {}
}

const __widl_f_set_width_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'width').set || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.width does not exist`);
};

__exports.__widl_f_set_width_HTMLCanvasElement = function(arg0, arg1) {
    __widl_f_set_width_HTMLCanvasElement_target.call(getObject(arg0), arg1);
};

const __widl_f_set_height_HTMLCanvasElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLCanvasElement === 'undefined' ? null : HTMLCanvasElement.prototype, 'height').set || function() {
    throw new Error(`wasm-bindgen: HTMLCanvasElement.height does not exist`);
};

__exports.__widl_f_set_height_HTMLCanvasElement = function(arg0, arg1) {
    __widl_f_set_height_HTMLCanvasElement_target.call(getObject(arg0), arg1);
};

__exports.__widl_instanceof_HTMLElement = function(idx) {
    return getObject(idx) instanceof HTMLElement ? 1 : 0;
};

const __widl_f_style_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'style').get || function() {
    throw new Error(`wasm-bindgen: HTMLElement.style does not exist`);
};

__exports.__widl_f_style_HTMLElement = function(arg0) {
    return addHeapObject(__widl_f_style_HTMLElement_target.call(getObject(arg0)));
};

const __widl_f_offset_width_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'offsetWidth').get || function() {
    throw new Error(`wasm-bindgen: HTMLElement.offsetWidth does not exist`);
};

__exports.__widl_f_offset_width_HTMLElement = function(arg0) {
    return __widl_f_offset_width_HTMLElement_target.call(getObject(arg0));
};

const __widl_f_offset_height_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'offsetHeight').get || function() {
    throw new Error(`wasm-bindgen: HTMLElement.offsetHeight does not exist`);
};

__exports.__widl_f_offset_height_HTMLElement = function(arg0) {
    return __widl_f_offset_height_HTMLElement_target.call(getObject(arg0));
};

const __widl_f_set_onchange_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'onchange').set || function() {
    throw new Error(`wasm-bindgen: HTMLElement.onchange does not exist`);
};

__exports.__widl_f_set_onchange_HTMLElement = function(arg0, arg1) {
    __widl_f_set_onchange_HTMLElement_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_set_onclick_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'onclick').set || function() {
    throw new Error(`wasm-bindgen: HTMLElement.onclick does not exist`);
};

__exports.__widl_f_set_onclick_HTMLElement = function(arg0, arg1) {
    __widl_f_set_onclick_HTMLElement_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_set_onmousedown_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'onmousedown').set || function() {
    throw new Error(`wasm-bindgen: HTMLElement.onmousedown does not exist`);
};

__exports.__widl_f_set_onmousedown_HTMLElement = function(arg0, arg1) {
    __widl_f_set_onmousedown_HTMLElement_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_set_onmousemove_HTMLElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLElement === 'undefined' ? null : HTMLElement.prototype, 'onmousemove').set || function() {
    throw new Error(`wasm-bindgen: HTMLElement.onmousemove does not exist`);
};

__exports.__widl_f_set_onmousemove_HTMLElement = function(arg0, arg1) {
    __widl_f_set_onmousemove_HTMLElement_target.call(getObject(arg0), getObject(arg1));
};

__exports.__widl_instanceof_HTMLInputElement = function(idx) {
    return getObject(idx) instanceof HTMLInputElement ? 1 : 0;
};

const __widl_f_value_HTMLInputElement_target = GetOwnOrInheritedPropertyDescriptor(typeof HTMLInputElement === 'undefined' ? null : HTMLInputElement.prototype, 'value').get || function() {
    throw new Error(`wasm-bindgen: HTMLInputElement.value does not exist`);
};

let cachedTextEncoder = new TextEncoder('utf-8');

function passStringToWasm(arg) {

    const buf = cachedTextEncoder.encode(arg);
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

const __widl_f_x_MouseEvent_target = GetOwnOrInheritedPropertyDescriptor(typeof MouseEvent === 'undefined' ? null : MouseEvent.prototype, 'x').get || function() {
    throw new Error(`wasm-bindgen: MouseEvent.x does not exist`);
};

__exports.__widl_f_x_MouseEvent = function(arg0) {
    return __widl_f_x_MouseEvent_target.call(getObject(arg0));
};

const __widl_f_y_MouseEvent_target = GetOwnOrInheritedPropertyDescriptor(typeof MouseEvent === 'undefined' ? null : MouseEvent.prototype, 'y').get || function() {
    throw new Error(`wasm-bindgen: MouseEvent.y does not exist`);
};

__exports.__widl_f_y_MouseEvent = function(arg0) {
    return __widl_f_y_MouseEvent_target.call(getObject(arg0));
};

const __widl_f_buttons_MouseEvent_target = GetOwnOrInheritedPropertyDescriptor(typeof MouseEvent === 'undefined' ? null : MouseEvent.prototype, 'buttons').get || function() {
    throw new Error(`wasm-bindgen: MouseEvent.buttons does not exist`);
};

__exports.__widl_f_buttons_MouseEvent = function(arg0) {
    return __widl_f_buttons_MouseEvent_target.call(getObject(arg0));
};

__exports.__widl_instanceof_WebGLRenderingContext = function(idx) {
    return getObject(idx) instanceof WebGLRenderingContext ? 1 : 0;
};

const __widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.bufferData || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.bufferData does not exist`);
};

__exports.__widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
    __widl_f_buffer_data_with_opt_array_buffer_WebGLRenderingContext_target.call(getObject(arg0), arg1, getObject(arg2), arg3);
};

const __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.texImage2D || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.texImage2D does not exist`);
};

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

__exports.__widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10, exnptr) {
    let varg9 = arg9 == 0 ? undefined : getArrayU8FromWasm(arg9, arg10);
    try {
        __widl_f_tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_u8_array_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, varg9);
    } catch (e) {
        const view = getUint32Memory();
        view[exnptr / 4] = 1;
        view[exnptr / 4 + 1] = addHeapObject(e);

    }
};

const __widl_f_uniform1iv_with_i32_array_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.uniform1iv || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.uniform1iv does not exist`);
};

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

function getArrayI32FromWasm(ptr, len) {
    return getInt32Memory().subarray(ptr / 4, ptr / 4 + len);
}

__exports.__widl_f_uniform1iv_with_i32_array_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
    let varg2 = getArrayI32FromWasm(arg2, arg3);
    __widl_f_uniform1iv_with_i32_array_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), varg2);
};

const __widl_f_attach_shader_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.attachShader || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.attachShader does not exist`);
};

__exports.__widl_f_attach_shader_WebGLRenderingContext = function(arg0, arg1, arg2) {
    __widl_f_attach_shader_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), getObject(arg2));
};

const __widl_f_bind_buffer_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.bindBuffer || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.bindBuffer does not exist`);
};

__exports.__widl_f_bind_buffer_WebGLRenderingContext = function(arg0, arg1, arg2) {
    __widl_f_bind_buffer_WebGLRenderingContext_target.call(getObject(arg0), arg1, getObject(arg2));
};

const __widl_f_bind_texture_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.bindTexture || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.bindTexture does not exist`);
};

__exports.__widl_f_bind_texture_WebGLRenderingContext = function(arg0, arg1, arg2) {
    __widl_f_bind_texture_WebGLRenderingContext_target.call(getObject(arg0), arg1, getObject(arg2));
};

const __widl_f_clear_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.clear || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.clear does not exist`);
};

__exports.__widl_f_clear_WebGLRenderingContext = function(arg0, arg1) {
    __widl_f_clear_WebGLRenderingContext_target.call(getObject(arg0), arg1);
};

const __widl_f_clear_color_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.clearColor || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.clearColor does not exist`);
};

__exports.__widl_f_clear_color_WebGLRenderingContext = function(arg0, arg1, arg2, arg3, arg4) {
    __widl_f_clear_color_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3, arg4);
};

const __widl_f_compile_shader_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.compileShader || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.compileShader does not exist`);
};

__exports.__widl_f_compile_shader_WebGLRenderingContext = function(arg0, arg1) {
    __widl_f_compile_shader_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_create_buffer_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.createBuffer || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.createBuffer does not exist`);
};

__exports.__widl_f_create_buffer_WebGLRenderingContext = function(arg0) {

    const val = __widl_f_create_buffer_WebGLRenderingContext_target.call(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_create_program_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.createProgram || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.createProgram does not exist`);
};

__exports.__widl_f_create_program_WebGLRenderingContext = function(arg0) {

    const val = __widl_f_create_program_WebGLRenderingContext_target.call(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_create_shader_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.createShader || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.createShader does not exist`);
};

__exports.__widl_f_create_shader_WebGLRenderingContext = function(arg0, arg1) {

    const val = __widl_f_create_shader_WebGLRenderingContext_target.call(getObject(arg0), arg1);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_create_texture_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.createTexture || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.createTexture does not exist`);
};

__exports.__widl_f_create_texture_WebGLRenderingContext = function(arg0) {

    const val = __widl_f_create_texture_WebGLRenderingContext_target.call(getObject(arg0));
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_draw_arrays_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.drawArrays || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.drawArrays does not exist`);
};

__exports.__widl_f_draw_arrays_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
    __widl_f_draw_arrays_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3);
};

const __widl_f_enable_vertex_attrib_array_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.enableVertexAttribArray || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.enableVertexAttribArray does not exist`);
};

__exports.__widl_f_enable_vertex_attrib_array_WebGLRenderingContext = function(arg0, arg1) {
    __widl_f_enable_vertex_attrib_array_WebGLRenderingContext_target.call(getObject(arg0), arg1);
};

const __widl_f_get_program_info_log_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.getProgramInfoLog || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.getProgramInfoLog does not exist`);
};

__exports.__widl_f_get_program_info_log_WebGLRenderingContext = function(ret, arg0, arg1) {
    const val = __widl_f_get_program_info_log_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
    const [retptr, retlen] = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_get_program_parameter_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.getProgramParameter || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.getProgramParameter does not exist`);
};

__exports.__widl_f_get_program_parameter_WebGLRenderingContext = function(arg0, arg1, arg2) {
    return addHeapObject(__widl_f_get_program_parameter_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), arg2));
};

const __widl_f_get_shader_info_log_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.getShaderInfoLog || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.getShaderInfoLog does not exist`);
};

__exports.__widl_f_get_shader_info_log_WebGLRenderingContext = function(ret, arg0, arg1) {
    const val = __widl_f_get_shader_info_log_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
    const [retptr, retlen] = isLikeNone(val) ? [0, 0] : passStringToWasm(val);
    const mem = getUint32Memory();
    mem[ret / 4] = retptr;
    mem[ret / 4 + 1] = retlen;

};

const __widl_f_get_shader_parameter_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.getShaderParameter || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.getShaderParameter does not exist`);
};

__exports.__widl_f_get_shader_parameter_WebGLRenderingContext = function(arg0, arg1, arg2) {
    return addHeapObject(__widl_f_get_shader_parameter_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), arg2));
};

const __widl_f_get_uniform_location_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.getUniformLocation || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.getUniformLocation does not exist`);
};

__exports.__widl_f_get_uniform_location_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);

    const val = __widl_f_get_uniform_location_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), varg2);
    return isLikeNone(val) ? 0 : addHeapObject(val);

};

const __widl_f_link_program_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.linkProgram || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.linkProgram does not exist`);
};

__exports.__widl_f_link_program_WebGLRenderingContext = function(arg0, arg1) {
    __widl_f_link_program_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_shader_source_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.shaderSource || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.shaderSource does not exist`);
};

__exports.__widl_f_shader_source_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
    let varg2 = getStringFromWasm(arg2, arg3);
    __widl_f_shader_source_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1), varg2);
};

const __widl_f_tex_parameteri_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.texParameteri || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.texParameteri does not exist`);
};

__exports.__widl_f_tex_parameteri_WebGLRenderingContext = function(arg0, arg1, arg2, arg3) {
    __widl_f_tex_parameteri_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3);
};

const __widl_f_use_program_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.useProgram || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.useProgram does not exist`);
};

__exports.__widl_f_use_program_WebGLRenderingContext = function(arg0, arg1) {
    __widl_f_use_program_WebGLRenderingContext_target.call(getObject(arg0), getObject(arg1));
};

const __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.vertexAttribPointer || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.vertexAttribPointer does not exist`);
};

__exports.__widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext = function(arg0, arg1, arg2, arg3, arg4, arg5, arg6) {
    __widl_f_vertex_attrib_pointer_with_i32_WebGLRenderingContext_target.call(getObject(arg0), arg1, arg2, arg3, arg4 !== 0, arg5, arg6);
};

const __widl_f_viewport_WebGLRenderingContext_target = typeof WebGLRenderingContext === 'undefined' ? null : WebGLRenderingContext.prototype.viewport || function() {
    throw new Error(`wasm-bindgen: WebGLRenderingContext.viewport does not exist`);
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

__exports.__wbg_newwithlength_987928aadf394859 = function(arg0) {
    return addHeapObject(new Float32Array(arg0));
};

const __wbg_fill_88e5dbd95aaecbfb_target = typeof Float32Array === 'undefined' ? null : Float32Array.prototype.fill || function() {
    throw new Error(`wasm-bindgen: Float32Array.fill does not exist`);
};

__exports.__wbg_fill_88e5dbd95aaecbfb = function(arg0, arg1, arg2, arg3) {
    return addHeapObject(__wbg_fill_88e5dbd95aaecbfb_target.call(getObject(arg0), arg1, arg2, arg3));
};

const __wbg_buffer_9d7b5894c32615e1_target = GetOwnOrInheritedPropertyDescriptor(typeof Float32Array === 'undefined' ? null : Float32Array.prototype, 'buffer').get || function() {
    throw new Error(`wasm-bindgen: Float32Array.buffer does not exist`);
};

__exports.__wbg_buffer_9d7b5894c32615e1 = function(arg0) {
    return addHeapObject(__wbg_buffer_9d7b5894c32615e1_target.call(getObject(arg0)));
};

const __wbg_subarray_5cec2cd069e3aa89_target = typeof Float32Array === 'undefined' ? null : Float32Array.prototype.subarray || function() {
    throw new Error(`wasm-bindgen: Float32Array.subarray does not exist`);
};

__exports.__wbg_subarray_5cec2cd069e3aa89 = function(arg0, arg1, arg2) {
    return addHeapObject(__wbg_subarray_5cec2cd069e3aa89_target.call(getObject(arg0), arg1, arg2));
};

const __wbg_forEach_2b89c96ddfdd5b4e_target = typeof Float32Array === 'undefined' ? null : Float32Array.prototype.forEach || function() {
    throw new Error(`wasm-bindgen: Float32Array.forEach does not exist`);
};

let cachedGlobalArgumentPtr = null;
function globalArgumentPtr() {
    if (cachedGlobalArgumentPtr === null) {
        cachedGlobalArgumentPtr = wasm.__wbindgen_global_argument_ptr();
    }
    return cachedGlobalArgumentPtr;
}

function getGlobalArgument(arg) {
    const idx = globalArgumentPtr() / 4 + arg;
    return getUint32Memory()[idx];
}

__exports.__wbg_forEach_2b89c96ddfdd5b4e = function(arg0, arg1) {
    let cbarg1 = function(arg0, arg1, arg2) {
        let a = this.a;
        this.a = 0;
        try {
            return this.f(a, this.b, arg0, arg1, addHeapObject(arg2));

        } finally {
            this.a = a;

        }

    };
    cbarg1.f = wasm.__wbg_function_table.get(arg1);
    cbarg1.a = getGlobalArgument(0);
    cbarg1.b = getGlobalArgument(0 + 1);
    try {
        __wbg_forEach_2b89c96ddfdd5b4e_target.call(getObject(arg0), cbarg1.bind(cbarg1));
    } finally {
        cbarg1.a = cbarg1.b = 0;

    }
};

const __wbg_length_973f0dbdc2c7e1c3_target = GetOwnOrInheritedPropertyDescriptor(typeof Float32Array === 'undefined' ? null : Float32Array.prototype, 'length').get || function() {
    throw new Error(`wasm-bindgen: Float32Array.length does not exist`);
};

__exports.__wbg_length_973f0dbdc2c7e1c3 = function(arg0) {
    return __wbg_length_973f0dbdc2c7e1c3_target.call(getObject(arg0));
};

__exports.__wbg_newnoargs_b5dbe629f3c72f37 = function(arg0, arg1) {
    let varg0 = getStringFromWasm(arg0, arg1);
    return addHeapObject(new Function(varg0));
};

const __wbg_call_80c8cb20bdc473db_target = typeof Function === 'undefined' ? null : Function.prototype.call || function() {
    throw new Error(`wasm-bindgen: Function.call does not exist`);
};

__exports.__wbg_call_80c8cb20bdc473db = function(arg0, arg1, exnptr) {
    try {
        return addHeapObject(__wbg_call_80c8cb20bdc473db_target.call(getObject(arg0), getObject(arg1)));
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

__exports.__wbindgen_cb_forget = dropRef;

__exports.__wbindgen_closure_wrapper459 = function(a, b, fi, di, _ignored) {
    const f = wasm.__wbg_function_table.get(fi);
    const d = wasm.__wbg_function_table.get(di);
    const cb = function(arg0) {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b, addHeapObject(arg0));

        } finally {
            this.a = a;
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_closure_wrapper463 = function(a, b, fi, di, _ignored) {
    const f = wasm.__wbg_function_table.get(fi);
    const d = wasm.__wbg_function_table.get(di);
    const cb = function() {
        this.cnt++;
        let a = this.a;
        this.a = 0;
        try {
            return f(a, b);

        } finally {
            this.a = a;
            if (this.cnt-- == 1) d(this.a, b);

        }

    };
    cb.a = a;
    cb.cnt = 1;
    let real = cb.bind(cb);
    real.original = cb;
    return addHeapObject(real);
};

__exports.__wbindgen_throw = function(ptr, len) {
    throw new Error(getStringFromWasm(ptr, len));
};

function init(path_or_module) {
    let instantiation;
    const imports = { './block_lighting_demos': __exports };
    if (path_or_module instanceof WebAssembly.Module) {
        instantiation = WebAssembly.instantiate(path_or_module, imports);
    } else {
        const data = fetch(path_or_module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            instantiation = WebAssembly.instantiateStreaming(data, imports);
        } else {
            instantiation = data
            .then(response => response.arrayBuffer())
            .then(buffer => WebAssembly.instantiate(buffer, imports));
        }
    }
    return instantiation.then(({instance}) => {
        wasm = init.wasm = instance.exports;
        return;
    });
};
self.wasm_bindgen = Object.assign(init, __exports);
})();
