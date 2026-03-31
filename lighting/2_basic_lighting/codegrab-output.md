# Project Structure

```
2_basic_lighting/
├── proc_macros/
│   ├── src/
│   │   ├── lib.rs
│   │   └── vertex_layout.rs
│   ├── Cargo.lock
│   └── Cargo.toml
├── src/
│   ├── bin/
│   │   └── exercise-4/
│   │       ├── shaders/
│   │       │   ├── fragment.glsl
│   │       │   └── vertex.glsl
│   │       └── main.rs
│   ├── gl_utils/
│   │   ├── camera.rs
│   │   ├── mod.rs
│   │   ├── model.rs
│   │   └── shader.rs
│   ├── shaders/
│   │   ├── emitter_fragment.glsl
│   │   ├── fragment.glsl
│   │   └── vertex.glsl
│   ├── lib.rs
│   └── main.rs
└── Cargo.toml
```

# Project Files

## File: `proc_macros/src/lib.rs`

```rust
extern crate proc_macro;
mod vertex_layout;

use proc_macro::TokenStream;
use quote::quote;
use syn::{Data::*, DeriveInput, parse_macro_input};

use crate::vertex_layout::gen_layout_for_struct;

/// Derives a vertex layout for a struct, generating OpenGL calls to set up the vertex attributes
/// based on the provided field attributes.
///
/// # Attributes
///
/// The struct fields wich should be included in the vertex layout must have a `layout` attribute
/// with the following properties:
/// - `location`: The vertex attribute location in the shader (integer).
/// - `elements`: The number of components in the attribute (e.g., 3 for a `vec3`).
///
/// # Example
///
/// ```rust
/// use proc_macros::VertexLayout;
///
/// #[derive(VertexLayout)]
/// struct Vertex {
///    #[layout(location = 0, elements = 3)]
///    position: Vec<f32; 3>,
///
///    #[layout(location = 1, elements = 3)]
///    color: Vec<f32; 3>,
/// }
/// ```
///
/// The above example will generate OpenGL calls to set up the vertex attributes for `position` and
/// `color` with the specified locations and element counts. The generated code will be similar to:
///
/// ```rust
/// gl::VertexAttribPointer(
///     0 as gl::types::GLuint,
///     3 as gl::types::GLint,
///     gl::FLOAT,
///     gl::FALSE,
///     std::mem::size_of::<Vertex>() as gl::types::GLsizei,
///     std::mem::offset_of!(Vertex, position) as *const std::ffi::c_void,
/// );
/// gl::EnableVertexAttribArray(#loc);
/// ```
///
/// # Panics
///
/// Panics if the macro is applied to an enum or if the struct fields do not have the required
/// `layout` attributes.
#[proc_macro_derive(VertexLayout, attributes(layout))]
pub fn vertex_layout(item: TokenStream) -> TokenStream {
    let DeriveInput {
        ident,
        data,
        generics,
        ..
    } = parse_macro_input!(item as DeriveInput);
    let where_clause = &generics.where_clause;

    let gl_vertex_layouts = match data {
        Struct(data_struct) => gen_layout_for_struct(ident.clone(), data_struct),
        Union(_data_union) => todo!("Add support for unions in VertexLayout"),
        Enum(_data_enum) => panic!("VertexLayout cannot be derived for enums"),
    };

    // Generate each field's layout setup
    quote! {
      impl #generics #ident #generics #where_clause {
          fn setup_layout() {
              unsafe {
                  #(#gl_vertex_layouts)*
              }
          }
      }
    }
    .into()
}

```

## File: `proc_macros/src/vertex_layout.rs`

```rust
use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataStruct, Fields::*, FieldsNamed, Ident, LitInt, meta::ParseNestedMeta};

#[derive(Default)]
pub struct VertexLayoutAttributes {
    location: Option<LitInt>,
    elements: Option<LitInt>,
}

impl VertexLayoutAttributes {
    fn parse(&mut self, meta: ParseNestedMeta) -> syn::parse::Result<()> {
        if meta.path.is_ident("location") {
            self.location = Some(meta.value()?.parse()?);
            Ok(())
        } else if meta.path.is_ident("elements") {
            self.elements = Some(meta.value()?.parse()?);
            Ok(())
        } else {
            Err(meta.error("Unsupported vertex_layout property"))
        }
    }
}

