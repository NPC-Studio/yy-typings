use std::collections::BTreeMap;

use crate::{TexturePath, TexturePathLocation};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct TextureGroup {
    #[serde(flatten)]
    pub common_data: crate::CommonData<ConstGmTextureGroup>,

    #[serde(rename = "ConfigValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_values: Option<BTreeMap<String, serde_json::Value>>,

    #[default(true)]
    pub is_scaled: bool,
    pub compress_format: String,
    #[default(true)]
    pub autocrop: bool,
    #[default(2)]
    pub border: usize,
    pub mips_to_generate: GenerateMipMaps,
    pub group_parent: Option<TexturePath>,
    #[default(-1)]
    pub targets: isize,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, SmartDefault, Eq, PartialEq)]
pub enum ConstGmTextureGroup {
    #[serde(rename = "GMTextureGroup")]
    #[default]
    Const,
}

#[derive(
    Debug,
    Serialize_repr,
    Deserialize_repr,
    SmartDefault,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
    Clone,
    Copy,
)]
#[repr(i8)]
pub enum GenerateMipMaps {
    True = -1,
    #[default]
    False = 0,
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
