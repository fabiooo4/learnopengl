extern crate nalgebra_glm as glm;

use basic_lighting::gl_utils::{
    self,
    camera::Camera,
    model::{Color, Cube, Model, Normalize},
    shader::{Shader, ShaderType},
};
use glm::{Mat4, TVec2, Vec3, vec2, vec3};

use gl::types::GLint;
use glfw::{self, Action, Context, Key, PWindow, ffi::glfwGetTime};

// Window
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;
const TITLE: &str = "HELLO BASIC LIGHTING!";

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

    println!("Exercise instructions:");
    println!(
        "Right now the light source is a boring static light source that doesn't move. Try to move the light source around the scene over time using either sin or cos. Watching the lighting change over time gives you a good understanding of Phong's lighting model"
    );

    println!("----------------------- KEYBINDS ------------------------");
    println!("          ESCAPE - Close the window");
    println!("               P - Toggle between fill and wireframe mode");
    println!("      W, A, S, D - Move around");
    println!("           SPACE - Fly up");
    println!("        LeftCtrl - Fly down");
    println!("      Left Shift - Sprint");
    println!("  Mouse movement - Look around");
    println!("          Scroll - Zoom in/out");

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

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
    }

    let mut delta_time;
    let mut camera = Camera::default();
    let mut mouse = MouseState::default();
    let cube = Cube::new(Color::from_hex(0xED5700));

    let light_color = Color::from_hex(0xFFFFFF);
    let mut light_position: Vec3 = vec3(1.2, 1., 2.);
    let light_scale: Vec3 = vec3(0.2, 0.2, 0.2);
    let light = Cube::new(light_color);

    // A shader program is the result of linking multiple compiled shaders
    let scene_shader: Shader = Shader::new(&[
        ("src/shaders/vertex.glsl", ShaderType::VertexShader),
        ("src/bin/exercise-2/shaders/fragment.glsl", ShaderType::FragmentShader),
    ])
    .unwrap_or_else(|log| panic!("{log}"));

    let emitter_shader: Shader = Shader::new(&[
        ("src/shaders/vertex.glsl", ShaderType::VertexShader),
        (
            "src/shaders/emitter_fragment.glsl",
            ShaderType::FragmentShader,
        ),
    ])
    .unwrap_or_else(|log| panic!("{log}"));

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
            let bg_color = Color::from_hex(0x191919);
            gl::ClearColor(
                bg_color.red().normalize(),
                bg_color.green().normalize(),
                bg_color.blue().normalize(),
                bg_color.alpha().normalize(),
            );
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            // Draw elements
            // Set shader program to use
            scene_shader.use_program();

            scene_shader
                .set_uniform_vec3("light_color", Vec3::from(light_color).normalize())
                .unwrap_or_else(|e| panic!("{e}"));

            scene_shader
                .set_uniform_vec3("light_pos", light_position)
                .unwrap_or_else(|e| panic!("{e}"));

            scene_shader
                .set_uniform_vec3("camera_pos", camera.pos)
                .unwrap_or_else(|e| panic!("{e}"));

            // Vertex coords -> World coords
            let mut model_mat: Mat4 = glm::identity();

            // World coords -> View coords
            // Camera
            let view_mat = glm::look_at(&camera.pos, &(camera.pos + camera.front), &camera.up);

            // View coords -> Clip coords
            let proj_mat: Mat4 = glm::perspective(
                WIDTH as f32 / HEIGHT as f32,
                f32::to_radians(camera.fov),
                0.1,
                100.,
            );

            scene_shader
                .set_uniform_mat_4fv("model", model_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            scene_shader
                .set_uniform_mat_4fv("view", view_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            scene_shader
                .set_uniform_mat_4fv("projection", proj_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            cube.draw();

            // Set light color
            emitter_shader.use_program();

            emitter_shader
                .set_uniform_3f(
                    "light_color",
                    light_color.red().normalize(),
                    light_color.green().normalize(),
                    light_color.blue().normalize(),
                )
                .unwrap_or_else(|e| panic!("{e}"));

            // Translate the light
            let light_angle = 5. * delta_time;
            light_position = glm::rotate_vec3(&light_position, f32::to_radians(light_angle), &vec3(0., 1., 0.));
            model_mat = glm::translate(&model_mat, &light_position);
            model_mat = glm::scale(&model_mat, &light_scale);

            emitter_shader
                .set_uniform_mat_4fv("model", model_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            emitter_shader
                .set_uniform_mat_4fv("view", view_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            emitter_shader
                .set_uniform_mat_4fv("projection", proj_mat)
                .unwrap_or_else(|e| panic!("{e}"));

            light.draw();
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

    if window.get_key(Key::Space) == Action::Press {
        camera.pos += camera.up * velocity;
    }
    if window.get_key(Key::LeftControl) == Action::Press {
        camera.pos -= camera.up * velocity;
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
