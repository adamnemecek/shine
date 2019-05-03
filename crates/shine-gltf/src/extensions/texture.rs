use serde_derive::{Deserialize, Serialize};
use shine_gltf_macro::Validate;

/// Texture sampler properties for filtering and wrapping modes.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Sampler {}

/// A texture and its sampler.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Texture {}

#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
/// Reference to a `Texture`.
pub struct Info {}
