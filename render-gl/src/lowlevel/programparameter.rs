use std::mem;
use std::cell::*;
use std::rc::*;
use lowlevel::*;
use libconfig::*;

/// Shader parameters info: required type, binding locations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ProgramParameter {
    Index,

    Attribute {
        name: String,
        location: GLint,
        size: GLint,
        type_id: GLenum,
    },

    Uniform {
        name: String,
        location: GLint,
        size: GLint,
        type_id: GLenum,
    },

    Empty,
}

impl ProgramParameter {
    pub fn is_valid(&self) -> bool {
        match self {
            &ProgramParameter::Attribute { type_id, .. } => type_id != 0,
            &ProgramParameter::Uniform { type_id, .. } => type_id != 0,
            _ => false
        }
    }

    pub fn set_index(&self, ll: &mut LowLevel, ib: &GLIndexBuffer) {
        match self {
            &ProgramParameter::Index => {
                gl_check_error();
                ib.bind(ll);
                gl_check_error();
            }
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_attribute<A: Into<usize>>(&self, ll: &mut LowLevel, vb: &GLVertexBuffer, attribute: A) {
        match self {
            &ProgramParameter::Attribute { location, type_id, .. } =>
                if type_id != 0 {
                    gl_check_error();
                    vb.bind(ll, location, attribute.into());
                    gl_check_error();
                },
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32(&self, _ll: &mut LowLevel, data: f32) {
        match self {
            &ProgramParameter::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT && size == 1);
                    gl_check_error();
                    ffi!(gl::Uniform1fv(location, size, mem::transmute(&data)));
                    gl_check_error();
                },
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x2(&self, _ll: &mut LowLevel, data: &Float32x2) {
        match self {
            &ProgramParameter::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_VEC2 && size == 1);
                    gl_check_error();
                    ffi!(gl::Uniform2fv(location, size, mem::transmute(data)));
                    gl_check_error();
                },
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x3(&self, _ll: &mut LowLevel, data: &Float32x3) {
        match self {
            &ProgramParameter::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_VEC3 && size == 1);
                    gl_check_error();
                    ffi!(gl::Uniform3fv(location, size, mem::transmute(data)));
                    gl_check_error();
                },
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x4(&self, _ll: &mut LowLevel, data: &Float32x4) {
        match self {
            &ProgramParameter::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_VEC4 && size == 1);
                    gl_check_error();
                    ffi!(gl::Uniform4fv(location, size, mem::transmute(data)));
                    gl_check_error();
                },
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }

    pub fn set_f32x16(&self, _ll: &mut LowLevel, data: &Float32x16) {
        match self {
            &ProgramParameter::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    assert!(type_id == gl::FLOAT_MAT4 && size == 1);
                    gl_check_error();
                    ffi!(gl::UniformMatrix4fv(location, size, gl::FALSE, mem::transmute(data)));
                    gl_check_error();
                },
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }


    pub fn set_texture_2d(&self, ll: &mut LowLevel, tx: &GLTexture) {
        match self {
            &ProgramParameter::Uniform { location, size, type_id, .. } =>
                if type_id != 0 {
                    // todo: check if texture conforms to the sampler
                    assert!(type_id == gl::SAMPLER_2D && size == 1);
                    gl_check_error();
                    let slot = tx.bind(ll) as u32;
                    ffi!(gl::Uniform1i(location, slot as i32));
                    gl_check_error();
                },
            &ProgramParameter::Empty => {}
            _ => panic!("Invalid Shader location: {:?}", self)
        }
    }
}

impl Default for ProgramParameter {
    fn default() -> ProgramParameter {
        ProgramParameter::Empty
    }
}


/// Type to store the required shader parameter locations.
pub type ProgramParameters = Rc<RefCell<[ProgramParameter; MAX_USED_PARAMETER_COUNT]>>;
