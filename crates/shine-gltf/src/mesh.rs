use crate::validation::{Checked, Error, Validate};
use crate::{accessor, extensions, material, Index};
use serde::{de, ser};
use serde_derive::{Deserialize, Serialize};
use serde_json::from_value;
use shine_gltf_macro::Validate;
use std::collections::HashMap;
use std::fmt;

/// Corresponds to `GL_POINTS`.
pub const POINTS: u32 = 0;

/// Corresponds to `GL_LINES`.
pub const LINES: u32 = 1;

/// Corresponds to `GL_LINE_LOOP`.
pub const LINE_LOOP: u32 = 2;

/// Corresponds to `GL_LINE_STRIP`.
pub const LINE_STRIP: u32 = 3;

/// Corresponds to `GL_TRIANGLES`.
pub const TRIANGLES: u32 = 4;

/// Corresponds to `GL_TRIANGLE_STRIP`.
pub const TRIANGLE_STRIP: u32 = 5;

/// Corresponds to `GL_TRIANGLE_FAN`.
pub const TRIANGLE_FAN: u32 = 6;

/// All valid primitive rendering modes.
pub const VALID_MODES: &[u32] = &[POINTS, LINES, LINE_LOOP, LINE_STRIP, TRIANGLES, TRIANGLE_STRIP, TRIANGLE_FAN];

/// All valid semantic names for Morph targets.
pub const VALID_MORPH_TARGETS: &[&str] = &["POSITION", "NORMAL", "TANGENT"];

/// The type of primitives to render.
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq)]
pub enum Mode {
    /// Corresponds to `GL_POINTS`.
    Points = 1,

    /// Corresponds to `GL_LINES`.
    Lines,

    /// Corresponds to `GL_LINE_LOOP`.
    LineLoop,

    /// Corresponds to `GL_LINE_STRIP`.
    LineStrip,

    /// Corresponds to `GL_TRIANGLES`.
    Triangles,

    /// Corresponds to `GL_TRIANGLE_STRIP`.
    TriangleStrip,

    /// Corresponds to `GL_TRIANGLE_FAN`.
    TriangleFan,
}

/// A set of primitives to be rendered.
///
/// A node can contain one or more meshes and its transform places the meshes in
/// the scene.
#[derive(Clone, Debug, Default, Deserialize, Serialize, Validate)]
pub struct Mesh {
    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::mesh::Mesh>,

    /// Defines the geometry to be renderered with a material.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub primitives: Vec<Primitive>,

    /// Defines the weights to be applied to the morph targets.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weights: Option<Vec<f32>>,
}

/// Geometry to be rendered with the given material.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Primitive {
    /// Maps attribute semantic names to the `Accessor`s containing the
    /// corresponding attribute data.
    pub attributes: HashMap<Checked<Semantic>, Index<accessor::Accessor>>,

    /// Extension specific data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extensions: Option<extensions::mesh::Primitive>,

    /// The index of the accessor that contains the indices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub indices: Option<Index<accessor::Accessor>>,

    /// The index of the material to apply to this primitive when rendering
    #[serde(skip_serializing_if = "Option::is_none")]
    pub material: Option<Index<material::Material>>,

    /// The type of primitives to render.
    #[serde(default, skip_serializing_if = "is_primitive_mode_default")]
    pub mode: Checked<Mode>,

    /// An array of Morph Targets, each  Morph Target is a dictionary mapping
    /// attributes (only `POSITION`, `NORMAL`, and `TANGENT` supported) to their
    /// deviations in the Morph Target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<Vec<MorphTarget>>,
}

// Help serde avoid serializing this glTF 2.0 default value.
#[allow(clippy::trivially_copy_pass_by_ref)]
fn is_primitive_mode_default(mode: &Checked<Mode>) -> bool {
    *mode == Checked::Valid(Mode::Triangles)
}

impl Validate for Primitive {
    fn validate_minimally<P, R>(&self, root: &crate::Root, path: P, report: &mut R)
    where
        P: Fn() -> crate::Path,
        R: FnMut(&dyn Fn() -> crate::Path, crate::validation::Error),
    {
        // Generated part
        self.attributes
            .validate_minimally(root, || path().field("attributes"), report);
        self.extensions
            .validate_minimally(root, || path().field("extensions"), report);
        self.indices.validate_minimally(root, || path().field("indices"), report);
        self.material.validate_minimally(root, || path().field("material"), report);
        self.mode.validate_minimally(root, || path().field("mode"), report);
        self.targets.validate_minimally(root, || path().field("targets"), report);

        // Custom part
        let position_path = &|| path().field("attributes").key("POSITION");
        if let Some(pos_accessor_index) = self.attributes.get(&Checked::Valid(Semantic::Positions)) {
            // spec: POSITION accessor **must** have `min` and `max` properties defined.
            let pos_accessor = &root.accessors[pos_accessor_index.value()];

            let min_path = &|| position_path().field("min");
            if let Some(ref min) = pos_accessor.min {
                if from_value::<[f32; 3]>(min.clone()).is_err() {
                    report(min_path, Error::Invalid);
                }
            } else {
                report(min_path, Error::Missing);
            }

            let max_path = &|| position_path().field("max");
            if let Some(ref max) = pos_accessor.max {
                if from_value::<[f32; 3]>(max.clone()).is_err() {
                    report(max_path, Error::Invalid);
                }
            } else {
                report(max_path, Error::Missing);
            }
        } else {
            report(position_path, Error::Missing);
        }
    }
}

