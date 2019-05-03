use serde_derive::{Deserialize, Serialize};
use shine_gltf_macro::Validate;

/// Image data used to create a texture.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Image {}
