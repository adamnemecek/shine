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


    let attribute_type_name = format!("{}Attribute", declaration_type_name);
    let attribute_type_ident = syn::Ident::new(attribute_type_name);
    let uniform_type_name = format!("{}Uniform", declaration_type_name);
    let uniform_type_ident = syn::Ident::new(uniform_type_name);

    let (attributes, uniforms, sources) = prepocess_sources(declaration_type_name.to_string(), sources.iter());
    let sources = sources.iter().map(|shader| {
        let sh_type = shader.0.to_ident();
        let ref source = shader.1;
        Some(quote! { (::dragorust_engine::render::ShaderType::#sh_type, #source) })
    }).collect::<Vec<_>>();
    let source_count = sources.len();


    let (gen_attributes, qualified_attribute_idents) = impl_attribute_declaration(&attribute_type_ident, attributes);
    let attribute_count = qualified_attribute_idents.len();

    let gen_uniforms = impl_uniform_declaration(&uniform_type_ident, uniforms);
    //let uniform_count = qualified_attribute_idents.len();

    let gen_decl = quote! {
        impl ShaderDeclaration for #declaration_type_name {
            type Attribute = #attribute_type_ident;
            //type Uniform = #uniform_type_ident;

            #[allow(dead_code)]
            fn get_sources() -> slice::Iter<'static, (ShaderType, &'static str)> {
                static SOURCE : [(ShaderType,&str); #source_count] = #sources;
                SOURCE.iter()
            }

            #[allow(dead_code)]
            fn get_attributes() -> slice::Iter<'static, #attribute_type_ident> {
                static IDS : [#attribute_type_ident; #attribute_count] = #qualified_attribute_idents;
                IDS.iter()
            }
        }
    };

    println!("{}", gen_decl);

    quote! {
        #gen_decl
        #gen_attributes
        //#gen_uniforms
    }
}


fn impl_attribute_declaration(attribute_type_ident: &syn::Ident, attributes: Vec<Attribute>) -> (quote::Tokens, Vec<quote::Tokens>) {
    let mut attribute_idents = vec!();
    let mut qualified_attribute_idents = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();
    let mut match_from_usize_cases: Vec<quote::Tokens> = vec!();
    let mut match_to_usize_cases: Vec<quote::Tokens> = vec!();

    let count = attributes.len();

    for (index, attr) in attributes.iter().enumerate() {
        let attr_enum_name = attr.name.clone();
        let attr_enum_ident = {
            let mut chars = attr_enum_name.chars();
            if chars.next().unwrap() != 'v' || !chars.next().unwrap().is_uppercase() {
                panic!("Invalid attribute naming: {}. v[CamelCase] is required", attr_enum_name);
            };
            syn::Ident::new(attr_enum_name.trim_left_matches("v"))
        };

        attribute_idents.push(quote! {
            #attr_enum_ident
        });

        qualified_attribute_idents.push(quote! {
            #attribute_type_ident::#attr_enum_ident
        });

        match_name_cases.push(
            quote! {
                #attr_enum_name => Ok(#attribute_type_ident::#attr_enum_ident)
            }
        );

        match_from_usize_cases.push(
            quote! {
                #index => #attribute_type_ident::#attr_enum_ident
            }
        );

        match_to_usize_cases.push(
            quote! {
                #attribute_type_ident::#attr_enum_ident => #index
            }
        );
    }

    let gen_enum = quote! {
        #[derive(Copy, Clone, Debug)]
        #[repr(u8)]
        enum #attribute_type_ident {
            #(#attribute_idents,)*
        }
    };

    let gen_from_usize = quote! {
        impl From<usize> for #attribute_type_ident {
            fn from(index: usize) -> #attribute_type_ident {
                match index {
                    #(#match_from_usize_cases,)*
                    _ => panic!("Index ({}) out of range (0..{})", index, #count)
                }
            }
        }

        impl Into<usize> for #attribute_type_ident {
            fn into(self) -> usize {
                match self {
                    #(#match_to_usize_cases,)*
                }
            }
        }
    };

    let gen_from_str = quote! {
        impl str::FromStr for #attribute_type_ident {
            type Err = String;

            fn from_str(name: &str) -> Result<#attribute_type_ident, String> {
                match name {
                    #(#match_name_cases,)*
                    _ => Err(format!("Attribute not found by '{}' name", name).to_string())
                }
            }
        }
    };

    let gen = quote! {
        #gen_enum
        #gen_from_usize
        #gen_from_str
    };

    (gen, qualified_attribute_idents)
}


fn impl_uniform_declaration(uniform_type_ident: &syn::Ident, uniforms: Vec<Uniform>) -> quote::Tokens {
    let mut uniform_enums = vec!();

    for uniform in uniforms.iter() {
        let uniform_enum_name = uniform.name.clone();
        let uniform_enum_ident = {
            let mut chars = uniform_enum_name.chars();
            if chars.next().unwrap() != 'u' || !chars.next().unwrap().is_uppercase() {
                panic!("Invalid uniform naming: {}. u[CamelCase] is required", uniform_enum_name);
            };
            syn::Ident::new(uniform_enum_name.trim_left_matches("u"))
        };

        let type_token = uniform.get_type_token().unwrap();

        uniform_enums.push(quote! {
            #uniform_enum_ident(#type_token)
        });
    }

    let gen_enum = quote! {
        #[derive(Copy, Clone, Debug)]
        #[repr(usize)]
        enum #uniform_type_ident {
            #(#uniform_enums,)*
            None
        }
    };

    quote! {
        #gen_enum
    }
}