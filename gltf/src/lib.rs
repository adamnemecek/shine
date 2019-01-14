use serde_json;

/// Contains `Accessor` and other related data structures.
pub mod accessor;

/// Contains `Animation` and other related data structures.
pub mod animation;

/// Contains `Asset` metadata.
pub mod asset;

/// Contains `Buffer`, `View`, and other related data structures.
pub mod buffer;

/// Contains `Camera` and other related data structures.
pub mod camera;

/// Contains extension specific data structures and the names of all
/// 2.0 extensions supported by the library.
pub mod extensions;

/// Contains `Image` and other related data structures.
pub mod image;

/// Contains `Material` and other related data structures.
pub mod material;

/// Contains `Mesh` and other related data structures.
#[macro_use]
pub mod mesh;

/// Contains `Path`.
pub mod path;

/// Contains `Root`.
pub mod root;

/// Contains `Scene`, `Node`, and other related data structures.
pub mod scene;

/// Contains `Skin` and other related data structures.
pub mod skin;

/// Contains `Texture`, `Sampler`, and other related data structures.
pub mod texture;

/// Contains functions that validate glTF JSON data against the specification.
pub mod validation;

#[doc(inline)]
pub use crate::accessor::Accessor;
#[doc(inline)]
pub use crate::animation::Animation;
#[doc(inline)]
pub use crate::asset::Asset;
#[doc(inline)]
pub use crate::buffer::Buffer;
#[doc(inline)]
pub use crate::camera::Camera;
#[doc(inline)]
pub use crate::image::Image;
#[doc(inline)]
pub use crate::material::Material;
#[doc(inline)]
pub use crate::mesh::Mesh;
#[doc(inline)]
pub use crate::mesh::Primitive;
//#[doc(inline)]
//pub use mesh::attribute_map;
#[doc(inline)]
pub use crate::scene::Node;
#[doc(inline)]
pub use crate::scene::Scene;
#[doc(inline)]
pub use crate::skin::Skin;
#[doc(inline)]
pub use crate::texture::Texture;

#[doc(inline)]
pub use crate::path::Path;
#[doc(inline)]
pub use crate::root::Get;
#[doc(inline)]
pub use crate::root::GetMut;
#[doc(inline)]
pub use crate::root::Index;
#[doc(inline)]
pub use crate::root::Root;

#[doc(inline)]
pub use serde_json::Error;
#[doc(inline)]
pub use serde_json::Value;

/// Re-exports of `serde_json` deserialization functions.
///
/// This module re-exports the generic serde deserialization functions
/// so that one can deserialize data structures other than `Root` without
/// being bound to a specific version of `serde_json`.
pub mod deserialize {
    pub use serde_json::{from_reader, from_slice, from_str, from_value};
}

/// Re-exports of `serde_json` serialization functions.
///
/// This module re-exports the generic serde serialization functions
/// so that one can serialize data structures other than `Root` without
/// being bound to a specific version of `serde_json`.
pub mod serialize {
    pub use serde_json::{to_string, to_string_pretty, to_value, to_vec, to_vec_pretty, to_writer, to_writer_pretty};
}
