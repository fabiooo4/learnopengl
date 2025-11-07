use glfw::{self, Context};
use std::ptr::null;

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
    // Initialization ---------------------------------------------------------
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("GLFW: Failed on init.");

    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(width, height, title, glfw::WindowMode::Windowed)
        .expect("GLFW: Failed on window creation.");

    // get the actual screen resulution size
    let (screen_width, screen_height) = window.get_framebuffer_size();

    window.make_current();
    window.set_key_polling(true);
    gl::load_with(|s| {
        window
            .get_proc_address(s)
            .map(|f| f as *const _)
            .unwrap_or(std::ptr::null())
    });

    println!("GLFW: OpenGL Context Created.");

    unsafe {
        gl::Viewport(0, 0, screen_width, screen_height);
        gl::ClearColor(0.2, 0.3, 0.4, 1.0);
    }

    println!("OpenGL version: {}", gl_get_string(gl::VERSION));
    println!(
        "GLSL version: {}",
        gl_get_string(gl::SHADING_LANGUAGE_VERSION)
    );
    // Initialization ---------------------------------------------------------

    // Vertex input -----------------------------------------------------------
    // Already in normalized device coords
    let triangle_vertices: &[f32] = &[-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];

    // Create and bind a Vertex Array Object (VAO)
    let mut vertex_array = 0;
    // Initialize and bind the vertex buffer with unique id 1
    let mut vertex_buffer = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vertex_array);
        gl::GenBuffers(1, &mut vertex_buffer);

        // Binds the vertex array object
        gl::BindVertexArray(vertex_array);
        // Binds the bertex buffer to the GL_ARRAY_BUFFER target
        gl::BindBuffer(gl::ARRAY_BUFFER, vertex_buffer);

        // Copy vertex data to the new bound buffer
        gl::BufferData(
            gl::ARRAY_BUFFER,
            std::mem::size_of_val(triangle_vertices) as gl::types::GLsizeiptr,
            triangle_vertices.as_ptr() as *const gl::types::GLvoid,
            gl::STATIC_DRAW,
        );

        // Tell OpenGL how to interpret the input data
        gl::VertexAttribPointer(
            0,                           // id (location = 0 in the vertex shader)
            3,         // size (number of components per vertex attribute), in this case 3 (x, y, z)
            gl::FLOAT, // type of data
            gl::FALSE, // normalized?
            3 * size_of::<f32>() as i32, // stride (byte offset between consecutive vertex attries)
            null(),    // offset of the first component
        );

        gl::EnableVertexAttribArray(0);

        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
    // Vertex shader ----------------------------------------------------------
    // Get the shader source code
    const VERTEX_SHADER: &str = include_str!("shaders/vertex_shader.glsl");

    // The shader needs to be compiled in runtime
    // Create a shader of type vertex shader
    let vertex_shader = unsafe { gl::CreateShader(gl::VERTEX_SHADER) };

    // Assign the source code to the shader and compile
    unsafe {
        gl::ShaderSource(
            vertex_shader,
            1,
            &(VERTEX_SHADER.as_ptr() as *const i8),
            null(),
        );
        gl::CompileShader(vertex_shader);
    }
    // Vertex shader ----------------------------------------------------------

    // Fragment shader --------------------------------------------------------
    const FRAGMENT_SHADER: &str = include_str!("shaders/fragment_shader.glsl");
    let fragment_shader = unsafe { gl::CreateShader(gl::FRAGMENT_SHADER) };

    unsafe {
        gl::ShaderSource(
            fragment_shader,
            1,
            &(FRAGMENT_SHADER.as_ptr() as *const i8),
            null(),
        );
        gl::CompileShader(fragment_shader);
    }
    // Fragment shader --------------------------------------------------------

    // Shader program ---------------------------------------------------------
    // Create the shader program (combines vertex and fragment shaders)
    let shader_program = unsafe { gl::CreateProgram() };

    // Attach the previously compiled shaders to the program
    unsafe {
        gl::AttachShader(shader_program, vertex_shader);
        gl::AttachShader(shader_program, fragment_shader);

        // Link them
        gl::LinkProgram(shader_program);
    }
    // Shader program ---------------------------------------------------------

    while !window.should_close() {
        // handle events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            glfw_handle_event(&mut window, event);
        }

        // Render
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Use the combined shader program
            gl::UseProgram(shader_program);
            gl::BindVertexArray(vertex_array);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
            gl::BindVertexArray(0);
        }

        // Draw on screen
        window.swap_buffers();
    }

    // Delete shaders ---------------------------------------------------------
    unsafe {
        gl::DeleteShader(vertex_shader);
        gl::DeleteShader(fragment_shader);
    }
    // Delete shaders ---------------------------------------------------------
}

fn glfw_handle_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    use glfw::Action;
    use glfw::Key;
    use glfw::WindowEvent as Event;

    match event {
        Event::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true);
        }
        // handle other events here
        _ => {}
    }
}
