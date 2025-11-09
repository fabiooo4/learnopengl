use core::str;
use gl::types::GLint;
use glfw::Context;
use std::ptr::null_mut;

/// Initializes glfw, creates a window and loads OpenGL function pointers. The default window size
/// callback, which updates the OpenGL viewport on window resize, is used if no custom callback is
/// provided.
/// Returns the glfw instance and the created window.
///
/// # Panics
/// Panics if glfw fails to initialize or if the window creation fails.
pub fn init_window(
    width: u32,
    height: u32,
    title: &str,
    window_mode: glfw::WindowMode,
    window_size_callback: Option<fn(&mut glfw::Window, i32, i32)>,
) -> (glfw::Glfw, glfw::PWindow) {
    // Initialize glfw with OpenGL settings
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize glfw");
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create a window
    let (mut window, _) = glfw
        .create_window(width, height, title, window_mode)
        .expect("Failed to create window");

    // Set the window the current OpenGL target
    window.make_current();

    // Resize callback
    match window_size_callback {
        None => window.set_size_callback(default_framebuffer_size_callback),
        Some(callback) => window.set_size_callback(callback),
    }

    // Load OpenGL symbols
    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map(|a| a as *const _)
            .expect("Failed to load OpenGL API")
    });

    // Set viewport settings
    unsafe {
        gl::Viewport(0, 0, width as i32, height as i32);
    }

    (glfw, window)
}

/// Default framebuffer size callback that updates the OpenGL viewport when the window is resized.
fn default_framebuffer_size_callback(_: &mut glfw::Window, new_width: i32, new_height: i32) {
    unsafe {
        gl::Viewport(0, 0, new_width, new_height);
    }
}


/// Creates a shader object, applies the source to the object and compiles the shader checking if
/// the compilation was succesful.
///
/// Returns the shader object id.
///
/// # Errors
/// If the shader fails to compile, an error string is returned.
pub fn create_shader(shader_src: &str, shader_type: gl::types::GLenum) -> Result<u32, String> {
    let vertex_shader = unsafe { gl::CreateShader(shader_type) };

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
                str::from_utf8(&info_log.map(|c| c as u8))
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
pub fn link_program(shaders: &[u32]) -> Result<u32, String> {
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
                str::from_utf8(&info_log.map(|c| c as u8))
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
pub fn create_program(shaders_src: &[(&str, gl::types::GLenum)]) -> Result<u32, String> {
    let mut compiled = vec![];
    for &(shader_src, shader_type) in shaders_src {
        compiled.push(create_shader(shader_src, shader_type)?);
    }

    link_program(&compiled)
}
