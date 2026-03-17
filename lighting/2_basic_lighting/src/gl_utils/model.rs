use glm::{Vec3, vec3};
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
    pub color: Vec3,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec3) -> Self {
        Vertex { position, color }
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
        let color: Vec3 = color.into();

        vec![
            Vertex::new(vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(vec3(0.5, -0.5, -0.5), color),
            Vertex::new(vec3(0.5, 0.5, -0.5), color),
            Vertex::new(vec3(0.5, 0.5, -0.5), color),
            Vertex::new(vec3(-0.5, 0.5, -0.5), color),
            Vertex::new(vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(vec3(0.5, -0.5, 0.5), color),
            Vertex::new(vec3(0.5, 0.5, 0.5), color),
            Vertex::new(vec3(0.5, 0.5, 0.5), color),
            Vertex::new(vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(vec3(-0.5, 0.5, -0.5), color),
            Vertex::new(vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(vec3(0.5, 0.5, 0.5), color),
            Vertex::new(vec3(0.5, 0.5, -0.5), color),
            Vertex::new(vec3(0.5, -0.5, -0.5), color),
            Vertex::new(vec3(0.5, -0.5, -0.5), color),
            Vertex::new(vec3(0.5, -0.5, 0.5), color),
            Vertex::new(vec3(0.5, 0.5, 0.5), color),
            Vertex::new(vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(vec3(0.5, -0.5, -0.5), color),
            Vertex::new(vec3(0.5, -0.5, 0.5), color),
            Vertex::new(vec3(0.5, -0.5, 0.5), color),
            Vertex::new(vec3(-0.5, -0.5, 0.5), color),
            Vertex::new(vec3(-0.5, -0.5, -0.5), color),
            Vertex::new(vec3(-0.5, 0.5, -0.5), color),
            Vertex::new(vec3(0.5, 0.5, -0.5), color),
            Vertex::new(vec3(0.5, 0.5, 0.5), color),
            Vertex::new(vec3(0.5, 0.5, 0.5), color),
            Vertex::new(vec3(-0.5, 0.5, 0.5), color),
            Vertex::new(vec3(-0.5, 0.5, -0.5), color),
        ]
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
