pub mod camera;
pub mod model;
pub mod shader;

use core::str;
use gl::types::{GLchar, GLenum, GLint, GLsizei, GLuint};
use glfw::Context;
use std::{os::raw::c_void, ptr::null};

#[allow(dead_code)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

/// Initializes glfw, creates a window and loads OpenGL function pointers. The default window size
/// callback, which updates the OpenGL viewport on window resize, is used if no custom callback is
/// provided.
/// Returns the glfw instance, the created window and the event reciever
///
/// # Panics
/// Panics if glfw fails to initialize or if the window creation fails.
pub fn init_window(
    width: u32,
    height: u32,
    title: &str,
    window_mode: WindowMode,
    window_size_callback: Option<fn(&mut glfw::Window, i32, i32)>,
) -> (
    glfw::Glfw,
    glfw::PWindow,
    glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    // Initialize glfw with OpenGL settings
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize glfw");
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Activate debug output (affects performance)
    // Comment this line in release build
    glfw.window_hint(glfw::WindowHint::OpenGlDebugContext(true));

    // Create a window
    let (mut window, event) = match window_mode {
        WindowMode::Windowed => {
            if width == 0 || height == 0 {
                panic!("Window dimensions must be greater than zero in windowed mode");
            }

            glfw.create_window(width, height, title, glfw::WindowMode::Windowed)
                .expect("Failed to create window")
        }

        WindowMode::Fullscreen => glfw.with_primary_monitor(|glfw, m| {
            glfw.create_window(
                800,
                600,
                title,
                m.map_or(glfw::WindowMode::Windowed, |m| {
                    glfw::WindowMode::FullScreen(m)
                }),
            )
            .expect("Failed to create window")
        }),
    };

    window.set_key_polling(true);

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
    let (screen_width, screen_height) = window.get_framebuffer_size();
    unsafe {
        // Set the viewport to cover the whole window
        gl::Viewport(0, 0, screen_width, screen_height);
    }

    // Setup debug callback
    let mut opengl_flags: GLint = 0;
    unsafe {
        gl::GetIntegerv(gl::CONTEXT_FLAGS, &mut opengl_flags);
        if (opengl_flags & gl::CONTEXT_FLAG_DEBUG_BIT as i32) == gl::CONTEXT_FLAG_DEBUG_BIT as i32 {
            gl::Enable(gl::DEBUG_OUTPUT);

            // Call the callback the moment an error occured
            gl::Enable(gl::DEBUG_OUTPUT_SYNCHRONOUS);
            gl::DebugMessageCallback(Some(gl_debug_output), null());

            // Filter errors (no filter applied)
            gl::DebugMessageControl(
                gl::DONT_CARE,
                gl::DONT_CARE,
                gl::DONT_CARE,
                0,
                null(),
                gl::TRUE,
            );
            // Activate only high severity errors
            // gl::DebugMessageControl(
            //     gl::DEBUG_SOURCE_API,
            //     gl::DEBUG_TYPE_ERROR,
            //     gl::DEBUG_SEVERITY_HIGH,
            //     0,
            //     null(),
            //     gl::TRUE,
            // );
        }
    }

    (glfw, window, event)
}

/// Default framebuffer size callback that updates the OpenGL viewport when the window is resized.
fn default_framebuffer_size_callback(_: &mut glfw::Window, new_width: i32, new_height: i32) {
    unsafe {
        gl::Viewport(0, 0, new_width, new_height);
    }
}

/// Debug callback
extern "system" fn gl_debug_output(
    source: GLenum,
    gltype: GLenum,
    id: GLuint,
    severity: GLenum,
    _length: GLsizei,
    message: *const GLchar,
    _user_param: *mut c_void,
) {
    // ignore non-significant error/warning codes
    if id == 131169 || id == 131185 || id == 131218 || id == 131204 {
        return;
    };

    eprintln!("---------------");
    eprintln!("Debug message ({id}): {:?}", message);

    match source {
        gl::DEBUG_SOURCE_API => eprintln!("Source: API"),
        gl::DEBUG_SOURCE_WINDOW_SYSTEM => eprintln!("Source: Window System"),
        gl::DEBUG_SOURCE_SHADER_COMPILER => eprintln!("Source: Shader Compiler"),
        gl::DEBUG_SOURCE_THIRD_PARTY => eprintln!("Source: Third Party"),
        gl::DEBUG_SOURCE_APPLICATION => eprintln!("Source: Application"),
        gl::DEBUG_SOURCE_OTHER => eprintln!("Source: Other"),
        _ => eprintln!(),
    }

    match gltype {
        gl::DEBUG_TYPE_ERROR => eprintln!("Type: Error"),
        gl::DEBUG_TYPE_DEPRECATED_BEHAVIOR => eprintln!("Type: Deprecated Behaviour"),
        gl::DEBUG_TYPE_UNDEFINED_BEHAVIOR => eprintln!("Type: Undefined Behaviour"),
        gl::DEBUG_TYPE_PORTABILITY => eprintln!("Type: Portability"),
        gl::DEBUG_TYPE_PERFORMANCE => eprintln!("Type: Performance"),
        gl::DEBUG_TYPE_MARKER => eprintln!("Type: Marker"),
        gl::DEBUG_TYPE_PUSH_GROUP => eprintln!("Type: Push Group"),
        gl::DEBUG_TYPE_POP_GROUP => eprintln!("Type: Pop Group"),
        gl::DEBUG_TYPE_OTHER => eprintln!("Type: Other"),
        _ => eprintln!(),
    }

    match severity {
        gl::DEBUG_SEVERITY_HIGH => eprintln!("Severity: high"),
        gl::DEBUG_SEVERITY_MEDIUM => eprintln!("Severity: medium"),
        gl::DEBUG_SEVERITY_LOW => eprintln!("Severity: low"),
        gl::DEBUG_SEVERITY_NOTIFICATION => eprintln!("Severity: notification"),
        _ => eprintln!(),
    }
}
