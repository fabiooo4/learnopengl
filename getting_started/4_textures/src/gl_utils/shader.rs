use std::{fs::read_to_string, ptr::null_mut};

use gl::types::GLint;

#[derive(Clone, Copy)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER as isize,
    FragmentShader = gl::FRAGMENT_SHADER as isize,
    ComputeShader = gl::COMPUTE_SHADER as isize,
    GeometryShader = gl::GEOMETRY_SHADER as isize,
}

pub struct Shader {
    pub program_id: u32,
}

impl Shader {
    /// Creates a shader object, applies the source to the object and compiles the shader checking if
    /// the compilation was succesful.
    ///
    /// Returns the shader object id.
    ///
    /// # Errors
    /// If file reading fails or if the shader fails to compile, an error string is returned.
    fn create_shader(shader_path: &str, shader_type: ShaderType) -> Result<u32, String> {
        let shader_src = read_to_string(shader_path).unwrap_or_else(|e| panic!("Error: Failed to read file {}\n{}", shader_path, e));

        let vertex_shader = unsafe { gl::CreateShader(shader_type as u32) };

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        unsafe {
            // Set shader source to shader object
            gl::ShaderSource(
                vertex_shader,
                1,
                &(shader_src.as_ptr() as *const i8),
                &(shader_src.len() as GLint),
            );

            // Compile the shader
            gl::CompileShader(vertex_shader);

            // Check compilation errors
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    info_log.len() as i32,
                    null_mut(),
                    info_log.as_mut_ptr(),
                );

                return Err(format!(
                    "Error: Shader compilation failed\n{}",
                    std::str::from_utf8(&info_log.map(|c| c as u8))
                        .expect("Failed to convert info log to string")
                ));
            }
        };

        Ok(vertex_shader)
    }

    /// Creates a program object, links all the provided shader objects and checks if the linking was
    /// successful.
    ///
    /// Returns the program object id.
    ///
    /// # Errors
    /// If the program fails to link, an error string is returned.
    fn link_program(shaders: &[u32]) -> Result<u32, String> {
        let shader_program: u32 = unsafe { gl::CreateProgram() };

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        unsafe {
            // Attach all shaders to the program
            for &shader in shaders {
                gl::AttachShader(shader_program, shader);
            }

            // Link the attached shaderss
            gl::LinkProgram(shader_program);

            // Check compilation errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                gl::GetProgramInfoLog(
                    shader_program,
                    info_log.len() as i32,
                    null_mut(),
                    info_log.as_mut_ptr(),
                );

                return Err(format!(
                    "Error: Program linking failed\n{}",
                    std::str::from_utf8(&info_log.map(|c| c as u8))
                        .expect("Failed to convert info log to string")
                ));
            }
        };

        // Remove all the shaders objects
        for &shader in shaders {
            unsafe { gl::DeleteShader(shader) };
        }

        Ok(shader_program)
    }

    /// Creates a shader program given a list of shader sources.
    ///
    /// Returns the program object id.
    ///
    /// # Errors
    /// If any of the shaders fail to compile or if the program fails to link, an error string is
    /// returned.
    pub fn new(shaders: &[(&str, ShaderType)]) -> Result<Self, String> {
        let mut compiled = vec![];
        for &(shader_path, shader_type) in shaders {
            compiled.push(Shader::create_shader(shader_path, shader_type)?);
        }

        match Shader::link_program(&compiled) {
            Ok(id) => Ok(Shader { program_id: id }),
            Err(e) => Err(e),
        }
    }

    /// Sets the current shader program for rendering
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    /// Gets the given uniform location
    ///
    /// Returns the uniform location number
    ///
    /// # Errors
    /// If the location was not found, an error string is returned
    fn get_uniform_location(&self, uniform: &str) -> Result<i32, String> {
        let result = unsafe {
            gl::GetUniformLocation(
                self.program_id,
                (uniform
                    .as_bytes()
                    .iter()
                    .map(|&b| b as i8)
                    .collect::<Vec<i8>>())
                .as_ptr(),
            )
        };

        if result == -1 {
            Err(format!("Error: Could not find uniform location for \"{}\"", uniform))
        } else {
            Ok(result)
        }
    }

    /// Sets the value of a float uniform
    pub fn set_uniform_1f(&self, uniform: &str, val: f32) -> Result<(), String> {
        unsafe { gl::Uniform1f(self.get_uniform_location(uniform)?, val) };
        Ok(())
    }

    /// Sets the value of a vec2 float uniform
    pub fn set_uniform_2f(&self, uniform: &str, val1: f32, val2: f32) -> Result<(), String> {
        unsafe { gl::Uniform2f(self.get_uniform_location(uniform)?, val1, val2) };
        Ok(())
    }

    /// Sets the value of a vec3 float uniform
    pub fn set_uniform_3f(
        &self,
        uniform: &str,
        val1: f32,
        val2: f32,
        val3: f32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform3f(self.get_uniform_location(uniform)?, val1, val2, val3) };
        Ok(())
    }

    /// Sets the value of a vec4 float uniform
    pub fn set_uniform_4f(
        &self,
        uniform: &str,
        val1: f32,
        val2: f32,
        val3: f32,
        val4: f32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform4f(self.get_uniform_location(uniform)?, val1, val2, val3, val4) };
        Ok(())
    }

    /// Sets the value of an integer uniform
    pub fn set_uniform_1i(&self, uniform: &str, val: i32) -> Result<(), String> {
        unsafe { gl::Uniform1i(self.get_uniform_location(uniform)?, val) };
        Ok(())
    }

    /// Sets the value of a vec2 integer uniform
    pub fn set_uniform_2i(&self, uniform: &str, val1: i32, val2: i32) -> Result<(), String> {
        unsafe { gl::Uniform2i(self.get_uniform_location(uniform)?, val1, val2) };
        Ok(())
    }

    /// Sets the value of a vec3 integer uniform
    pub fn set_uniform_3i(
        &self,
        uniform: &str,
        val1: i32,
        val2: i32,
        val3: i32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform3i(self.get_uniform_location(uniform)?, val1, val2, val3) };
        Ok(())
    }

    /// Sets the value of a vec4 integer uniform
    pub fn set_uniform_4i(
        &self,
        uniform: &str,
        val1: i32,
        val2: i32,
        val3: i32,
        val4: i32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform4i(self.get_uniform_location(uniform)?, val1, val2, val3, val4) };
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}
