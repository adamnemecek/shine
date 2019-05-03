use serde_derive::{Deserialize, Serialize};
use shine_gltf_macro::Validate;

/// Joints and matrices defining a skin.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Skin {}
