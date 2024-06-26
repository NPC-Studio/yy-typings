use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shader {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Shader>,

    pub parent: crate::ViewPath,

    #[serde(rename = "type")]
    pub shader_type: ShaderType,
}

impl Shader {
    pub const FRAG_FILE_ENDING: &'static str = "fsh";
    pub const VERT_FILE_ENDING: &'static str = "vsh";
}

#[derive(Debug, Serialize_repr, Deserialize_repr, SmartDefault, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum ShaderType {
    #[default]
    GlslEs = 1,
    Glsl,
    Hlsl,
}

gm_const!(Shader -> "GMShader");

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{utils::TrailingCommaUtility, ResourceVersion, ViewPath, ViewPathLocation};
    use include_dir::{include_dir, Dir, DirEntry};
    use pretty_assertions::assert_eq;

    #[test]
    fn trivial_sprite_parsing() {
        static DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/data/shaders");
        let tcu = TrailingCommaUtility::new();

        for file in DIR.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = file {
                println!("parsing {}", file.path().display());
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Shader>(&our_str).unwrap();
            }
        }
    }

    #[test]
    fn deep_equality() {
        let file_raw = include_str!("../data/shaders/shd_anchor_outline.yy");

        let file_parsed: Shader =
            serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(file_raw))
                .unwrap();

        let script = Shader {
            common_data: crate::CommonData {
                resource_type: consts::Shader,
                resource_version: ResourceVersion::default(),
                name: "shd_anchor_outline".to_string(),
            },
            shader_type: ShaderType::GlslEs,
            parent: ViewPath {
                name: "Shaders".to_string(),
                path: ViewPathLocation("folders/Shaders.yy".to_string()),
            },
        };

        assert_eq!(file_parsed, script);
    }
}
