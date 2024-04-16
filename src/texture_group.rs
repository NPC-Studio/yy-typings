use std::collections::BTreeMap;

use crate::{TexturePath, TexturePathLocation};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct TextureGroup {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::TextureGroup, String>,

    #[default(true)]
    pub autocrop: bool,

    #[default(2)]
    pub border: usize,

    pub compress_format: String,

    /// Some custom options, we'll probably do evil things with this
    pub custom_options: String,

    #[serde(rename = "ConfigValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_values: Option<BTreeMap<String, serde_json::Value>>,

    pub directory: String,
    pub group_parent: Option<TexturePath>,

    #[default(true)]
    pub is_scaled: bool,

    /// Unclear what this means -- seems to be just "default"
    pub load_type: String,

    /// 0 for true, -1 for false. The normal numbers ofc.
    pub mips_to_generate: i8,

    #[default(-1)]
    pub targets: isize,
}

impl From<TextureGroup> for TexturePath {
    fn from(o: TextureGroup) -> Self {
        TexturePath {
            name: o.common_data.name.clone(),
            path: TexturePathLocation(format!("texturegroups/{}", o.common_data.name)),
        }
    }
}

impl From<&TextureGroup> for TexturePath {
    fn from(o: &TextureGroup) -> TexturePath {
        TexturePath {
            name: o.common_data.name.clone(),
            path: TexturePathLocation(format!("texturegroups/{}", o.common_data.name)),
        }
    }
}

gm_const!(TextureGroup -> "GMTextureGroup");
