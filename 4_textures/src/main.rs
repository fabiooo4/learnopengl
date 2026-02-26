use image::{EncodableLayout, ImageReader};
use textures::gl_utils::{
    self,
    shader::{Shader, ShaderType},
};

use gl::types::{GLint, GLsizei};
use glfw::{self, Action, Context, Key, PWindow};
use std::{ffi::c_void, ptr::null};

const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TITLE: &str = "HELLO TEXTURES!";

fn main() {
    let (mut glfw, mut window, events) =
        gl_utils::init_window(WIDTH, HEIGHT, TITLE, gl_utils::WindowMode::Windowed, None);

    println!("Keybinds:");
    println!("  ESCAPE - Close the window");
    println!("  P      - Toggle between fill and wireframe mode");

    render_loop(&mut glfw, &mut window, &events);
}

fn render_loop(
    glfw: &mut glfw::Glfw,
    window: &mut PWindow,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    // A shader program is the result of linking multiple compiled shaders
    let shader_program: Shader = Shader::new(&[
        ("src/shaders/vertex.glsl", ShaderType::VertexShader),
        ("src/shaders/fragment.glsl", ShaderType::FragmentShader),
    ])
    .unwrap_or_else(|log| panic!("{log}"));

    #[rustfmt::skip]
    let vertices: &[f32] = &[
        // Positions       // Colors        // Texture coords
         0.5,  0.5, 0.0,   1.0, 0.0, 0.0,   1.0, 1.0,   // Top right
         0.5, -0.5, 0.0,   0.0, 1.0, 0.0,   1.0, 0.0,   // Bottom right
        -0.5, -0.5, 0.0,   0.0, 0.0, 1.0,   0.0, 0.0,   // Bottom left
        -0.5,  0.5, 0.0,   1.0, 1.0, 0.0,   0.0, 1.0    // Top left 
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

        // Position attributes ----------------------------
        gl::VertexAttribPointer(
            0,                                         // Location = 0 in vertex shader
            3,                                         // Size of data (3 floats = vec3)
            gl::FLOAT,                                 // Data type
            gl::FALSE,                                 // Normalization
            (3 + 3 + 2) * size_of::<f32>() as GLsizei, // Space between 2 consecutive attributes
            null(),                                    // Offset
        );

        gl::EnableVertexAttribArray(0);
        // Position attributes ----------------------------

        // Color attributes -------------------------------
        gl::VertexAttribPointer(
            1,                                         // Location = 1 in vertex shader
            3,                                         // Size of data (3 floats = vec3)
            gl::FLOAT,                                 // Data type
            gl::FALSE,                                 // Normalization
            (3 + 3 + 2) * size_of::<f32>() as GLsizei, // Space between 2 consecutive attributes
            (3 * size_of::<f32>()) as *const c_void,   // Offset (after position)
        );

        gl::EnableVertexAttribArray(1);
        // Color attributes -------------------------------

        // Texture coords attributes ----------------------
        gl::VertexAttribPointer(
            2,                                         // Location = 1 in vertex shader
            2,                                         // Size of data (3 floats = vec3)
            gl::FLOAT,                                 // Data type
            gl::FALSE,                                 // Normalization
            (3 + 3 + 2) * size_of::<f32>() as GLsizei, // Space between 2 consecutive attributes
            (6 * size_of::<f32>()) as *const c_void,   // Offset (after color)
        );

        gl::EnableVertexAttribArray(2);
        // Texture coords attributes ----------------------

        // Unbind vao
        gl::BindVertexArray(0);
        // Unbind the vbo since it was bound to the vertex attribute pointer
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        // EBO should not be unbound since it is stored in the VAO
    }

    // Texture setup
    let mut foreground_texture = 0;
    let mut background_texture = 0;
    unsafe {
        // Background texture
        // Create texture object
        gl::GenTextures(1, &mut background_texture);
        gl::BindTexture(gl::TEXTURE_2D, background_texture);

        // Setup wrapping for s and t coordinates
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );

        // Set texture filtering for both magnified textures and minified textures
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Load the image data into the texture object
        let background_img = ImageReader::open("src/assets/background.jpg")
            .expect("Failed to load background texture image")
            .with_guessed_format()
            .expect("Failed to detect texture image format")
            .decode()
            .expect("Failed to decode texture image")
            .flipv()
            .to_rgb8();
        let background_pixels = background_img.as_bytes();

        gl::TexImage2D(
            gl::TEXTURE_2D,                              // Target texture
            0,                                           // Manual mipmap level
            gl::RGB as i32,                              // Texture object format
            background_img.width() as i32,               // Texture width
            background_img.height() as i32,              // Texture height
            0,                                           // Always 0
            gl::RGB,                                     // Image format
            gl::UNSIGNED_BYTE,                           // Image data type
            background_pixels.as_ptr() as *const c_void, // Image data
        );

        // Foreground texture
        // Automatically generate the mipmap
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Create texture object
        gl::GenTextures(1, &mut foreground_texture);
        gl::BindTexture(gl::TEXTURE_2D, foreground_texture);

        // Setup wrapping for s and t coordinates
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::MIRRORED_REPEAT as i32,
        );

        // Set texture filtering for both magnified textures and minified textures
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_MIN_FILTER,
            gl::LINEAR_MIPMAP_LINEAR as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        // Load the image data into the texture object
        let foreground_img = ImageReader::open("src/assets/foreground.png")
            .expect("Failed to load foreground texture image")
            .with_guessed_format()
            .expect("Failed to detect texture image format")
            .decode()
            .expect("Failed to decode texture image")
            .flipv()
            .to_rgba8();

        let foreground_pixels = foreground_img.as_bytes();

        gl::TexImage2D(
            gl::TEXTURE_2D,                              // Target texture
            0,                                           // Manual mipmap level
            gl::RGBA as i32,                             // Texture object format
            foreground_img.width() as i32,               // Texture width
            foreground_img.height() as i32,              // Texture height
            0,                                           // Always 0
            gl::RGBA,                                    // Image format
            gl::UNSIGNED_BYTE,                           // Image data type
            foreground_pixels.as_ptr() as *const c_void, // Image data
        );
        // Automatically generate the mipmap
        gl::GenerateMipmap(gl::TEXTURE_2D);
    };

    // Set the texture unit for each texture in the shader
    shader_program.use_program();
    shader_program
        .set_uniform_1i("background_texture", 0)
        .unwrap_or_else(|e| panic!("{e}"));
    shader_program
        .set_uniform_1i("foreground_texture", 1)
        .unwrap_or_else(|e| panic!("{e}"));

    while !window.should_close() {
        process_input(window, events);

        // Rendering ----------------------------
        unsafe {
            // Clear the color buffer with a specified color
            gl::ClearColor(0.2, 0.3, 0.3, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // Pass the texture data to the shaders
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, background_texture);

            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, foreground_texture);

            // Draw elements
            // Set shader program to use
            shader_program.use_program();
            // Allows to change data layout for different objects
            gl::BindVertexArray(vao);
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
    }
}

fn process_input(window: &mut PWindow, events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    for (_, event) in glfw::flush_messages(events) {
        match event {
            glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                window.set_should_close(true);
            }
            glfw::WindowEvent::Key(Key::P, _, Action::Press, _) => {
                let mut mode: GLint = 0;
                unsafe {
                    gl::GetIntegerv(gl::POLYGON_MODE, &mut mode);

                    if mode == gl::LINE as GLint {
                        gl::PolygonMode(gl::FRONT_AND_BACK, gl::FILL);
                    } else if mode == gl::FILL as GLint {
                        gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
                    }
                }
            }
            _ => {}
        }
    }
}