pub fn gen_layout_for_struct(struct_name: Ident, data_struct: DataStruct) -> Vec<TokenStream> {
    match data_struct.fields {
        Named(fields) => handle_named_fields(struct_name, fields),
        Unnamed(_fields) =>
        /* handle_unnamed_fields(fields) */
        {
            todo!("Add support for tuple structs in VertexLayout")
        }
        Unit => panic!("Unit structs are not supported for VertexLayout"),
    }
}

fn handle_named_fields(struct_name: Ident, fields: FieldsNamed) -> Vec<TokenStream> {
    let mut opengl_calls = Vec::new();

    for field in fields.named {
        // Skip fields without attributes
        if field.attrs.is_empty() {
            continue;
        }

        let field_name = field.ident.expect("Expected named fields");
        let mut layout_attrs = VertexLayoutAttributes::default();

        // Parse the field's attribute
        for attr in field.attrs {
            if attr.path().is_ident("layout") {
                attr.parse_nested_meta(|meta| layout_attrs.parse(meta))
                    .expect("Failed to parse layout attributes");
            }
        }

        if let Some(loc_lit) = layout_attrs.location
            && let Some(elems_lit) = layout_attrs.elements
        {
            let loc = loc_lit
                .base10_parse::<u32>()
                .expect("'location' must be an integer");

            let elems = elems_lit
                .base10_parse::<u32>()
                .expect("'elements' must be an integer");

            // Generate the OpenGL calls for this field
            opengl_calls.push(quote! {
                gl::VertexAttribPointer(
                    #loc as gl::types::GLuint,
                    #elems as gl::types::GLint,
                    gl::FLOAT, // todo!("Add support for other types")
                    gl::FALSE, // todo!("Add flag for normalized attributes")
                    std::mem::size_of::<Self>() as gl::types::GLsizei,
                    std::mem::offset_of!(#struct_name, #field_name) as *const std::ffi::c_void,
                );
                gl::EnableVertexAttribArray(#loc);
            })
        }
    }

    opengl_calls
}

/* fn handle_unnamed_fields(fields: FieldsUnnamed) -> Vec<TokenStream> {
    let _unnamed_fields_count = fields.unnamed.iter().count();
    vec![]
} */

```

## File: `proc_macros/Cargo.lock`

```text
# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "proc-macro2"
version = "1.0.106"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "8fd00f0bb2e90d81d1044c2b32617f68fcb9fa3bb7640c23e9c748e53fb30934"
dependencies = [
 "unicode-ident",
]

[[package]]
name = "proc_macros"
version = "0.1.0"
dependencies = [
 "proc-macro2",
 "quote",
 "syn",
]

[[package]]
name = "quote"
version = "1.0.45"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "41f2619966050689382d2b44f664f4bc593e129785a36d6ee376ddf37259b924"
dependencies = [
 "proc-macro2",
]

[[package]]
name = "syn"
version = "2.0.117"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e665b8803e7b1d2a727f4023456bbbbe74da67099c585258af0ad9c5013b9b99"
dependencies = [
 "proc-macro2",
 "quote",
 "unicode-ident",
]

[[package]]
name = "unicode-ident"
version = "1.0.24"
source = "registry+https://github.com/rust-lang/crates.io-index"
checksum = "e6e4313cd5fcd3dad5cafa179702e2b244f760991f45397d14d4ebf38247da75"

```

## File: `proc_macros/Cargo.toml`

```toml
[package]
name = "proc_macros"
version = "0.1.0"
edition = "2024"

[lib]
name = "proc_macros"
path = "src/lib.rs"
proc-macro = true

[dependencies]
proc-macro2 = "1.0.106"
quote = "1.0.45"
syn = { version = "2.0.117", features = ["parsing", "derive", "extra-traits"] }

```

## File: `src/bin/exercise-4/shaders/fragment.glsl`

```glsl
#version 330 core

in vec3 vertex_color;

out vec4 frag_color;

void main() {
  frag_color = vec4(vertex_color, 1.0);
}

```

## File: `src/bin/exercise-4/shaders/vertex.glsl`

```glsl
#version 330 core
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 texture;
layout(location = 3) in vec3 color;

// Projection
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

// Lighting
uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 camera_pos;

out vec3 vertex_color;

void main() {
  gl_Position = projection * view * model * vec4(pos, 1.0);

  vec3 world_vertex_pos = vec3(model * vec4(pos, 1.0));
  vec3 normal = mat3(transpose(inverse(model))) * norm; // Costly operation

  // Ambient light
  float ambient_strength = 0.1;
  vec3 ambient_light = light_color * ambient_strength;

  // Diffuse light
  vec3 vertex_normal = normalize(normal);
  vec3 light_direction = normalize(light_pos - world_vertex_pos);
  float diffuse = max(dot(vertex_normal, light_direction), 0.0);
  vec3 diffuse_light = diffuse * light_color;

  // Specular light
  float specular_strength = 0.5;
  vec3 view_direction = normalize(camera_pos - world_vertex_pos);
  // The light direction needs to point from the light to the fragment, so it is negated
  vec3 reflected_direction = reflect(-light_direction, vertex_normal);
  int shine = 32;
  float specular = pow(max(dot(view_direction, reflected_direction), 0.0), shine);
  vec3 specular_light = specular_strength * specular * light_color;  

  vec3 lighting = (ambient_light + diffuse_light + specular_light) * vertex_color;

  vertex_color = lighting;
}

```

## File: `src/bin/exercise-4/main.rs`

```rust
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
const TITLE: &str = "Basic Lighting - Exercise 4";

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
        "Implement Gouraud shading instead of Phong shading"
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
        ("src/bin/exercise-4/shaders/vertex.glsl", ShaderType::VertexShader),
        ("src/bin/exercise-4/shaders/fragment.glsl", ShaderType::FragmentShader),
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

```

## File: `src/gl_utils/camera.rs`

```rust
use glm::{Vec3, vec3};

pub struct Camera {
    pub speed: f32,
    pub speed_mul: f32,
    pub is_sprinting: bool,

    pub fov: f32,

    pub pos: Vec3,
    pub front: Vec3,
    pub up: Vec3,

    pub pitch: f32,
    pub yaw: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            speed: 2.5,
            speed_mul: 4.,
            is_sprinting: false,

            fov: 45.,

            pos: vec3(0., 0., 3.),
            front: vec3(0., 0., -1.),
            up: vec3(0., 1., 0.),

            pitch: 0.,
            yaw: -90.,
        }
    }
}

