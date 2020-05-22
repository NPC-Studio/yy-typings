use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

create_guarded_uuid!(TextureGroupId);

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct TextureGroup {
    #[default(true)]
    pub is_scaled: bool,
    #[default(true)]
    pub autocrop: bool,
    #[default(2)]
    pub border: usize,
    pub mips_to_generate: GenerateMipMaps,
    #[default(461609314234257646)]
    pub targets: usize,
    #[default("1.0".to_string())]
    pub resource_version: String,
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
