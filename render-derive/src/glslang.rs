use std::process::Command;
use std::str;
use std::env;
use std::path::Path;
use std::fs;
use std::io::prelude::*;

use syn;
use quote;

const GLSL_VALIDATOR_EXECUTABLE: &'static str = "glslangValidator";
const GLSL_VALIDATOR_ARGS_TEST: [&'static str; 1] = ["-v"];
const GLSL_VALIDATOR_ARGS_INFO: [&'static str; 3] = ["-l", "-d", "-q"];
const GLSL_VALIDATOR_ARGS_PREPROCESS: [&'static str; 1] = ["-E"];

#[derive(Copy, Clone, Debug)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

impl ShaderType {
    pub fn to_ident(&self) -> syn::Ident {
        match self {
            &ShaderType::VertexShader => syn::Ident::new("VertexShader"),
            &ShaderType::FragmentShader => syn::Ident::new("FragmentShader"),
        }
    }

    pub fn to_extension(&self) -> &str {
        match self {
            &ShaderType::VertexShader => ".vert",
            &ShaderType::FragmentShader => ".frag",
        }
    }
}

#[derive(Debug)]
pub struct Uniform {
    pub name: String,
    pub type_id: u32,
    pub size: u32,
}

impl Uniform {
    pub fn get_parameter_type_token(&self) -> Result<quote::Tokens, String> {
        let type_id =
            match self.type_id {
                5126 => quote!(f32), // GLenum(GL_FLOAT)
                35664 => quote!(_dragorust_render_core::Float32x2), // GLenum(GL_FLOAT_VEC2)
                35665 => quote!(_dragorust_render_core::Float32x3),// GLenum(GL_FLOAT_VEC3)
                35666 => quote!(_dragorust_render_core::Float32x4),// GLenum(GL_FLOAT_VEC4)

                5124 => quote!(i32),// GLenum(GL_INT)
                35667 => quote!(_dragorust_render_core::Int32x2),// GLenum(GL_INT_VEC2)
                35668 => quote!(_dragorust_render_core::Int32x3),// GLenum(GL_INT_VEC3)
                35669 => quote!(_dragorust_render_core::Int32x4),// GLenum(GL_INT_VEC4)

                35670 => quote!(bool),// GLenum(GL_BOOL)
                35671 => quote!(_dragorust_render_core::Boolx2),// GLenum(GL_BOOL_VEC2)
                35672 => quote!(_dragorust_render_core::Boolx3),// GLenum(GL_BOOL_VEC3)
                35673 => quote!(_dragorust_render_core::Boolx4),// GLenum(GL_BOOL_VEC4)

                35674 => quote!(_dragorust_render_core::Float32x4),// GLenum(GL_FLOAT_MAT2)
                35675 => quote!(_dragorust_render_core::Float32x9),// GLenum(GL_FLOAT_MAT3)
                35676 => quote!(_dragorust_render_core::Float32x16),// GLenum(GL_FLOAT_MAT4)

                35678 => quote!(_dragorust_render_gl::UnsafeTexture2DIndex),// GLenum(GL_SAMPLER_2D)
                35680 => quote!(_dragorust_render_gl::UnsafeTextureCubeIndex),// GLenum(GL_SAMPLER_CUBE)

                _ => return Err(format!("Could not find built-in type for uniform {}, type id:{}", self.name, self.type_id))
            };

        let size = self.size;
        if size > 1 {
            Ok(quote!([#type_id; #size]))
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
pub struct Attribute {
    pub name: String,
}

fn parse_uniform(line: &str) -> Uniform {
    let name = line.split(":").nth(0).unwrap();

    let type_id = line.split("type").nth(1).unwrap().split(" ").nth(1).unwrap().trim_right_matches(",");
    let type_id = u32::from_str_radix(&type_id, 16).unwrap();

    let size = line.split("size").nth(1).unwrap().split(" ").nth(1).unwrap().trim_right_matches(",");
    let size = size.parse::<u32>().unwrap();

    Uniform {
        name: name.to_string(),
        type_id: type_id,
        size: size,
    }
}


fn parse_attribute(line: &str) -> Attribute {
    let name = line.split(":").nth(0).unwrap();

    Attribute {
        name: name.to_string(),
    }
}

#[allow(non_snake_case)]
fn get_glslangValidator_executable() -> String {
    use std::path::*;
    let bin_path =
        Err(env::VarError::NotPresent)
            .or(
                env::var("CARGO_MANIFEST_DIR").and_then(|p| {
                    let path = Path::new(&p).join("tools").join(GLSL_VALIDATOR_EXECUTABLE);
                    if Command::new(path.clone()).args(&GLSL_VALIDATOR_ARGS_TEST).output().is_ok() { Ok(path) } else { Err(env::VarError::NotPresent) }
                }))
            .or(
                env::var("CARGO_MANIFEST_DIR").and_then(|p| {
                    let path = Path::new(&p).join("..").join("tools").join(GLSL_VALIDATOR_EXECUTABLE);
                    if Command::new(path.clone()).args(&GLSL_VALIDATOR_ARGS_TEST).output().is_ok() { Ok(path) } else { Err(env::VarError::NotPresent) }
                }))
            .or(
                env::var("GLSLANG_ROOT").and_then(|p| {
                    let path = Path::new(&p).join("tools").join(GLSL_VALIDATOR_EXECUTABLE);
                    if Command::new(path.clone()).args(&GLSL_VALIDATOR_ARGS_TEST).output().is_ok() { Ok(path) } else { Err(env::VarError::NotPresent) }
                }))
            .or(
                env::var("GLSLANG_BIN").and_then(|p| {
                    let path = PathBuf::from(&p);
                    if Command::new(path.clone()).args(&GLSL_VALIDATOR_ARGS_TEST).output().is_ok() { Ok(path) } else { Err(env::VarError::NotPresent) }
                }));

    if bin_path.is_err() {
        panic!("glslang was not found, try setting GLSLANG_ROOT or GLSLANG_BIN")
    }
    bin_path.unwrap().to_str().unwrap().to_string()
}


fn extract_shader_info(shaders: &[String]) -> Result<(Vec<Attribute>, Vec<Uniform>), String> {
    let bin = get_glslangValidator_executable();

    let mut cmd = Command::new(bin);
    cmd.args(shaders).args(&GLSL_VALIDATOR_ARGS_INFO);
    let output = cmd.output().expect(&format!("Failed execute '{:?}' {:?} to extract info", cmd, GLSL_VALIDATOR_ARGS_INFO));

    let stdout = str::from_utf8(&output.stdout).unwrap();

    enum State {
        Attribute,
        Uniform,
        Ignore,
    }

    let mut state = State::Ignore;
    let mut uniforms: Vec<Uniform> = vec!();
    let mut attributes: Vec<Attribute> = vec!();

    for line in stdout.lines() {
        match state {
            State::Ignore => {
                if line.starts_with("ERROR:") {
                    return Err(format!("{}", stdout));
                }

                state = match line {
                    "Uniform reflection:" => State::Uniform,
                    "Vertex attribute reflection:" => State::Attribute,
                    _ => State::Ignore,
                };
            }

            State::Uniform => {
                if line.is_empty() {
                    state = State::Ignore;
                } else {
                    let uni = parse_uniform(line);
                    uniforms.push(uni);
                }
            }

            State::Attribute => {
                if line.is_empty() {
                    state = State::Ignore;
                } else {
                    let attr = parse_attribute(line);
                    attributes.push(attr);
                }
            }
        }
    }

    Ok((attributes, uniforms))
}


fn prepocess_shader(shaders: &[String]) -> Vec<(ShaderType, String)> {
    let bin = get_glslangValidator_executable();
    let mut res = vec!();

    for sh_type in vec!(ShaderType::VertexShader, ShaderType::FragmentShader).into_iter() {
        let shaders: Vec<_> = shaders.iter().filter(|e| e.ends_with(sh_type.to_extension())).collect();
        let output =
            Command::new(bin.clone())
                .args(&shaders)
                .args(&GLSL_VALIDATOR_ARGS_PREPROCESS)
                .output()
                .expect(&format!("Failed execute {} {:?} to prerocess shader", bin, GLSL_VALIDATOR_ARGS_PREPROCESS));

        let stdout = str::from_utf8(&output.stdout).unwrap();
        res.push((sh_type, stdout.to_string()))
    }

    res
}

pub fn prepocess_sources<'a, I: Iterator<Item=&'a (ShaderType, String)>>(name: String, shaders: I) -> Result<(Vec<Attribute>, Vec<Uniform>, Vec<(ShaderType, String)>), String> {
    // create temp files froom the shader

    let root = env::var("CARGO_MANIFEST_DIR").expect("Environmant variable CARGO_MANIFEST_DIR is not set");
    let root = Path::new(&root).join("target").join("shaders").join(name.to_string());

    let mut source_files: Vec<String> = vec!();
    fs::DirBuilder::new().recursive(true).create(root.clone()).expect("Temporary target directory could not be created:");

    for (idx, ref shader) in shaders.enumerate() {
        let file = root.join(format!("{}{}", idx, shader.0.to_extension()));
        source_files.push(file.to_str().unwrap().to_string());
        let mut file = fs::File::create(file).expect("Temporary shader files could not be created:");
        file.write_all(shader.1.as_bytes()).expect("Temporary shader files could not be written:");
    }

    let (attributes, uniforms) = extract_shader_info(&source_files)?;
    let sources = prepocess_shader(&source_files);

    //println!("{:?},{:?}", attributes, uniforms);

    Ok((attributes, uniforms, sources))
}

/*
fn main() {
	let shaders = ["a.vert", "b.vert", "a.frag"];
	
	let (a,u) = extract_shader_info(&shaders);
	let ps = prepocess_shader(&shaders);
	
	println!("***************** vert *****************\n{}\n----------------------------------------", ps[0]);
	println!("***************** frag *****************\n{}\n----------------------------------------", ps[1]);
	println!("attributes: {:?}", a);
	println!("uniforms: {:?}", u);
	
}
*/