impl Camera {
    /// Toggles a faster camera speed
    pub fn toggle_sprint(&mut self) {
        if self.is_sprinting {
            self.speed /= self.speed_mul;
            self.is_sprinting = false;
        } else {
            self.speed *= self.speed_mul;
            self.is_sprinting = true;
        }
    }
}

```

## File: `src/gl_utils/mod.rs`

```rust
pub mod shader;
pub mod model;
pub mod camera;

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
) -> (glfw::Glfw, glfw::PWindow, glfw::GlfwReceiver<(f64, glfw::WindowEvent)>) {
    // Initialize glfw with OpenGL settings
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to initialize glfw");
    glfw.window_hint(glfw::WindowHint::ContextVersionMajor(3));
    glfw.window_hint(glfw::WindowHint::ContextVersionMinor(3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Create a window
    let (mut window, event) = match window_mode {
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

    (glfw, window, event)
}

/// Default framebuffer size callback that updates the OpenGL viewport when the window is resized.
fn default_framebuffer_size_callback(_: &mut glfw::Window, new_width: i32, new_height: i32) {
    unsafe {
        gl::Viewport(0, 0, new_width, new_height);
    }
}

```

## File: `src/gl_utils/model.rs`

```rust
use glm::{Vec2, Vec3, vec2, vec3};
use proc_macros::VertexLayout;
use std::ffi::c_void;

pub trait Model {
    fn draw(&self);
}

#[repr(C)]
#[derive(Default, Clone, Copy, VertexLayout)]
pub struct Vertex {
    #[layout(location = 0, elements = 3)]
    pub position: Vec3,

    #[layout(location = 1, elements = 3)]
    pub normal: Vec3,

    #[layout(location = 2, elements = 2)]
    pub texture: Vec2,

    #[layout(location = 3, elements = 3)]
    pub color: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, normal: Vec3, texture: Vec2, color: Vec3) -> Self {
        Vertex {
            position,
            normal,
            texture,
            color,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Default, Debug)]
pub struct Color {
    pub hex: u32,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        let red: u32 = (r as u32) << (8 * 3);
        let green: u32 = (g as u32) << (8 * 2);
        let blue: u32 = (b as u32) << 8;
        let alpha: u32 = a as u32;

        Color {
            hex: red | green | blue | alpha,
        }
    }

    pub fn red(&self) -> u8 {
        ((self.hex & 0xff000000) >> (8 * 3)) as u8
    }

    pub fn green(&self) -> u8 {
        ((self.hex & 0x00ff0000) >> (8 * 2)) as u8
    }

    pub fn blue(&self) -> u8 {
        ((self.hex & 0x0000ff00) >> 8) as u8
    }

    pub fn alpha(&self) -> u8 {
        self.hex as u8
    }

    pub fn from_hex(hex: u32) -> Color {
        Color {
            hex: (hex << 8) | 0x00000011,
        }
    }

    pub fn from_hex_alpha(hex: u32) -> Color {
        Color { hex }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::new(r, g, b, 1)
    }
}

impl From<Color> for Vec3 {
    fn from(val: Color) -> Self {
        vec3(
            val.red().normalize(),
            val.green().normalize(),
            val.blue().normalize(),
        )
    }
}

pub trait Normalize {
    fn normalize(self) -> f32;
}

impl Normalize for u8 {
    fn normalize(self) -> f32 {
        self as f32 / u8::MAX as f32
    }
}

#[derive(Clone)]
pub struct Cube {
    vao: u32,
    vbo: u32,

    vertices: Vec<Vertex>,

    pub color: Color,
}

impl Cube {
    fn vertices(color: Color) -> Vec<Vertex> {
        let color = color.into();

        #[rustfmt::skip]
        let res = vec![
           Vertex::new(vec3(-0.5, -0.5, -0.5), vec3( 0.0,  0.0, -1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5, -0.5), vec3( 0.0,  0.0, -1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5, -0.5), vec3( 0.0,  0.0, -1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5, -0.5), vec3( 0.0,  0.0, -1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5, -0.5), vec3( 0.0,  0.0, -1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5, -0.5), vec3( 0.0,  0.0, -1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5,  0.5), vec3( 0.0,  0.0,  1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5,  0.5), vec3( 0.0,  0.0,  1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5,  0.5), vec3( 0.0,  0.0,  1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5,  0.5), vec3( 0.0,  0.0,  1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5,  0.5), vec3( 0.0,  0.0,  1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5,  0.5), vec3( 0.0,  0.0,  1.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(-1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5, -0.5), vec3(-1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(-1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(-1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(-1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(-1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5,  0.5), vec3( 1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5, -0.5), vec3( 1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5, -0.5), vec3( 1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5, -0.5), vec3( 1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5,  0.5), vec3( 1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5,  0.5), vec3( 1.0,  0.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5, -0.5), vec3( 0.0, -1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5, -0.5), vec3( 0.0, -1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5,  0.5), vec3( 0.0, -1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5, -0.5,  0.5), vec3( 0.0, -1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5,  0.5), vec3( 0.0, -1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5, -0.5, -0.5), vec3( 0.0, -1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5, -0.5), vec3( 0.0,  1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5, -0.5), vec3( 0.0,  1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5,  0.5), vec3( 0.0,  1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3( 0.5,  0.5,  0.5), vec3( 0.0,  1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5,  0.5), vec3( 0.0,  1.0,  0.0), vec2(0.0, 0.0), color),
           Vertex::new(vec3(-0.5,  0.5, -0.5), vec3( 0.0,  1.0,  0.0), vec2(0.0, 0.0), color),
        ];
        res
    }

    pub fn new(color: Color) -> Self {
        let vertices = Cube::vertices(color);

        // Create and bind a vertex buffer object (vertex attribute storage),
        // and a vertex array object (attribute layout)
        // an element buffer object (vertex indices order)
        let mut vao = 0;
        let mut vbo = 0;
        // let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            // gl::GenBuffers(1, &mut ebo);

            // VAO must be bound first
            gl::BindVertexArray(vao);

            // Copy vertex data into the vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(vertices.as_slice()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            Vertex::setup_layout();

            // Unbind vao
            gl::BindVertexArray(0);
            // Unbind the vbo since it was bound to the vertex attribute pointer
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // EBO should not be unbound since it is stored in the VAO
        };

        Cube {
            vao,
            vbo,
            vertices,
            color,
        }
    }

    pub fn with_color(&self, color: Color) -> Cube {
        Cube::new(color)
    }
}

impl Default for Cube {
    fn default() -> Self {
        let color = Color::from_hex(0xffffff);

        let vertices = Cube::vertices(color);

        // Create and bind a vertex buffer object (vertex attribute storage),
        // and a vertex array object (attribute layout)
        // an element buffer object (vertex indices order)
        let mut vao = 0;
        let mut vbo = 0;
        // let mut ebo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            // gl::GenBuffers(1, &mut ebo);

            // VAO must be bound first
            gl::BindVertexArray(vao);

            // Copy vertex data into the vbo
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                size_of_val(vertices.as_slice()) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            Vertex::setup_layout();

            // Unbind vao
            gl::BindVertexArray(0);
            // Unbind the vbo since it was bound to the vertex attribute pointer
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // EBO should not be unbound since it is stored in the VAO
        };

        Cube {
            vao,
            vbo,
            vertices,
            color,
        }
    }
}

impl Model for Cube {
    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32);
        }
    }
}

impl Drop for Cube {
    fn drop(&mut self) {
        // Deallocate resources
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
            // gl::DeleteBuffers(1, &self.ebo);
        }
    }
}

```

## File: `src/gl_utils/shader.rs`

```rust
use std::{ffi::CString, fs::read_to_string, ptr::null_mut};

use gl::types::GLint;
use glm::Vec3;
use nalgebra_glm::Mat4;

#[derive(Clone, Copy)]
pub enum ShaderType {
    VertexShader = gl::VERTEX_SHADER as isize,
    FragmentShader = gl::FRAGMENT_SHADER as isize,
    ComputeShader = gl::COMPUTE_SHADER as isize,
    GeometryShader = gl::GEOMETRY_SHADER as isize,
}

pub struct Shader {
    pub program_id: u32,
}

impl Shader {
    /// Creates a shader object, applies the source to the object and compiles the shader checking if
    /// the compilation was succesful.
    ///
    /// Returns the shader object id.
    ///
    /// # Errors
    /// If file reading fails or if the shader fails to compile, an error string is returned.
    fn create_shader(shader_path: &str, shader_type: ShaderType) -> Result<u32, String> {
        let shader_src = read_to_string(shader_path)
            .unwrap_or_else(|e| panic!("Error: Failed to read file {}\n{}", shader_path, e));

        let vertex_shader = unsafe { gl::CreateShader(shader_type as u32) };

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        unsafe {
            // Set shader source to shader object
            gl::ShaderSource(
                vertex_shader,
                1,
                &(shader_src.as_ptr() as *const i8),
                &(shader_src.len() as GLint),
            );

            // Compile the shader
            gl::CompileShader(vertex_shader);

            // Check compilation errors
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);

            if success == 0 {
                gl::GetShaderInfoLog(
                    vertex_shader,
                    info_log.len() as i32,
                    null_mut(),
                    info_log.as_mut_ptr(),
                );

                return Err(format!(
                    "Error: Shader compilation failed\n{}",
                    std::str::from_utf8(&info_log.map(|c| c as u8))
                        .expect("Failed to convert info log to string")
                ));
            }
        };

        Ok(vertex_shader)
    }

    /// Creates a program object, links all the provided shader objects and checks if the linking was
    /// successful.
    ///
    /// Returns the program object id.
    ///
    /// # Errors
    /// If the program fails to link, an error string is returned.
    fn link_program(shaders: &[u32]) -> Result<u32, String> {
        let shader_program: u32 = unsafe { gl::CreateProgram() };

        let mut success = 0;
        let mut info_log: [i8; 512] = [0; 512];

        unsafe {
            // Attach all shaders to the program
            for &shader in shaders {
                gl::AttachShader(shader_program, shader);
            }

            // Link the attached shaderss
            gl::LinkProgram(shader_program);

            // Check compilation errors
            gl::GetProgramiv(shader_program, gl::LINK_STATUS, &mut success);

            if success == 0 {
                gl::GetProgramInfoLog(
                    shader_program,
                    info_log.len() as i32,
                    null_mut(),
                    info_log.as_mut_ptr(),
                );

                return Err(format!(
                    "Error: Program linking failed\n{}",
                    std::str::from_utf8(&info_log.map(|c| c as u8))
                        .expect("Failed to convert info log to string")
                ));
            }
        };

        // Remove all the shaders objects
        for &shader in shaders {
            unsafe { gl::DeleteShader(shader) };
        }

        Ok(shader_program)
    }

    /// Creates a shader program given a list of shader sources.
    ///
    /// Returns the program object id.
    ///
    /// # Errors
    /// If any of the shaders fail to compile or if the program fails to link, an error string is
    /// returned.
    pub fn new(shaders: &[(&str, ShaderType)]) -> Result<Self, String> {
        let mut compiled = vec![];
        for &(shader_path, shader_type) in shaders {
            compiled.push(Shader::create_shader(shader_path, shader_type)?);
        }

        match Shader::link_program(&compiled) {
            Ok(id) => Ok(Shader { program_id: id }),
            Err(e) => Err(e),
        }
    }

    /// Sets the current shader program for rendering
    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.program_id);
        }
    }

    /// Gets the given uniform location
    ///
    /// Returns the uniform location number
    ///
    /// # Errors
    /// If the location was not found, an error string is returned
    fn get_uniform_location(&self, uniform: &str) -> Result<i32, String> {
        let c_str = CString::new(uniform)
            .map_err(|e| format!("Error: Uniform name is null terminated\n{}", e))?;

        let result = unsafe { gl::GetUniformLocation(self.program_id, c_str.as_ptr()) };

        if result == -1 {
            Err(format!(
                "Error: Could not find uniform location for \"{}\"",
                uniform
            ))
        } else {
            Ok(result)
        }
    }

    /// Sets the value of a float uniform
    pub fn set_uniform_1f(&self, uniform: &str, val: f32) -> Result<(), String> {
        unsafe { gl::Uniform1f(self.get_uniform_location(uniform)?, val) };
        Ok(())
    }

    /// Sets the value of a vec2 float uniform
    pub fn set_uniform_2f(&self, uniform: &str, val1: f32, val2: f32) -> Result<(), String> {
        unsafe { gl::Uniform2f(self.get_uniform_location(uniform)?, val1, val2) };
        Ok(())
    }

    /// Sets the value of a vec3 float uniform
    pub fn set_uniform_3f(
        &self,
        uniform: &str,
        val1: f32,
        val2: f32,
        val3: f32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform3f(self.get_uniform_location(uniform)?, val1, val2, val3) };
        Ok(())
    }

    pub fn set_uniform_vec3(
        &self,
        uniform: &str,
        vec: Vec3,
    ) -> Result<(), String> {
        self.set_uniform_3f(uniform, vec.x, vec.y, vec.z)
    }

    /// Sets the value of a vec4 float uniform
    pub fn set_uniform_4f(
        &self,
        uniform: &str,
        val1: f32,
        val2: f32,
        val3: f32,
        val4: f32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform4f(self.get_uniform_location(uniform)?, val1, val2, val3, val4) };
        Ok(())
    }

    /// Sets the value of an integer uniform
    pub fn set_uniform_1i(&self, uniform: &str, val: i32) -> Result<(), String> {
        unsafe { gl::Uniform1i(self.get_uniform_location(uniform)?, val) };
        Ok(())
    }

    /// Sets the value of a vec2 integer uniform
    pub fn set_uniform_2i(&self, uniform: &str, val1: i32, val2: i32) -> Result<(), String> {
        unsafe { gl::Uniform2i(self.get_uniform_location(uniform)?, val1, val2) };
        Ok(())
    }

    /// Sets the value of a vec3 integer uniform
    pub fn set_uniform_3i(
        &self,
        uniform: &str,
        val1: i32,
        val2: i32,
        val3: i32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform3i(self.get_uniform_location(uniform)?, val1, val2, val3) };
        Ok(())
    }

    /// Sets the value of a vec4 integer uniform
    pub fn set_uniform_4i(
        &self,
        uniform: &str,
        val1: i32,
        val2: i32,
        val3: i32,
        val4: i32,
    ) -> Result<(), String> {
        unsafe { gl::Uniform4i(self.get_uniform_location(uniform)?, val1, val2, val3, val4) };
        Ok(())
    }

    /// Sets the value of a mat4 float uniform
    pub fn set_uniform_mat_4fv(&self, uniform: &str, mat: Mat4) -> Result<(), String> {
        let matrix_num = 1;
        let transpose = gl::FALSE;
        let values = glm::value_ptr(&mat).as_ptr();
        unsafe {
            gl::UniformMatrix4fv(
                self.get_uniform_location(uniform)?,
                matrix_num,
                transpose,
                values,
            )
        };
        Ok(())
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program_id);
        }
    }
}

