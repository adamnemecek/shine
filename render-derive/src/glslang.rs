use std::process::Command;
use std::str;
use std::env;
use std::path::Path;

const GLSL_VALIDATOR_EXECUTABLE: &'static str = "glslangValidator";
const GLSL_VALIDATOR_ARGS_TEST: [&'static str; 1] = ["-v"];
const GLSL_VALIDATOR_ARGS_INFO: [&'static str; 3] = ["-l", "-d", "-q"];
const GLSL_VALIDATOR_ARGS_PREPROCESS: [&'static str; 1] = ["-E"];


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ShaderType {
    VertexShader,
    FragmentShader,
}

impl ShaderType {
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


pub fn extract_shader_info<'a, I: Iterator<Item=&'a Path>>(shader_files: I) -> Result<(Vec<Attribute>, Vec<Uniform>), String> {
    let bin = get_glslangValidator_executable();

    let mut cmd = Command::new(bin);
    let cmd = cmd.args(&GLSL_VALIDATOR_ARGS_INFO).args(shader_files);
    let output = cmd.output().expect(&format!("Failed execute '{:?}' {:?} to extract info", cmd, GLSL_VALIDATOR_ARGS_INFO));
    //println!("shader info: {:?}", cmd);

    let stdout = str::from_utf8(&output.stdout).unwrap();
    //println!("shader info result: {}", stdout);

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


pub fn prepocess_shader_file(shader_file: &Path) -> Result<String, String> {
    let bin = get_glslangValidator_executable();

    let mut cmd = Command::new(bin.clone());
    let cmd = cmd.args(&GLSL_VALIDATOR_ARGS_PREPROCESS).arg(shader_file);
    let output = cmd.output().expect(&format!("Failed to execute '{:?}' {:?} to process shaders", cmd, GLSL_VALIDATOR_ARGS_INFO));
    //println!("shader source: {:?}", cmd);

    let stdout = str::from_utf8(&output.stdout).unwrap();
    //println!("shader source result: {}", stdout);

    Ok(stdout.to_string())
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
