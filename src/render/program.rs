use crate::render::error::*;
use crate::render::types::*;

use glow::*;

/// Simple shader program.
pub struct Program {
    /// Shader Program
    pub pg: ProgramId,
}

impl Program {
    /// Create simple shader program, out of vertex and fragment source, with
    /// their corresponding uniforms name.
    pub fn new(gl: &Context, v_source: &str, f_source: &str) -> Result<Self, String> {
        let pg = unsafe { gl.create_program() }?;
        let shader_sources = [
            (glow::VERTEX_SHADER, v_source),
            (glow::FRAGMENT_SHADER, f_source),
        ];

        let shaders: Vec<_> = shader_sources
            .iter()
            .map(|(shader_type, shader_source)| unsafe {
                let shader = gl
                    .create_shader(*shader_type)
                    .expect("Cannot create shader");
                gl.shader_source(shader, shader_source);
                gl.compile_shader(shader);
                if !gl.get_shader_compile_status(shader) {
                    crate::log("Shader failed to compile.");
                    panic!(gl.get_shader_info_log(shader));
                }
                gl.attach_shader(pg, shader);
                shader
            })
            .collect();

        unsafe {
            gl.link_program(pg);
            if !gl.get_program_link_status(pg) {
                return Err(gl.get_program_info_log(pg));
            }

            for shader in shaders {
                gl.detach_shader(pg, shader);
                gl.delete_shader(shader);
            }

            gl.use_program(Some(pg));
            gl.clear_color(0.1, 0.2, 0.3, 1.0);
        }

        Ok(Program { pg })
    }

    pub unsafe fn use_program(&self, gl: &Context) {
        gl.use_program(Some(self.pg));
    }

    /// TODO
    pub unsafe fn delete(&self, gl: &Context) {
        unimplemented!();
    }
}