```

## File: `src/shaders/emitter_fragment.glsl`

```glsl
#version 330 core

uniform vec3 light_color;

out vec4 frag_color;

void main() {
  frag_color = vec4(light_color, 1.0);
}

```

## File: `src/shaders/fragment.glsl`

```glsl
#version 330 core

uniform vec3 light_color;
uniform vec3 light_pos;
uniform vec3 camera_pos;

in vec3 vertex_color;
in vec3 fragment_pos;
in vec3 normal;

out vec4 frag_color;

void main() {

  // Ambient light
  float ambient_strength = 0.1;
  vec3 ambient_light = light_color * ambient_strength;

  // Diffuse light
  vec3 fragment_normal = normalize(normal);
  vec3 light_direction = normalize(light_pos - fragment_pos);
  float diffuse = max(dot(fragment_normal, light_direction), 0.0);
  vec3 diffuse_light = diffuse * light_color;

  // Specular light
  float specular_strength = 0.5;
  vec3 view_direction = normalize(camera_pos - fragment_pos);
  // The light direction needs to point from the light to the fragment, so it is negated
  vec3 reflected_direction = reflect(-light_direction, fragment_normal);
  int shine = 32;
  float specular = pow(max(dot(view_direction, reflected_direction), 0.0), shine);
  vec3 specular_light = specular_strength * specular * light_color;  

  vec3 lighting = (ambient_light + diffuse_light + specular_light) * vertex_color;

  frag_color = vec4(lighting, 1.0);
}