/// A dictionary mapping attributes to their deviations in the Morph Target.
#[derive(Clone, Debug, Deserialize, Serialize, Validate)]
pub struct MorphTarget {
    /// XYZ vertex position displacements of type `[f32; 3]`.
    #[serde(rename = "POSITION")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub positions: Option<Index<accessor::Accessor>>,

    /// XYZ vertex normal displacements of type `[f32; 3]`.
    #[serde(rename = "NORMAL")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub normals: Option<Index<accessor::Accessor>>,

    /// XYZ vertex tangent displacements of type `[f32; 3]`.
    #[serde(rename = "TANGENT")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tangents: Option<Index<accessor::Accessor>>,
}

/// Vertex attribute semantic name.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Semantic {
    /// XYZ vertex positions.
    Positions,

    /// XYZ vertex normals.
    Normals,

    /// XYZW vertex tangents where the `w` component is a sign value indicating the
    /// handedness of the tangent basis.
    Tangents,

    /// RGB or RGBA vertex color.
    Colors(u32),

    /// UV texture co-ordinates.
    TexCoords(u32),

    /// Joint indices.
    Joints(u32),

    /// Joint weights.
    Weights(u32),
}

impl Default for Mode {
    fn default() -> Mode {
        Mode::Triangles
    }
}

impl Mode {
    /// Returns the equivalent `GLenum`.
    pub fn as_gl_enum(self) -> u32 {
        match self {
            Mode::Points => POINTS,
            Mode::Lines => LINES,
            Mode::LineLoop => LINE_LOOP,
            Mode::LineStrip => LINE_STRIP,
            Mode::Triangles => TRIANGLES,
            Mode::TriangleStrip => TRIANGLE_STRIP,
            Mode::TriangleFan => TRIANGLE_FAN,
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Mode> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Mode>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "any of: {:?}", VALID_MODES)
            }

            fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                use self::Mode::*;
                use crate::validation::Checked::*;
                Ok(match value as u32 {
                    POINTS => Valid(Points),
                    LINES => Valid(Lines),
                    LINE_LOOP => Valid(LineLoop),
                    LINE_STRIP => Valid(LineStrip),
                    TRIANGLES => Valid(Triangles),
                    TRIANGLE_STRIP => Valid(TriangleStrip),
                    TRIANGLE_FAN => Valid(TriangleFan),
                    _ => Invalid,
                })
            }
        }
        deserializer.deserialize_u64(Visitor)
    }
}

impl ser::Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_u32(self.as_gl_enum())
    }
}

impl Semantic {
    fn checked(s: &str) -> Checked<Self> {
        use self::Semantic::*;
        use crate::validation::Checked::*;
        match s {
            "NORMAL" => Valid(Normals),
            "POSITION" => Valid(Positions),
            "TANGENT" => Valid(Tangents),
            _ if s.starts_with("COLOR_") => match s["COLOR_".len()..].parse() {
                Ok(set) => Valid(Colors(set)),
                Err(_) => Invalid,
            },
            _ if s.starts_with("TEXCOORD_") => match s["TEXCOORD_".len()..].parse() {
                Ok(set) => Valid(TexCoords(set)),
                Err(_) => Invalid,
            },
            _ if s.starts_with("JOINTS_") => match s["JOINTS_".len()..].parse() {
                Ok(set) => Valid(Joints(set)),
                Err(_) => Invalid,
            },
            _ if s.starts_with("WEIGHTS_") => match s["WEIGHTS_".len()..].parse() {
                Ok(set) => Valid(Weights(set)),
                Err(_) => Invalid,
            },
            _ => Invalid,
        }
    }
}

impl ser::Serialize for Semantic {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for Semantic {
    fn to_string(&self) -> String {
        use self::Semantic::*;
        match *self {
            Positions => "POSITION".into(),
            Normals => "NORMAL".into(),
            Tangents => "TANGENT".into(),
            Colors(set) => format!("COLOR_{}", set),
            TexCoords(set) => format!("TEXCOORD_{}", set),
            Joints(set) => format!("JOINTS_{}", set),
            Weights(set) => format!("WEIGHTS_{}", set),
        }
    }
}

impl ToString for Checked<Semantic> {
    fn to_string(&self) -> String {
        match *self {
            Checked::Valid(ref semantic) => semantic.to_string(),
            Checked::Invalid => "<invalid semantic name>".into(),
        }
    }
}

impl<'de> de::Deserialize<'de> for Checked<Semantic> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct Visitor;
        impl<'de> de::Visitor<'de> for Visitor {
            type Value = Checked<Semantic>;

            fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "semantic name")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Semantic::checked(value))
            }
        }
        deserializer.deserialize_str(Visitor)
    }
}

/// Helper to initialize an attribute map in the  [ Att1 => value1, Attr2 => val2 ] form.
#[macro_export]
macro_rules! attribute_map {
    ($( $key: ident => $val: expr ),*) => {{
        use $crate::validation::Checked;
        use $crate::mesh::Semantic;
        let mut map = ::std::collections::HashMap::new();
        $( map.insert(Checked::Valid(Semantic::$key), $val); )*
        map
    }}
}

/// Helper to initialize an attribute map in the [ Att1 => value1, Attr2 => val2 ] form where value is an Option
#[macro_export]
macro_rules! optional_attribute_map {
    ($( $key: ident => $val: expr ),*) => {{
        use $crate::validation::Checked;
        use $crate::mesh::Semantic;
        let mut map = ::std::collections::HashMap::new();
        $(
            match($val) {
                Some(val) => {map.insert(Checked::Valid(Semantic::$key), val);}
                None => {}
            };
        )*
        map
    }}
}
