use std::{ffi::c_void, ptr::null};

use gl::types::GLsizei;
use glm::Vec3;

pub trait Model {
    fn draw(&self);
}

pub struct Cube {
    vao: u32,
    vbo: u32,

    vertices: Vec<f32>,
}

impl Cube {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Cube {
    fn default() -> Self {
        #[rustfmt::skip]
        let vertices = &[
            // Positions       // Texture coords
            -0.5, -0.5, -0.5,  0.0, 0.0,
             0.5, -0.5, -0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5,  0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 0.0,

            -0.5, -0.5,  0.5,  0.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 1.0,
            -0.5,  0.5,  0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,

            -0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5, -0.5,  1.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5,  0.5,  1.0, 0.0,

             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5,  0.5,  0.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,

            -0.5, -0.5, -0.5,  0.0, 1.0,
             0.5, -0.5, -0.5,  1.0, 1.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
             0.5, -0.5,  0.5,  1.0, 0.0,
            -0.5, -0.5,  0.5,  0.0, 0.0,
            -0.5, -0.5, -0.5,  0.0, 1.0,

            -0.5,  0.5, -0.5,  0.0, 1.0,
             0.5,  0.5, -0.5,  1.0, 1.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
             0.5,  0.5,  0.5,  1.0, 0.0,
            -0.5,  0.5,  0.5,  0.0, 0.0,
            -0.5,  0.5, -0.5,  0.0, 1.0
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
                size_of_val(vertices) as isize,
                vertices.as_ptr() as *const c_void,
                gl::STATIC_DRAW,
            );

            // Copy the vertex indices into the ebo
            // gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            // gl::BufferData(
            //     gl::ELEMENT_ARRAY_BUFFER,
            //     size_of_val(indices) as isize,
            //     indices.as_ptr() as *const c_void,
            //     gl::STATIC_DRAW,
            // );

            // Position attributes ----------------------------
            gl::VertexAttribPointer(
                0,                                     // Location = 0 in vertex shader
                3,                                     // Size of data (3 floats = vec3)
                gl::FLOAT,                             // Data type
                gl::FALSE,                             // Normalization
                (3 + 2) * size_of::<f32>() as GLsizei, // Space between 2 consecutive attributes
                null(),                                // Offset
            );

            gl::EnableVertexAttribArray(0);
            // Position attributes ----------------------------

            // Texture coords attributes ----------------------
            gl::VertexAttribPointer(
                2,                                       // Location = 1 in vertex shader
                2,                                       // Size of data (2 floats = vec2)
                gl::FLOAT,                               // Data type
                gl::FALSE,                               // Normalization
                (3 + 2) * size_of::<f32>() as GLsizei,   // Space between 2 consecutive attributes
                (3 * size_of::<f32>()) as *const c_void, // Offset (after position)
            );

            gl::EnableVertexAttribArray(2);
            // Texture coords attributes ----------------------

            // Unbind vao
            gl::BindVertexArray(0);
            // Unbind the vbo since it was bound to the vertex attribute pointer
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            // EBO should not be unbound since it is stored in the VAO
        };

        Cube {
            vao,
            vbo,

            vertices: vertices.to_vec(),
        }
    }
}

impl Model for Cube {
    fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.vertices.len() as i32 / 5);
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