```

## File: `src/shaders/vertex.glsl`

```glsl
#version 330 core
layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 norm;
layout(location = 2) in vec2 texture;
layout(location = 3) in vec3 color;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 vertex_color;
out vec3 fragment_pos;
out vec3 normal;

void main() {
  gl_Position = projection * view * model * vec4(pos, 1.0);

  fragment_pos = vec3(model * vec4(pos, 1.0));
  normal = mat3(transpose(inverse(model))) * norm; // Costly operation
  vertex_color = color;
}

```

## File: `src/lib.rs`

```rust
pub mod gl_utils;
extern crate nalgebra_glm as glm;

```

## File: `src/main.rs`

```rust
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
    let light_position: Vec3 = vec3(1.2, 1., 2.);
    let light_scale: Vec3 = vec3(0.2, 0.2, 0.2);
    let light = Cube::new(light_color);

    // A shader program is the result of linking multiple compiled shaders
    let scene_shader: Shader = Shader::new(&[
        ("src/shaders/vertex.glsl", ShaderType::VertexShader),
        ("src/shaders/fragment.glsl", ShaderType::FragmentShader),
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

```

## File: `Cargo.toml`

```toml
[package]
name = "basic_lighting"
version = "0.1.0"
edition = "2024"

[dependencies]
proc_macros = { path = "./proc_macros" }

gl = "0.14.0"
glfw = "0.60.0"
image = "0.25.8"
nalgebra-glm = "0.20.0"

```

