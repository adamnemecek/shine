use serde_derive::{Deserialize, Serialize};
use shine_gltf_macro::Validate;

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Asset {}
