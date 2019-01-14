use crate::validation::{Error, Validate};
use crate::{buffer, extensions, Index, Path, Root};
use serde_derive::{Deserialize, Serialize};
use shine_gltf_macro::Validate;

/// All valid MIME types.
pub const VALID_MIME_TYPES: &'static [&'static str] = &["image/jpeg", "image/png"];

/// Image data used to create a texture.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct Image {
    /// The index of the buffer view that contains the image. Use this instead of
    /// the image's uri property.
    #[serde(rename = "bufferView")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<Index<buffer::View>>,

    /// The image's MIME type.
    #[serde(rename = "mimeType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<MimeType>,

    /// The uri of the image.  Relative paths are relative to the .gltf file.
    /// Instead of referencing an external file, the uri can also be a data-uri.
    /// The image format must be jpg or png.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uri: Option<String>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::image::Image>,
}

/// An image MIME type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MimeType(pub String);

impl Validate for MimeType {
    fn validate_completely<P, R>(&self, _: &Root, path: P, report: &mut R)
    where
        P: Fn() -> Path,
        R: FnMut(&dyn Fn() -> Path, Error),
    {
        if !VALID_MIME_TYPES.contains(&self.0.as_str()) {
            report(&path, Error::Invalid);
        }
    }
}
