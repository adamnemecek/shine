use serde_derive::{Deserialize, Serialize};
use shine_gltf_macro::Validate;

/// The root object of a glTF 2.0 asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Root {}
