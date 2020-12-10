use std::collections::BTreeMap;

use crate::{ResourceVersion, TexturePath, TexturePathLocation};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct TextureGroup {
    #[serde(rename = "ConfigValues")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_values: Option<BTreeMap<String, serde_json::Value>>,
    #[default(true)]
    pub is_scaled: bool,
    #[default(true)]
    pub autocrop: bool,
    #[default(2)]
    pub border: usize,
    pub mips_to_generate: GenerateMipMaps,
    pub group_parent: Option<TexturePath>,
    #[default(461609314234257646)]
    pub targets: usize,
    pub resource_version: ResourceVersion,
    #[default("Default".to_string())]
    pub name: String,
    pub resource_type: ConstGmTextureGroup,
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

impl Into<TexturePath> for TextureGroup {
    fn into(self) -> TexturePath {
        TexturePath {
            name: self.name.clone(),
            path: TexturePathLocation(format!("texturegroups/{}", self.name)),
        }
    }
}

impl Into<TexturePath> for &TextureGroup {
    fn into(self) -> TexturePath {
        TexturePath {
            name: self.name.clone(),
            path: TexturePathLocation(format!("texturegroups/{}", self.name)),
        }
    }
}
