use syn;
use quote;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;


use glslang::*;
use utils::*;

#[derive(Debug)]
enum SourceKind {
    Src(String),
    Path(String),
}

pub fn impl_shader_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let declaration_type_name = &ast.ident;

    let sources = ast.attrs.iter()
        .filter_map(|attr| {
            match attr.value {
                syn::MetaItem::NameValue(ref i, syn::Lit::Str(ref val, _)) if i == "vert_src" => {
                    Some((ShaderType::VertexShader, SourceKind::Src(val.clone())))
                }

                syn::MetaItem::NameValue(ref i, syn::Lit::Str(ref val, _)) if i == "vert_path" => {
                    Some((ShaderType::VertexShader, SourceKind::Path(val.clone())))
                }

                syn::MetaItem::NameValue(ref i, syn::Lit::Str(ref val, _)) if i == "frag_src" => {
                    Some((ShaderType::FragmentShader, SourceKind::Src(val.clone())))
                }

                syn::MetaItem::NameValue(ref i, syn::Lit::Str(ref val, _)) if i == "frag_path" => {
                    Some((ShaderType::FragmentShader, SourceKind::Path(val.clone())))
                }

                _ => None
            }
        })
        .filter_map(|(sh_type, source)| {
            match source {
                SourceKind::Src(source) => Some((sh_type, source)),

                SourceKind::Path(path) => {
                    let root = env::var("CARGO_MANIFEST_DIR").expect("Environmant variable CARGO_MANIFEST_DIR is not set");
                    let full_path = Path::new(&root).join(&path);

                    if full_path.is_file() {
                        let mut buf = String::new();
                        File::open(full_path)
                            .and_then(|mut file| file.read_to_string(&mut buf))
                            .expect(&format!("Error reading source from {:?}", path));
                        Some((sh_type, buf))
                    } else {
                        panic!("File {:?} was not found. Path must be relative to your Cargo.toml", path);
                    }
                }
            }
        }).collect::<Vec<_>>();


    let attribute_type_name = format!("{}Attribute", declaration_type_name);
    let attribute_type_ident = syn::Ident::new(attribute_type_name);
    let uniform_type_name = format!("{}Uniform", declaration_type_name);
    let uniform_type_ident = syn::Ident::new(uniform_type_name);

    let (attributes, uniforms, sources) = prepocess_sources(declaration_type_name.to_string(), sources.iter());
    let sources = sources.iter().map(|shader| {
        let sh_type = shader.0.to_ident();
        let ref source = shader.1;
        Some(quote! { (_dragorust_render::ShaderType::#sh_type, #source) })
    }).collect::<Vec<_>>();
    let source_count = sources.len();


    let gen_attributes = impl_attribute_declaration(&attribute_type_ident, attributes);
    let gen_uniforms = impl_uniform_declaration(&uniform_type_ident, uniforms);

    let gen_shader_decl = quote! {
        impl ShaderDeclaration for #declaration_type_name {
            type Attributes = #attribute_type_ident;
            type Uniforms = #uniform_type_ident;

            #[allow(dead_code)]
            fn get_sources() -> slice::Iter<'static, (ShaderType, &'static str)> {
                static SOURCE : [(ShaderType,&str); #source_count] = #sources;
                SOURCE.iter()
            }
        }
    };

    //println!("{}", gen_uniforms);

    let dummy_mod = syn::Ident::new(format!("_IMPL_SHADERDECLARATION_FOR_{}", declaration_type_name));
    let gen = quote! {
        #[allow(unused_imports, non_snake_case)]
        pub mod #dummy_mod {
            extern crate dragorust_render as _dragorust_render;
            #gen_attributes
            #gen_uniforms
            #gen_shader_decl
        }
        pub use self::#dummy_mod::*;
    };

    //println!("{}", gen);

    gen
}


fn impl_attribute_declaration(attribute_type_ident: &syn::Ident, attributes: Vec<Attribute>) -> quote::Tokens {
    let mut attribute_fields = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();
    let mut visit_fields: Vec<quote::Tokens> = vec!();

    for (index, attr) in attributes.iter().enumerate() {
        let attr_name = attr.name.clone();
        let attr_field_ident = {
            let mut chars = attr_name.chars();
            if chars.next().unwrap() != 'v' || !chars.next().unwrap().is_uppercase() {
                panic!("Invalid attribute name: {}. 'v[CamelCase]' is required", attr_name);
            };
            syn::Ident::new(convert_camel_to_snake_case(attr_name.trim_left_matches("v")))
        };

        attribute_fields.push(quote! {
            #attr_field_ident : _dragorust_render::backend::VertexAttributeRefImpl
        });

        match_name_cases.push(
            quote! {
                #attr_name => Some(#index)
            }
        );


        visit_fields.push(
            quote! {
                visitor.process_attribute(#index, &self.#attr_field_ident);
            }
        );
    }

    let count = attributes.len();

    let gen = quote! {
        #[derive(Clone)]
        pub struct #attribute_type_ident {
            #(pub #attribute_fields,)*
        }

        impl _dragorust_render::ShaderAttribute for #attribute_type_ident {
            fn get_count() -> usize {
                #count
            }

            fn get_index_by_name(name: &str) -> Option<usize> {
                match name {
                    #(#match_name_cases,)*
                    _ => None
                }
            }

            fn visit<V: _dragorust_render::ShaderAttributeVisitor>(&self, visitor: &mut V) {
                #(#visit_fields)*
            }
        }
    };

    //println!("{}", gen);

    gen
}


fn impl_uniform_declaration(uniform_type_ident: &syn::Ident, uniforms: Vec<Uniform>) -> quote::Tokens {
    let mut uniform_fields = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();
    let mut visit_fields: Vec<quote::Tokens> = vec!();

    for (index, uniform) in uniforms.iter().enumerate() {
        let uniform_name = uniform.name.clone();
        let uniform_field_ident = {
            let mut chars = uniform_name.chars();
            if chars.next().unwrap() != 'u' || !chars.next().unwrap().is_uppercase() {
                panic!("Invalid uniform naming: {}. u[CamelCase] is required", uniform_name);
            };
            syn::Ident::new(convert_camel_to_snake_case(uniform_name.trim_left_matches("u")))
        };

        let type_token = uniform.get_stored_type_token().unwrap();
        let type_function_ident = syn::Ident::new(format!("process_{}", uniform.get_process_function_name().unwrap()));

        uniform_fields.push(quote! {
            #uniform_field_ident : #type_token
        });

        match_name_cases.push(
            quote! {
                #uniform_name => Some(#index)
            }
        );

        visit_fields.push(
            quote! {
                visitor.#type_function_ident(#index, &self.#uniform_field_ident);
            }
        );
    }

    let count = uniforms.len();

    let gen = quote! {
        #[derive(Clone)]
        pub struct #uniform_type_ident {
            #(pub #uniform_fields,)*
        }

        impl _dragorust_render::ShaderUniform for #uniform_type_ident {
            fn get_count() -> usize {
                #count
            }

            fn get_index_by_name(name: &str) -> Option<usize> {
                match name {
                    #(#match_name_cases,)*
                    _ => None
                }
            }

            fn visit<V: _dragorust_render::ShaderUniformVisitor>(&self, visitor: &mut V) {
                #(#visit_fields)*
            }
        }
    };

    //println!("{}", gen);

    gen
}