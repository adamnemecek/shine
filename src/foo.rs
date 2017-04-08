//#![feature(trace_macros)]
//trace_macros!(true);
use std::fmt;
use std::slice;
use std::mem;

trait VertexDecaration {
    fn get_component_count(&self) -> usize;
}

#[repr(C)]
#[derive(Debug)]
pub struct Float32x4(f32,f32,f32,f32);

impl fmt::Display for Float32x4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{},{},{})", self.0, self.1, self.2, self.3)
    }
}

impl Float32x4 {
    fn gl_type() -> usize {
        0
    }

    pub fn to_bytes(&self) -> &[u8] {
        let p: *const Self = self;
        let p = p as *const u8;
        unsafe{
            slice::from_raw_parts(p, mem::size_of::<Self>())
        }
    }
}

impl Default for Float32x4 {
    fn default() -> Float32x4 {
        Float32x4(0.,0.,0.,0.)
    }
}

macro_rules! f32x4 {
    ($_x:expr, $_y:expr, $_z:expr, $_w:expr) => { Float32x4($_x as f32, $_y as f32, $_z as f32, $_w as f32) };
    ($_x:expr) => { Float32x4($_x as f32, $_x as f32, $_x as f32, $_x as f32) };
    () => { Default::default() };
}




pub struct Float2 {
    v: [f32; 2],
}

impl fmt::Display for Float2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({},{})", self.v[0], self.v[1])
    }
}

impl Float2 {
    fn gl_type() -> usize {
        1
    }

    fn new(x: f32, y: f32) -> Float2 {
        Float2 { v: [x, y] }
    }
}

impl Default for Float2 {
    fn default() -> Float2 {
        Float2 { v: [0.; 2] }
    }
}

pub fn f32x2(x: f32, y: f32) -> Float2 {
    Float2 { v: [x, y] }
}



macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}


macro_rules! vertex_declaration_init_member {
  ($_type:ty = $_default:expr) => { $_default };
  ($_type:ty) => { Default::default() };
}

macro_rules! vertex_declaration {
    ($_decl:ident{$($_name:ident: $_type:ty $(= $_default:expr)* ),+})  => {
        #[allow(non_snake_case, dead_code, non_camel_case_types)]
        pub mod $_decl {
            use std::fmt;
            use super::*;

            pub struct Vertex {
                $(pub $_name: $_type),*
            }

            impl Vertex {
                pub fn new() -> Vertex {
                    Vertex{ $($_name: vertex_declaration_init_member!($_type $(= $_default)*)),* }
                }
            }

            impl fmt::Display for Vertex {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                   //write!(f, vertex_declaration_build_fmt_args!($($_name)*), $(stringify!($_name), stringify!($_type), self.$_name),*)
                   write!(f,
                          concat!($(replace_expr!($_name "{}:{}{}, ")),*),
                          $(stringify!($_name), stringify!($_type), self.$_name),*)
                }
            }

            #[derive(Debug)]
            pub enum Location {
                $($_name,)*
                Count,
            }

            impl fmt::Display for Location {
                fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                    match *self {
                        $(Location::$_name => write!(f, "{}({})", stringify!($_name), Location::$_name as u32)),*,
                        Location::Count => write!(f, "Count({})", Location::Count as u32),
                    }
                }
            }

            pub fn get_location_by_name(name: &str) -> Location {
                match name {
                    $(stringify!($_name) => Location::$_name),*,
                    _ => Location::Count,
                }
            }

            pub fn allocate(len: usize) -> Vec<Vertex> {
                let mut v = Vec::new();
                v.reserve_exact(len);
                v
            }

        }
    };

    // handle trailing comma
    ($_decl:ident{$($_name:ident: $_type:ty $(= $_default:expr)* ),*,})  => {
        vertex_declaration!( $_decl{ $( $_name: $_type $(= $_default)*),* });
    };
}


vertex_declaration!(Alma{
    position: Float32x4 = f32x4!(),
    color: Float2 = f32x2(0.,1.),
});

pub fn do_foo() {
    let a = Alma::Vertex::new();
    println!("{}", a);
    println!("{:?}", a.position);

    let mut buf = Alma::allocate(3);
    buf.push(Alma::Vertex { position: f32x4!(30), color: Float2::new(41., 34.) });
    buf.push(a);

    println!("{}, {}, {}", Alma::Location::position, Alma::Location::color, Alma::Location::Count);
    println!("{}", Alma::get_location_by_name("position"));
    println!("{}", Alma::get_location_by_name("color"));
    println!("{}", Alma::get_location_by_name("sdf"));

    println!("{}", buf[0].position);

    use std::process;
    process::exit(1);
}





