use std::process::Command;
use std::str;
use std::env;
use std::path::Path;
use std::fs;
use std::io::prelude::*;

use syn;

static GLSL_VALIDATOR_EXECUTABLE: &'static str = "glslangValidator";
static GLSL_VALIDATOR_ARGS_INFO: [&'static str; 3] = ["-l", "-d", "-q"];
static GLSL_VALIDATOR_ARGS_PREPROCESS: [&'static str; 1] = ["-E"];

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
    name: String,
    type_id: u32,
    size: u32,
}

#[derive(Debug)]
pub struct Attribute {
    name: String,
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
        size: size
    }
}


fn parse_attribute(line: &str) -> Attribute {
    let name = line.split(":").nth(0).unwrap();

    Attribute {
        name: name.to_string(),
    }
}

fn get_glslangValidator_executable() -> String {
    let root = env::var("CARGO_MANIFEST_DIR").expect("Environmant variable CARGO_MANIFEST_DIR is not set");
    let root = Path::new(&root).join("tools").join(GLSL_VALIDATOR_EXECUTABLE);
    root.to_str().unwrap().to_string()
}


fn extract_shader_info(shaders: &[String]) -> (Vec<Attribute>, Vec<Uniform>) {
    let bin = get_glslangValidator_executable();

    let output =
        Command::new(bin.clone())
            .args(shaders)
            .args(&GLSL_VALIDATOR_ARGS_INFO)
            .output()
            .expect(&format!("Failed execute {} {:?} to extract info", bin, GLSL_VALIDATOR_ARGS_INFO));

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

    (attributes, uniforms)
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

pub fn prepocess_sources<'a, I: Iterator<Item=&'a (ShaderType, String)>>(name: String, shaders: I) -> (Vec<Attribute>, Vec<Uniform>, Vec<(ShaderType, String)>) {
    // create temp files froom the shader

    let sources: Vec<String> = vec!();
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

    let (attributes, uniforms) = extract_shader_info(&source_files);
    let sources = prepocess_shader(&source_files);

    println!("{:?},{:?}", attributes, uniforms);

    (attributes, uniforms, sources)
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