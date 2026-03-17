extern crate nalgebra_glm as glm;

use basic_lighting::gl_utils::{
    self,
    camera::Camera,
    model::{Cube, Model},
    shader::{Shader, ShaderType},
};
use glm::{Mat4, TVec2, Vec3, vec2, vec3};

use gl::types::GLint;
use glfw::{self, Action, Context, Key, PWindow, ffi::glfwGetTime};

// Window
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TITLE: &str = "HELLO COLORS!";

// Mouse
const SENSITIVITY: f64 = 0.1;

struct MouseState {
    last_pos: TVec2<f64>,
    is_first_mouse: bool,
}

impl Default for MouseState {
    fn default() -> Self {
        Self {
            last_pos: Default::default(),
            is_first_mouse: true,
        }
    }
}

fn main() {
    let (mut glfw, mut window, events) =
        gl_utils::init_window(WIDTH, HEIGHT, TITLE, gl_utils::WindowMode::Windowed, None);

    println!("----------------------- KEYBINDS ------------------------");
    println!("          ESCAPE - Close the window");
    println!("               P - Toggle between fill and wireframe mode");
    println!("      W, A, S, D - Move the camera");
    println!("  Mouse movement - Look around");
    println!("          Scroll - Zoom in/out");
    println!("      Left Shift - Sprint");

    render_loop(&mut glfw, &mut window, &events);
}

fn render_loop(
    glfw: &mut glfw::Glfw,
    window: &mut PWindow,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
) {
    // Capture mouse pointer
    window.set_cursor_mode(glfw::CursorMode::Disabled);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    let mut delta_time;
    let mut camera = Camera::default();
    let mut mouse = MouseState::default();
    let cube = Cube::new();

    // A shader program is the result of linking multiple compiled shaders
    let shader_program: Shader = Shader::new(&[
        ("src/shaders/vertex.glsl", ShaderType::VertexShader),
        ("src/shaders/fragment.glsl", ShaderType::FragmentShader),
    ])
    .unwrap_or_else(|log| panic!("{log}"));

    #[rustfmt::skip]
    let cube_positions: &[Vec3] = &[
        vec3( 0.0,  0.0,  0.0 ),
        vec3( 2.0,  5.0, -15.0),
        vec3(-1.5, -2.2, -2.5 ),
        vec3(-3.8, -2.0, -12.3),
        vec3( 2.4, -0.4, -3.5 ),
        vec3(-1.7,  3.0, -7.5 ),
        vec3( 1.3, -2.0, -2.5 ),
        vec3( 1.5,  2.0, -2.5 ),
        vec3( 1.5,  0.2, -1.5 ),
        vec3(-1.3,  1.0, -1.5 ),
    ];

    let mut last_frame = 0.;
    while !window.should_close() {
        // Calculate deltatime
        let current_frame = unsafe { glfwGetTime() } as f32;
        delta_time = current_frame - last_frame;
        last_frame = current_frame;

        process_input(window, events, &mut camera, &mut mouse, delta_time);

        // Rendering ----------------------------
        unsafe {
            // Clear the color buffer with a specified color
            gl::ClearColor(0.2, 0.2, 0.2, 1.);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Draw elements
            // Set shader program to use
            shader_program.use_program();

            // World coords -> View coords
            // Camera
            let view_mat = glm::look_at(&camera.pos, &(camera.pos + camera.front), &camera.up);

            shader_program
                .set_uniform_mat_4fv("view", view_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            // View coords -> Clip coords
            let proj_mat: Mat4 = glm::perspective(
                WIDTH as f32 / HEIGHT as f32,
                f32::to_radians(camera.fov),
                0.1,
                100.,
            );

            shader_program
                .set_uniform_mat_4fv("projection", proj_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            for (i, pos) in cube_positions.iter().enumerate() {
                // Vertex coords -> World coords
                let mut model_mat: Mat4 = glm::identity();

                // Translate each cube by the pos vector and then rotate it
                model_mat = glm::translate(&model_mat, pos);

                let angle = 20. * i as f32;
                model_mat = glm::rotate(&model_mat, f32::to_radians(angle), &vec3(1., 0.3, 0.5));

                // Assign the new model matrix and render
                shader_program
                    .set_uniform_mat_4fv("model", model_mat)
                    .unwrap_or_else(|e| panic!("{e}"));

                cube.draw();
            }
        }
        // Rendering ----------------------------

        window.swap_buffers();
        glfw.poll_events();
    }
}

fn process_input(
    window: &mut PWindow,
    events: &glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,
    camera: &mut Camera,
    mouse: &mut MouseState,
    delta_time: f32,
) {
    if window.get_key(Key::LeftShift) == Action::Press && !camera.is_sprinting {
        camera.toggle_sprint();
    }

    if window.get_key(Key::LeftShift) == Action::Release && camera.is_sprinting {
        camera.toggle_sprint();
    }

    // Polling
    let velocity = camera.speed * delta_time;

    if window.get_key(Key::W) == Action::Press {
        camera.pos += camera.front * velocity;
    }
    if window.get_key(Key::S) == Action::Press {
        camera.pos -= camera.front * velocity;
    }
    if window.get_key(Key::A) == Action::Press {
        let camera_right = glm::normalize(&glm::cross(&camera.front, &camera.up));

        camera.pos -= camera_right * velocity;
    }
    if window.get_key(Key::D) == Action::Press {
        let camera_right = glm::normalize(&glm::cross(&camera.front, &camera.up));

        camera.pos += camera_right * velocity;
    }

    // Discrete events
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

            glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
                if mouse.is_first_mouse {
                    mouse.last_pos = vec2(x_pos, y_pos);
                    mouse.is_first_mouse = false;
                }

                // Calculate offset since the last position
                let offset = vec2(x_pos - mouse.last_pos.x, mouse.last_pos.y - y_pos) * SENSITIVITY;

                mouse.last_pos = vec2(x_pos, y_pos);

                camera.yaw += offset.x as f32;
                camera.pitch += (offset.y as f32).clamp(-89.0, 89.0);

                let camera_direction: Vec3 = vec3(
                    f32::cos(f32::to_radians(camera.yaw)) * f32::cos(f32::to_radians(camera.pitch)),
                    f32::sin(f32::to_radians(camera.pitch)),
                    f32::sin(f32::to_radians(camera.yaw)) * f32::cos(f32::to_radians(camera.pitch)),
                );

                camera.front = glm::normalize(&camera_direction);
            }

            glfw::WindowEvent::Scroll(_x_offset, y_offset) => {
                camera.fov -= y_offset as f32;
                camera.fov = camera.fov.clamp(1., 45.);
            }

            _ => {}
        }
    }
}
