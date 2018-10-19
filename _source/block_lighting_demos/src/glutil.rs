use failure::{err_msg, Error};
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

pub fn compile_shader(
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

pub fn link_program<'a, T: IntoIterator<Item = &'a WebGlShader>>(
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
