pub mod shader;

use core::str;
use glfw::Context;

#[allow(dead_code)]
pub enum WindowMode {
    Windowed,
    Fullscreen,
}

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
    window_mode: WindowMode,
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
    let (mut window, _) = match window_mode {
        WindowMode::Windowed => {
            if width == 0 || height == 0 {
                panic!("Window dimensions must be greater than zero in windowed mode");
            }

            glfw
                .create_window(width, height, title, glfw::WindowMode::Windowed)
                .expect("Failed to create window")
        },

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

    (glfw, window)
}

/// Default framebuffer size callback that updates the OpenGL viewport when the window is resized.
fn default_framebuffer_size_callback(_: &mut glfw::Window, new_width: i32, new_height: i32) {
    unsafe {
        gl::Viewport(0, 0, new_width, new_height);
    }
}
