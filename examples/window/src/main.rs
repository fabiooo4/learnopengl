use glfw::{self, Context};

const WIDTH: u32 = 480;
const HEIGHT: u32 = 320;
const TITLE: &str = "Hello from GLFW";

fn main() {
    create_glfw_window(WIDTH, HEIGHT, TITLE);
}

fn gl_get_string<'a>(name: gl::types::GLenum) -> &'a str {
    let v = unsafe { gl::GetString(name) };
    let v: &std::ffi::CStr = unsafe { std::ffi::CStr::from_ptr(v as *const i8) };
    v.to_str().unwrap()
}

fn create_glfw_window(width: u32, height: u32, title: &str) {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("GLFW: Failed on init.");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw.create_window(width, height, title, glfw::WindowMode::Windowed).expect("GLFW: Failed on window creation.");

    // get the actual screen resulution size
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|s| {
        window.get_proc_address(s)
            .map(|f| f as *const _)
            .unwrap_or(std::ptr::null())
    });

    println!("GLFW: OpenGL Context Created.");
   
    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl::ClearColor(0.2, 0.3, 0.4, 1.0);
    }

    println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    println!("GLSL version: {}", gl_get_string(gl::SHADING_LANGUAGE_VERSION));
    
    while !window.should_close() {
        // handle events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event);
        }

        // Render
        unsafe {
            let red = f32::sin(glfw.get_time() as f32) / 2.0 + 0.5;
            gl::ClearColor(red, 0.3, 0.4, 1.0);

            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        // Draw on screen
        window.swap_buffers();
    }
}

fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::WindowEvent as Event;
    use glfw::Key;
    use glfw::Action;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        },
        // handle other events here
        _ => {},
    }
}
