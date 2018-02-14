use syn;
use quote;
use std::path::*;
use std::fs::*;
use std::env;
use proc_macro;
use proc_macro2::Span;
use glslang::*;
use utils::*;


impl ShaderType {
    fn to_tokens(&self) -> quote::Tokens {
        match self {
            &ShaderType::VertexShader => quote_call_site! {ShaderType::VertexShader},
            &ShaderType::FragmentShader => quote_call_site! {ShaderType::FragmentShader},
        }
    }
}


impl Uniform {
    pub fn get_parameter_type_token(&self) -> Result<quote::Tokens, String> {
        let type_id =
            match self.type_id {
                5126 => quote_call_site!(f32), // GLenum(GL_FLOAT)
                35664 => quote_call_site!(_shine_render_core::Float32x2), // GLenum(GL_FLOAT_VEC2)
                35665 => quote_call_site!(_shine_render_core::Float32x3),// GLenum(GL_FLOAT_VEC3)
                35666 => quote_call_site!(_shine_render_core::Float32x4),// GLenum(GL_FLOAT_VEC4)

                5124 => quote_call_site!(i32),// GLenum(GL_INT)
                35667 => quote_call_site!(_shine_render_core::Int32x2),// GLenum(GL_INT_VEC2)
                35668 => quote_call_site!(_shine_render_core::Int32x3),// GLenum(GL_INT_VEC3)
                35669 => quote_call_site!(_shine_render_core::Int32x4),// GLenum(GL_INT_VEC4)

                35670 => quote_call_site!(bool),// GLenum(GL_BOOL)
                35671 => quote_call_site!(_shine_render_core::Boolx2),// GLenum(GL_BOOL_VEC2)
                35672 => quote_call_site!(_shine_render_core::Boolx3),// GLenum(GL_BOOL_VEC3)
                35673 => quote_call_site!(_shine_render_core::Boolx4),// GLenum(GL_BOOL_VEC4)

                35674 => quote_call_site!(_shine_render_core::Float32x4),// GLenum(GL_FLOAT_MAT2)
                35675 => quote_call_site!(_shine_render_core::Float32x9),// GLenum(GL_FLOAT_MAT3)
                35676 => quote_call_site!(_shine_render_core::Float32x16),// GLenum(GL_FLOAT_MAT4)

                35678 => quote_call_site!(_shine_render_gl::UnsafeTexture2DIndex),// GLenum(GL_SAMPLER_2D)
                35680 => quote_call_site!(_shines_render_gl::UnsafeTextureCubeIndex),// GLenum(GL_SAMPLER_CUBE)

                _ => return Err(format!("Could not find built-in type for uniform {}, type id:{}", self.name, self.type_id))
            };

        let size = self.size;
        if size > 1 {
            Ok(quote_call_site!([#type_id; #size]))
        } else {
            Ok(type_id)
        }
    }

    pub fn get_function_postfix(&self) -> Result<String, String> {
        let type_id =
            match self.type_id {
                5126 => "f32", // GLenum(GL_FLOAT)
                35664 => "f32x2", // GLenum(GL_FLOAT_VEC2)
                35665 => "f32x3",// GLenum(GL_FLOAT_VEC3)
                35666 => "f32x4",// GLenum(GL_FLOAT_VEC4)

                5124 => "i32",// GLenum(GL_INT)
                35667 => "i32x2",// GLenum(GL_INT_VEC2)
                35668 => "i32x3",// GLenum(GL_INT_VEC3)
                35669 => "i32x4",// GLenum(GL_INT_VEC4)

                35670 => "bool",// GLenum(GL_BOOL)
                35671 => "boolx2",// GLenum(GL_BOOL_VEC2)
                35672 => "boolx3",// GLenum(GL_BOOL_VEC3)
                35673 => "boolx4",// GLenum(GL_BOOL_VEC4)

                35674 => "f32x4",// GLenum(GL_FLOAT_MAT2)
                35675 => "f32x9",// GLenum(GL_FLOAT_MAT3)
                35676 => "f32x16",// GLenum(GL_FLOAT_MAT4)

                35678 => "texture_2d",// GLenum(GL_SAMPLER_2D)
                35680 => "texture_cube",// GLenum(GL_SAMPLER_CUBE)

                _ => return Err(format!("Could not find built-in type for uniform {}, type id:{}", self.name, self.type_id))
            };

        let size = self.size;
        if size > 1 {
            Ok(format!("{}_{}", type_id, size))
        } else {
            Ok(type_id.to_string())
        }
    }
}


#[derive(Debug)]
enum SourceKind {
    Inline,
    External(String),
}


#[derive(Debug)]
struct Source {
    sh_type: ShaderType,
    source_kind: SourceKind,
    source: String,
    temp_file: PathBuf,
}


fn read_file_as_string(path: &Path) -> String {
    use std::io::Read;

    if !path.is_file() {
        panic!("File {:?} was not found. Path must be relative to the source file of calling site.", path);
    }

    let mut buf = String::new();
    File::open(path)
        .and_then(|mut file| file.read_to_string(&mut buf))
        .expect(&format!("Error reading source from {:?}", path));
    buf
}


fn write_string_to_file(path: &Path, string: &str) {
    use std::io::prelude::*;

    File::create(path)
        .and_then(|mut file| file.write_all(string.as_bytes()))
        .expect(&format!("Error creatinh temp file {:?}", path));
}


fn parse_source_inputs(attrs: &Vec<syn::Attribute>, src_path: &Path, out_path: &Path) -> Vec<Source>
{
    attrs.iter().enumerate()
        .filter_map(|(id, attr)| {
            if let Some(syn::Meta::NameValue(syn::MetaNameValue { ref ident, lit: syn::Lit::Str(ref value), .. })) = attr.interpret_meta() {
                match ident.to_string().as_ref() {
                    "vert_src" => Some(Source {
                        sh_type: ShaderType::VertexShader,
                        source_kind: SourceKind::Inline,
                        source: value.value(),
                        temp_file: out_path.join(format!("{}.vert", id)),
                    }),
                    "vert_path" => Some(Source {
                        sh_type: ShaderType::VertexShader,
                        source_kind: SourceKind::External(value.value()),
                        source: read_file_as_string(&src_path.join(value.value())),
                        temp_file: out_path.join(format!("{}.vert", id)),
                    }),
                    "frag_src" => Some(Source {
                        sh_type: ShaderType::FragmentShader,
                        source_kind: SourceKind::Inline,
                        source: value.value(),
                        temp_file: out_path.join(format!("{}.frag", id)),
                    }),
                    "frag_path" => Some(Source {
                        sh_type: ShaderType::FragmentShader,
                        source_kind: SourceKind::External(value.value()),
                        source: read_file_as_string(&src_path.join(value.value())),
                        temp_file: out_path.join(format!("{}.frag", id)),
                    }),
                    _ => None,
                }
            } else {
                None
            }
        }).collect::<Vec<_>>()
}


fn impl_parameter_declaration(param_type_ident: &syn::Ident, attributes: Vec<Attribute>, uniforms: Vec<Uniform>) -> quote::Tokens {
    let mut param_fields = vec!();
    let mut match_name_cases: Vec<quote::Tokens> = vec!();
    let mut bind_fields: Vec<quote::Tokens> = vec!();

    let mut index: usize = 0;

    // vertex buffers
    {
        for attr in attributes.iter() {
            let attr_name = attr.name.clone();
            let attr_field_ident = {
                let mut chars = attr_name.chars();
                if chars.next().unwrap() != 'v' || !chars.next().unwrap().is_uppercase() {
                    panic!("Invalid attribute name: {}. 'v[CamelCase]' is required", attr_name);
                };
                syn::Ident::new(&format!("v_{}", convert_camel_to_snake_case(attr_name.trim_left_matches("v"))), Span::call_site())
            };

            param_fields.push(quote_call_site! {
                #attr_field_ident: _shine_render_gl::UnsafeVertexAttributeIndex
            });

            match_name_cases.push(quote_call_site! {
                #attr_name => Some(#index)
            });

            bind_fields.push(quote_call_site! {
                {
                    let target = unsafe { context.vertex_store.at_unsafe_mut(&self.#attr_field_ident.0) };
                    locations[#index].set_attribute(context.ll, &target, self.#attr_field_ident.1);
                }
            });

            index += 1;
        }
    }

    // index buffers
    {
        param_fields.push(quote_call_site! {
            indices: _shine_render_gl::UnsafeIndexBufferIndex
        });

        match_name_cases.push(quote_call_site! {
            "indices" => Some(#index)
        });

        bind_fields.push(quote_call_site! {
            {
                let target = unsafe { context.index_store.at_unsafe_mut(&self.indices) };
                locations[#index].set_index(context.ll, &target);
            }
        });
        index += 1;
    }

    // uniforms
    {
        for uniform in uniforms.iter() {
            let uniform_name = uniform.name.clone();
            let uniform_field_ident = {
                let mut chars = uniform_name.chars();
                if chars.next().unwrap() != 'u' || !chars.next().unwrap().is_uppercase() {
                    panic!("Invalid uniform naming: {}. u[CamelCase] is required", uniform_name);
                };
                syn::Ident::new(&format!("u_{}", convert_camel_to_snake_case(uniform_name.trim_left_matches("u"))), Span::call_site())
            };

            let type_token = uniform.get_parameter_type_token().unwrap();
            let bind_postfix = uniform.get_function_postfix().unwrap();
            let bind_function_ident = syn::Ident::new(&format!("set_{}", bind_postfix), Span::call_site());

            param_fields.push(quote_call_site! {
                #uniform_field_ident: #type_token
            });

            match_name_cases.push(quote_call_site! {
                #uniform_name => Some(#index)
            });

            bind_fields.push(
                if bind_postfix == "texture_2d" {
                    quote_call_site! {
                        {
                            let target = unsafe { context.texture_2d_store.at_unsafe_mut(&self.#uniform_field_ident) };
                            locations[#index].set_texture_2d(context.ll, &target);
                        }
                    }
                } else {
                    quote_call_site! {
                       locations[#index].#bind_function_ident(context.ll, &self.#uniform_field_ident);
                    }
                }
            );

            index += 1;
        }
    }

    let count = index;

    let gen_index_by_name = quote_call_site! {
        fn get_index_by_name(name: &str) -> Option<usize> {
            match name {
                    #(#match_name_cases,)*
                _ => None
            }
        }
    };

    let gen_bind = quote_call_site! {
        fn bind(&self, context: &mut _shine_render_gl::GLCommandProcessContext) {
            let params = context.ll.program_binding.get_parameters();
            assert!(params.is_some(), "missing program parameters");
            if let Some(locations) = params {
                let locations = &mut *locations.borrow_mut();
                #(#bind_fields;)*
            }
        }
    };

    quote_call_site! {
        #[derive(Clone)]
        pub struct #param_type_ident {
            #(pub #param_fields,)*
        }

        #[allow(unused_variables)]
        impl _shine_render_core::ShaderParameters<_shine_render_gl::PlatformEngine> for #param_type_ident {
            fn get_count() -> usize {
                #count
            }
            #gen_index_by_name
            #gen_bind
        }
    }
}


pub fn impl_shader_declaration(ast: &syn::DeriveInput) -> quote::Tokens {
    let declaration_ident = &ast.ident;
    let declaration_name = declaration_ident.to_string();

    let out_root = {
        use std::fs;
        let out_root = env::var("CARGO_MANIFEST_DIR").expect("Environmant variable CARGO_MANIFEST_DIR is not set");
        let out_root = Path::new(&out_root).join("target").join("shaders").join(declaration_name.to_string());
        fs::DirBuilder::new().recursive(true).create(out_root.clone()).expect("Temporary target directory could not be created:");
        out_root
    };

    let src_root = {
        let source_file = proc_macro::Span::call_site().source_file();
        if !source_file.is_real() {
            panic!("This derive macro can be used from real files only, no macro or other dark magic, sry.");
        }

        let source_file = PathBuf::from(format!("{}", source_file.path()));
        if !source_file.exists() {
            panic!("Could not find source path: {:?}", source_file);
        }
        PathBuf::from(source_file.as_path().parent().unwrap())
    };

    let sources = parse_source_inputs(&ast.attrs, &src_root, &out_root);
    //println!("sources: {:?}", sources);

    let external_source_files = sources.iter().filter_map(|src| {
        if let SourceKind::External(ref path) = src.source_kind {
            Some(quote_call_site! { include_str!(#path) })
        } else {
            None
        }
    }).collect::<Vec<_>>();
    let external_source_count = external_source_files.len();
    //println!("external_source_files: {:?}", external_source_files);

    let preprocessed_sources = sources.iter().enumerate().filter_map(|(id, src)| {
        write_string_to_file(&src.temp_file, &src.source);
        let sh_src = prepocess_shader_file(&src.temp_file).unwrap();
        let sh_type = src.sh_type.to_tokens();
        Some(quote_call_site! { (#sh_type, #sh_src) })
    }).collect::<Vec<_>>();
    let preprocessed_source_count = preprocessed_sources.len();
    //println!("processed_sources: {:?}", preprocessed_sources);

    let parameters_name = format!("{}Parameters", declaration_name);
    let parameters_ident = syn::Ident::new(&parameters_name, Span::call_site());

    let gen_parameters = {
        let source_files = sources.iter().filter_map(|src| Some(src.temp_file.as_path()));
        let (attributes, uniforms) = extract_shader_info(source_files).unwrap();
        //println!("attributes: {:?}", attributes);
        //println!("uniforms: {:?}", uniforms);
        impl_parameter_declaration(&parameters_ident, attributes, uniforms)
    };

    let gen_shader_decl = quote_call_site! {
        impl ShaderDeclaration<_shine_render_gl::PlatformEngine> for #declaration_ident {
            type Parameters = #parameters_ident;

            #[allow(dead_code)]
            fn source_iter() -> slice::Iter<'static, (ShaderType, &'static str)> {
                // workaround to make the compilation depend on the input files
                const _EXTERNAL_SOURCES : [&str; #external_source_count] = [#(#external_source_files,)*];
                const SOURCES : [(ShaderType,&str); #preprocessed_source_count] = [#(#preprocessed_sources,)*];
                SOURCES.iter()
            }
        }
    };

    let dummy_mod = syn::Ident::new(&format!("_IMPL_SHADERDECLARATION_FOR_{}", declaration_name), Span::call_site());
    quote_call_site! {
        #[allow(unused_imports, non_snake_case)]
        pub mod #dummy_mod {
            extern crate shine_render_core as _shine_render_core;
            extern crate shine_render_gl as _shine_render_gl;
            use std::slice;
            #gen_parameters
            #gen_shader_decl
        }
        pub use self::#dummy_mod::*;
    }
}
