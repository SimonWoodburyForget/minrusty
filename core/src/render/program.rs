use super::*;

use glow::*;

/// Simple shader program.
#[derive(Clone, Debug, Default)]
pub struct Program {
    /// Shader Program
    program_id: Option<ProgramId>,
}

impl Program {
    /// Create simple shader program, out of vertex and fragment source, with
    /// their corresponding uniforms name.
    pub fn new(
        gl: &Context,
        vertex_source: &str,
        fragment_source: &str,
        attribute_locations: &[(u32, &str)],
    ) -> Result<Self, RenderError> {
        let program_id = unsafe { gl.create_program() }?;

        for (location, name) in attribute_locations {
            unsafe { gl.bind_attrib_location(program_id, *location, name) };
        }

        let mut shader_data = [
            (glow::VERTEX_SHADER, vertex_source, None),
            (glow::FRAGMENT_SHADER, fragment_source, None),
        ];

        for (shader_type, shader_source, ref mut shader_id) in shader_data.iter_mut() {
            unsafe {
                let shader = gl.create_shader(*shader_type)?;

                // to later detach the shader
                *shader_id = Some(shader);

                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    crate::log("Shader failed to compile.");
                    return Err(RenderError::from(gl.get_shader_info_log(shader)));
                } else {
                    gl.attach_shader(program_id, shader);
                    gl.delete_shader(shader);
                }
            }
        }

        unsafe {
            gl.link_program(program_id);
            if !gl.get_program_link_status(program_id) {
                return Err(gl.get_program_info_log(program_id).into());
            }

            gl.use_program(Some(program_id));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);

            for (_, _, shader_id) in shader_data.iter() {
                let shader = shader_id.expect("shader_id is None, when it should of compiled.");

                // FIXME: why does this break on web targets?
                #[cfg(feature = "nat")]
                gl.detach_shader(program_id, shader);
            }
        }

        Ok(Program {
            program_id: Some(program_id),
        })
    }

    pub unsafe fn use_program(&self, gl: &Context) {
        gl.use_program(self.program_id);
    }

    /// TODO
    pub unsafe fn _delete(&self, _gl: &Context) {
        unimplemented!();
    }

    pub unsafe fn set_uniform<T: Uniform>(&self, gl: &Context, name: &str, value: T) {
        let location = gl.get_uniform_location(self.program_id.unwrap(), name);
        value.set_as_uniform(gl, location);
    }
}
