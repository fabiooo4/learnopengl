mod gl_utils;

use gl::types::{GLint, GLsizei};
use glfw::{self, Action, Context, Key, PWindow};
use std::{ffi::c_void, ptr::null};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TITLE: &str = "HELLO TRIANGLE!";
static mut PREVIOUS_KEY_STATE: Action = Action::Release;

fn main() {
    let (mut glfw, mut window) = gl_utils::init_window(
        WIDTH,
        HEIGHT,
        TITLE,
        gl_utils::WindowMode::Windowed,
        None
    );

    println!("Keybinds:");
    println!("  ESCAPE - Close the window");
    println!("  P      - Toggle between fill and wireframe mode");

    render_loop(&mut glfw, &mut window);
}

fn render_loop(glfw: &mut glfw::Glfw, window: &mut PWindow) {
    const VERTEX_SHADER_SRC: &str = include_str!("shaders/vertex.glsl");
    const FRAGMENT_SHADER_SRC: &str = include_str!("shaders/fragment.glsl");

    // A shader program is the result of linking multiple compiled shaders
    let shader_program: u32 = match gl_utils::create_program(&[
        (VERTEX_SHADER_SRC, gl::VERTEX_SHADER),
        (FRAGMENT_SHADER_SRC, gl::FRAGMENT_SHADER),
    ]) {
        Ok(id) => id,
        Err(log) => {
            eprintln!("{log}");
            return;
        }
    };

    #[rustfmt::skip]
    let vertices: &[f32] = &[
         0.5,  0.5, 0., // top right
         0.5, -0.5, 0., // bottom right
        -0.5, -0.5, 0., // bottom left
        -0.5,  0.5, 0., // top left
    ];

    let indices: &[u32] = &[
        0, 1, 3, // First triangle
        1, 2, 3, // Second triangle
    ];

    // Create and bind a vertex buffer object (vertex attribute storage),
    // an element buffer object (vertex indices order)
    // and a vertex array object (attribute layout)
    let mut vao = 0;
    let mut vbo = 0;
    let mut ebo = 0;
    unsafe {
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // VAO must be bound first
        gl::BindVertexArray(vao);

        // Copy vertex data into the vbo
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            size_of_val(vertices) as isize,
            vertices.as_ptr() as *const c_void,
            gl::STATIC_DRAW,
        );

        // Copy the vertex indices into the ebo
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            size_of_val(indices) as isize,
            indices.as_ptr() as *const c_void,
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
        // EBO should not be unbound since it is stored in the VAO
    }

    while !window.should_close() {
        process_input(window);

        // Rendering ----------------------------
        unsafe {
            // Clear the color buffer with a specified color
            gl::ClearColor(0.2, 0.3, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Set shader program to use
            gl::UseProgram(shader_program);

            // Allows to change data layout for different objects
            gl::BindVertexArray(vao);

            // Draw triangles
            // gl::DrawArrays(gl::TRIANGLES, 0, 3);

            // Draw elements
            gl::DrawElements(
                gl::TRIANGLES,
                vertices.len() as GLsizei,
                gl::UNSIGNED_INT,
                null(),
            );
        }
        // Rendering ----------------------------

        window.swap_buffers();
        glfw.poll_events();
    }

    // Deallocate resources
    unsafe {
        gl::DeleteVertexArrays(1, &vao);
        gl::DeleteBuffers(1, &vbo);
        gl::DeleteProgram(shader_program);
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
