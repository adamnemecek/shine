#![deny(missing_docs)]
#![deny(missing_copy_implementations)]

/// Maximum number of vertex attributes stored in a buffer.
pub const MAX_VERTEX_ATTRIBUTE_COUNT: usize = 16;

#[macro_export]
macro_rules! vertex_declaration {
    // internal rule init
    (@init $type:ty = $default:expr) => { $default };
    (@init $type:ty) => { Default::default() };

    // internal rule offset_of
    (@offset_of $s:tt, $m:ident) => {0};

    // internal rule as_item
    (@as_item $i:ident) => {$i};

    //(@concat_ident $i:ident,$i2:ident) => {$i2};

    // the macro implementation
    ($decl:ident, $loc: ident, attributes{$($name:ident: $type:ty $(= $default:expr)* ),+})  => {

        #[allow(non_camel_case_types)]
        #[derive(Debug)]
        pub enum $loc {
            $($name,)*
            Count,
        }

        #[allow(dead_code, non_camel_case_types)]
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        pub struct $decl {
            $(pub $name: $type),*
        }

        impl $decl {
            pub fn new() -> $decl {
                $decl {
                    $($name: vertex_declaration!{@init $type $(= $default)*}),*
                }
            }

            fn get_declaration(vertex_count: usize) -> [render::VertexAttributeImpl; $loc::Count as usize] {
                use std::mem;
                [
                    $(render::VertexAttributeImpl::new_from_element::<$type>(
                        vertex_count,
                        vertex_declaration!{@offset_of $decl, $name},
                        mem::size_of::<$decl>()
                    )),*
                ]
            }

            pub fn get_location_by_name(name: &str) -> $loc {
                match name {
                    $(stringify!($name) => $loc::$name),*,
                    _ => $loc::Count,
                }
            }
        }

        impl Default for $decl {
            fn default() -> $decl {
                $decl::new()
            }
        }
    };

    // handle trailing comma
    ($decl:ident, $loc: ident, attributes{$($name:ident: $type:ty $(= $default:expr)* ),*,})  => {
        vertex_declaration!{ $decl, $loc, attributes{ $( $name: $type $(= $default)*),* }}
    };
}

