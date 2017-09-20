use std::process::Command;
use std::str;

static GLSL_VALIDATOR_EXECUTABLE: &'static str = "glslangValidator";
static GLSL_VALIDATOR_ARGS_INFO: [&'static str; 3] = ["-l", "-d", "-q"];
static GLSL_VALIDATOR_ARGS_PREPROCESS: [&'static str; 1] = ["-E"];


#[derive(Debug)]
struct Uniform {
    name: String,
    type_id: u32,
    size: u32,
}

#[derive(Debug)]
struct Attribute {
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


fn extract_shader_info(shaders: &[&str]) -> (Vec<Attribute>, Vec<Uniform>) {
    let output =
        Command::new(GLSL_VALIDATOR_EXECUTABLE)
            .args(shaders)
            .args(&GLSL_VALIDATOR_ARGS_INFO)
            .output()
            .expect(&format!("failed execute {} to extract info. args: {:?}", GLSL_VALIDATOR_EXECUTABLE, GLSL_VALIDATOR_ARGS_INFO));

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


fn prepocess_shader(shaders: &[&str]) -> [String; 2] {
    let mut res = [String::new(), String::new()];

    for (idx, ext) in [".vert", ".frag"].iter().enumerate() {
        let ext_shader: Vec<_> = shaders.iter().filter(|e| e.ends_with(ext)).collect();
        let output =
            Command::new(GLSL_VALIDATOR_EXECUTABLE)
                .args(&ext_shader)
                .args(&GLSL_VALIDATOR_ARGS_PREPROCESS)
                .output()
                .expect(&format!("failed execute {} to prerocess shader. args: {:?}", GLSL_VALIDATOR_EXECUTABLE, GLSL_VALIDATOR_ARGS_PREPROCESS));

        let stdout = str::from_utf8(&output.stdout).unwrap();
        res[idx] = stdout.to_string();
    }

    res
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