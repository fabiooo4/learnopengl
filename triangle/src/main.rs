use glfw::{self, Action, Context, Key, PWindow, Window};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TITLE: &str = "HELLO TRIANGLE!";

fn main() {
    init_window();
}

fn init_window() {
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
        .create_window(WIDTH, HEIGHT, TITLE, glfw::WindowMode::Windowed)
        .expect("Failed to create window");

    // Set the window the current OpenGL target
    window.make_current();
    // Resize callback
    window.set_size_callback(framebuffer_size_callback);

    // Load OpenGL symbols
    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map(|a| a as *const _)
            .expect("Failed to load OpenGL API")
    });

    // Set viewport settings
    unsafe {
        gl::Viewport(0, 0, WIDTH as i32, HEIGHT as i32);
    }

    render_loop(&mut glfw, &mut window);
}

/// Handles a window resize
fn framebuffer_size_callback(_: &mut Window, new_width: i32, new_height: i32) {
    unsafe {
        gl::Viewport(0, 0, new_width, new_height);
    }
}

fn render_loop(glfw: &mut glfw::Glfw, window: &mut PWindow) {
    while !window.should_close() {
        process_input(window);

        // Rendering ----------------------------
        unsafe {
            // Clear the color buffer with a specified color
            gl::ClearColor(0.2, 0.3, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        // Rendering ----------------------------

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(window: &mut PWindow) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    } 
}
