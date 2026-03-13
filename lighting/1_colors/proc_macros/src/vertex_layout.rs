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
