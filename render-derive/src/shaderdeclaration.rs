use syn;
use quote;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::Read;

use glslang::*;

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


    let (attributes, uniforms, sources) = prepocess_sources(declaration_type_name.to_string(), sources.iter());

    let sources = sources.iter().map(|shader| {
        let sh_type = shader.0.to_ident();
        let ref source = shader.1;
        Some(quote! { (::dragorust_engine::render::ShaderType::#sh_type, #source) })
    }).collect::<Vec<_>>();

    let attribute_type_name = syn::Ident::new(format!("{}Attribute", declaration_type_name));
    let uniform_type_name = syn::Ident::new(format!("{}Uniform", declaration_type_name));

    let mut attribute_enums = vec!();
    for attr in attributes.iter() {
        let attr_name = attr.name.clone();
        let id = {
            let mut chars = attr_name.chars();
            if chars.next().unwrap() != 'v' || !chars.next().unwrap().is_uppercase() {
                panic!("Invalid attribute naming: {}. v[CamelCase] is required", attr_name);
            };
            syn::Ident::new(attr_name.trim_left_matches("v"))
        };

        attribute_enums.push(quote! {
            #[name = #attr_name]
            #id
        });
    }

    let mut uniform_enums = vec!();
    for uniform in uniforms.iter() {
        let uniform_name = uniform.name.clone();
        let id = {
            let mut chars = uniform_name.chars();
            if chars.next().unwrap() != 'u' || !chars.next().unwrap().is_uppercase() {
                panic!("Invalid uniform naming: {}. u[CamelCase] is required", uniform_name);
            };
            syn::Ident::new(uniform_name.trim_left_matches("u"))
        };

        let type_token = uniform.get_type_token().unwrap();

        uniform_enums.push(quote! {
            #[name = #uniform_name]
            #id(#type_token)
        });
    }

    let gen = quote! {
        #[derive(Copy, Clone, Debug)]
        #[derive(IterableEnum)]
        #[repr(usize)]
        enum #attribute_type_name {
            #(#attribute_enums,)*
        }

        #[derive(Copy, Clone, Debug)]
        #[derive(IterableEnum)]
        #[repr(usize)]
        enum #uniform_type_name {
            #(#uniform_enums,)*
        }

        impl ShaderDeclaration for #declaration_type_name {
            type Attribute = #attribute_type_name;
            type Uniform = #uniform_type_name;

            fn map_sources<F: FnMut((ShaderType, &str)) -> bool>(mut f: F) -> bool {
                let sh_source = #sources;

                for src in sh_source.iter() {
                    if !f(*src) {
                        return false
                    }
                }
                true
            }
        }
    };

    //println!("{:?}", gen.to_string());

    gen
}