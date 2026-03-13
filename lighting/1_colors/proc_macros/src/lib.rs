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
