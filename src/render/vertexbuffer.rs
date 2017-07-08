use render::*;

pub trait IVertexSource {}

pub trait IVertexBuffer {
    fn iter_mut<'a, V: IVertexSource>(&mut self, queue: &mut CommandQueue, count: usize) /*-> Iterator<Item=&'a V>*/;
    fn release(&mut self, queue: &mut CommandQueue);
}

macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

macro_rules! vertex_declaration_init_member {
  ($_type:ty = $_default:expr) => { $_default };
  ($_type:ty) => { Default::default() };
}

#[macro_export]
macro_rules! vertex_declaration {
    ($_decl:ident{$($_name:ident: $_type:ty $(= $_default:expr)* ),+})  => {
        #[allow(non_snake_case, dead_code, non_camel_case_types)]
        pub mod $_decl {
            use super::*;
            use std::fmt;
            use std::ops::{Deref, DerefMut};

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
                   write!(f,
                        concat!($(replace_expr!($_name "{}:{:?}, ")),*),
                          $(stringify!($_name), self.$_name),*)
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

            pub struct StaticSource(Vec<Vertex>);

            impl StaticSource {
                pub fn new(size: usize) -> StaticSource {
                    StaticSource( vec!(Default::default(); size) )
                }
            }

            impl Deref for StaticSource {
                type Target = Vec<Vertex>;
                fn deref(&self) -> &Vec<Vertex> {
                    &self.0
                }
            }

            impl DerefMut for StaticSource {
                fn deref_mut(&mut self) -> &mut Vec<Vertex> {
                    &mut self.0
                }
            }

            impl IVertexSource for StaticSource {}
        }
    };

    // handle trailing comma
    ($_decl:ident{$($_name:ident: $_type:ty $(= $_default:expr)* ),*,})  => {
        vertex_declaration!( $_decl{ $( $_name: $_type $(= $_default)*),* });
    };
}