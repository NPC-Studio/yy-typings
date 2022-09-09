use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Shader {
    #[serde(flatten)]
    pub common_data: crate::CommonData<ConstGmShader>,

    #[serde(rename = "type")]
    pub shader_type: ShaderType,

    pub parent: crate::ViewPath,
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
    use crate::{utils::TrailingCommaUtility, ResourceVersion, ViewPath, ViewPathLocation};
    use include_dir::{include_dir, Dir, DirEntry};
    use pretty_assertions::assert_eq;

    #[test]
    fn trivial_sprite_parsing() {
        static DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/data/shaders");
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
        let file_raw = include_str!("../../data/shaders/sh_draw_light_to_screen.yy");

        let file_parsed: Shader =
            serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(file_raw))
                .unwrap();

        let script = Shader {
            common_data: crate::CommonData {
                resource_type: ConstGmShader::Const,
                resource_version: ResourceVersion::default(),
                name: "sh_draw_light_to_screen".to_string(),
            },
            shader_type: ShaderType::GlslEs,
            parent: ViewPath {
                name: "shaders".to_string(),
                path: ViewPathLocation("folders/Objects/system/lighting/shaders.yy".to_string()),
            },
        };

        assert_eq!(file_parsed, script);
    }
}
