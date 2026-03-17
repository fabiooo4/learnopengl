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

pub struct Cube {
    vao: u32,
    vbo: u32,

    vertices: Vec<Vertex>,
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Cube {
    fn default() -> Self {
        #[rustfmt::skip]
        let vertices = vec![
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),

            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),

            Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),

            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),

            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5, -0.5, -0.5), vec3(1.0, 1.0, 1.0)),

            Vertex::new(vec3(-0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3( 0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5,  0.5,  0.5), vec3(1.0, 1.0, 1.0)),
            Vertex::new(vec3(-0.5,  0.5, -0.5), vec3(1.0, 1.0, 1.0)),
        ];

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

        Cube { vao, vbo, vertices }
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
