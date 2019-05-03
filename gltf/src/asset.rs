use crate::extensions;
use serde_derive::{Deserialize, Serialize};
use shine_gltf_macro::Validate;

/// Metadata about the glTF asset.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Asset {
    /// A copyright message suitable for display to credit the content creator.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<String>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::asset::Asset>,

    /// Tool that generated this glTF model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generator: Option<String>,

    /// The minimum glTF version that this asset targets.
    #[serde(rename = "minVersion")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_version: Option<String>,

    /// The glTF version of this asset.
    pub version: String,
}

impl Default for Asset {
    fn default() -> Self {
        Self {
            copyright: None,
            extensions: Default::default(),
            generator: None,
            min_version: None,
            version: "2.0".to_string(),
        }
    }
}
