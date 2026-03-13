use hello_triangle::gl_utils;

use gl::types::{GLint, GLsizei};
use glfw::{self, Action, Context, Key, PWindow};
use std::{ffi::c_void, ptr::null};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TITLE: &str = "Hello Triangle - Exercise 1";
static mut PREVIOUS_KEY_STATE: Action = Action::Release;

fn main() {
    let (mut glfw, mut window) =
        gl_utils::init_window(WIDTH, HEIGHT, TITLE, gl_utils::WindowMode::Windowed, None);

    println!("Exercise instructions:");
    println!(
        "Create two shader programs where the second program uses a different fragment shader that outputs the color yellow; draw both triangles again where one outputs the color yellow"
    );

    println!("Keybinds:");
    println!("  ESCAPE - Close the window");
    println!("  P      - Toggle between fill and wireframe mode");

    render_loop(&mut glfw, &mut window);
}

fn render_loop(glfw: &mut glfw::Glfw, window: &mut PWindow) {
    const VERTEX_SHADER_SRC: &str = include_str!("../../shaders/vertex.glsl");

    // A shader program is the result of linking multiple compiled shaders
    const FRAGMENT_SHADER_SRC_T1: &str = include_str!("shaders/fragment_t1.glsl");
    let shader_program_t1: u32 = match gl_utils::create_program(&[
        (VERTEX_SHADER_SRC, gl::VERTEX_SHADER),
        (FRAGMENT_SHADER_SRC_T1, gl::FRAGMENT_SHADER),
    ]) {
        Ok(id) => id,
        Err(log) => {
            eprintln!("{log}");
            return;
        }
    };

    const FRAGMENT_SHADER_SRC_T2: &str = include_str!("shaders/fragment_t2.glsl");
    let shader_program_t2: u32 = match gl_utils::create_program(&[
        (VERTEX_SHADER_SRC, gl::VERTEX_SHADER),
        (FRAGMENT_SHADER_SRC_T2, gl::FRAGMENT_SHADER),
    ]) {
        Ok(id) => id,
        Err(log) => {
            eprintln!("{log}");
            return;
        }
    };

    #[rustfmt::skip]
    let triangle_1: &[f32] = &[
        -0.9, -0.5, 0.0, // Bottom left
        -0.0, -0.5, 0.0, // Bottom right
        -0.45, 0.5, 0.0, // Top
    ];

    #[rustfmt::skip]
    let triangle_2: &[f32] = &[
         0.0, -0.5, 0.0, // Bottom left
         0.9, -0.5, 0.0, // Bottom right
         0.45, 0.5, 0.0, // Top
    ];

    // Create and bind a vertex buffer object (vertex attribute storage),
    // an element buffer object (vertex indices order)
    // and a vertex array object (attribute layout)

    // Triangle 1 setup
    let mut vao_t1 = 0;
    let mut vbo_t1 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao_t1);
        gl::GenBuffers(1, &mut vbo_t1);

        // VAO must be bound first
        gl::BindVertexArray(vao_t1);

        // Copy vertex data into the vbo
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_t1);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(triangle_1) as isize,
            triangle_1.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Specify data layout by saving the config in a vertex array object
        gl::VertexAttribPointer(
            0,                               // Location = 0 in vertex shader
            3,                               // Size of data (3 floats = vec3)
            gl::FLOAT,                       // Data type
            gl::FALSE,                       // Normalization
            3 * size_of::<f32>() as GLsizei, // Space between 2 consecutive attributes
            null(),                          // Offset
        );

        // Enable vertex attributes with location = 0
        gl::EnableVertexAttribArray(0);

        // Unbind vao
        gl::BindVertexArray(0);
        // Unbind the vbo since it was bound to the vertex attribute pointer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    // Triangle 2 setup
    let mut vao_t2 = 0;
    let mut vbo_t2 = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao_t2);
        gl::GenBuffers(1, &mut vbo_t2);

        // VAO must be bound first
        gl::BindVertexArray(vao_t2);

        // Copy vertex data into the vbo
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo_t2);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(triangle_2) as isize,
            triangle_2.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Specify data layout by saving the config in a vertex array object
        gl::VertexAttribPointer(
            0,                               // Location = 0 in vertex shader
            3,                               // Size of data (3 floats = vec3)
            gl::FLOAT,                       // Data type
            gl::FALSE,                       // Normalization
            3 * size_of::<f32>() as GLsizei, // Space between 2 consecutive attributes
            null(),                          // Offset
        );

        // Enable vertex attributes with location = 0
        gl::EnableVertexAttribArray(0);

        // Unbind vao
        gl::BindVertexArray(0);
        // Unbind the vbo since it was bound to the vertex attribute pointer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    }

    while !window.should_close() {
        process_input(window);

        // Rendering ----------------------------
        unsafe {
            // Clear the color buffer with a specified color
            gl::ClearColor(0.2, 0.3, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Draw triangle 1
            gl::UseProgram(shader_program_t1);
            gl::BindVertexArray(vao_t1);
            gl::DrawArrays(gl::TRIANGLES, 0, triangle_1.len() as GLint);

            // Draw triangle 2
            gl::UseProgram(shader_program_t2);
            gl::BindVertexArray(vao_t2);
            gl::DrawArrays(gl::TRIANGLES, 0, triangle_2.len() as GLint);
        }
        // Rendering ----------------------------

        window.swap_buffers();
        glfw.poll_events();
    }

    // Deallocate resources
    unsafe {
        gl::DeleteVertexArrays(1, &vao_t1);
        gl::DeleteBuffers(1, &vbo_t1);
        gl::DeleteProgram(shader_program_t1);
    }
}

fn process_input(window: &mut PWindow) {
    if window.get_key(Key::Escape) == Action::Press {
        window.set_should_close(true);
    }
    match window.get_key(Key::P) {
        Action::Release => unsafe {
            // Execute only after the key is released
            if PREVIOUS_KEY_STATE == Action::Press {
                let mut mode: GLint = 0;
                gl::GetIntegerv(gl::POLYGON_MODE, &mut mode);

                if mode == gl::LINE as GLint {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                } else if mode == gl::FILL as GLint {
                    gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                }
            }

            PREVIOUS_KEY_STATE = Action::Release
        },
        Action::Press => unsafe { PREVIOUS_KEY_STATE = Action::Press },
        Action::Repeat => {}
    }
}
