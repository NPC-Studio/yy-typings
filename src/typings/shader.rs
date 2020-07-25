use crate::{ResourceVersion, Tags, ViewPath};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shader {
    #[serde(rename = "type")]
    pub shader_type: ShaderType,

    /// The parent in the Gms2 virtual file system, ie. the parent which
    /// a user would see in the Navigation Pane in Gms2. This has no relationship
    /// to the actual operating system's filesystem.
    pub parent: ViewPath,
    /// The resource version of this yy file. At default 1.0.
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,

    /// The tags given to the object.
    pub tags: Tags,

    /// Const id tag of the object, given by Gms2.
    pub resource_type: ConstGmShader,
}

impl Shader {
    pub const FRAG_FILE_ENDING: &'static str = "fsh";
    pub const VERT_FILE_ENDING: &'static str = "vsh";
}

#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmShader {
    #[serde(rename = "GMShader")]
    #[default]
    Const,
}

#[derive(Debug, Serialize_repr, Deserialize_repr, SmartDefault, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum ShaderType {
    #[default]
    GlslEs = 1,
    Glsl,
    Hlsl,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils::TrailingCommaUtility, ViewPathLocation};
    use include_dir::{include_dir, Dir, DirEntry};
    use pretty_assertions::assert_eq;

    #[test]
    fn trivial_sprite_parsing() {
        let dir: Dir = include_dir!("data/shaders");
        let tcu = TrailingCommaUtility::new();

        for file in dir.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = file {
                println!("parsing {}", file.path);
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Shader>(&our_str).unwrap();
            }
        }
    }

    #[test]
    fn deep_equality() {
        let file_raw = include_str!("../../data/shaders/sh_draw_light_to_screen.yy");

        let file_parsed: Shader =
            serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(file_raw))
                .unwrap();

        let script = Shader {
            shader_type: ShaderType::GlslEs,
            parent: ViewPath {
                name: "shaders".to_string(),
                path: ViewPathLocation("folders/Objects/system/lighting/shaders.yy".to_string()),
            },
            resource_version: ResourceVersion::default(),
            name: "sh_draw_light_to_screen".to_string(),
            tags: vec![],
            resource_type: ConstGmShader::Const,
        };

        assert_eq!(file_parsed, script);
    }
}
