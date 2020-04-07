use super::ResourceType;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;

create_guarded_uuid!(TextureGroupId);

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextureGroup {
    pub targets: usize,
    #[serde(default)]
    pub id: TextureGroupId,
    #[serde(default)]
    pub model_name: ConstGmTextureGroup,
    #[serde(default)]
    pub mvc: String,
    #[serde(default)]
    pub group_name: String,
    #[serde(default)]
    pub autocrop: bool,
    #[serde(default)]
    pub border: usize,
    #[serde(default)]
    pub group_parent: TextureGroupId,
    #[serde(default)]
    pub mips_to_generate: GenerateMipMaps,
    #[serde(default)]
    pub scaled: bool,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, SmartDefault)]
pub enum ConstGmTextureGroup {
    #[serde(rename = "GMTextureGroup")]
    #[default]
    GmTextureGroup,
}

impl From<ConstGmTextureGroup> for ResourceType {
    fn from(_: ConstGmTextureGroup) -> Self {
        Self::GmTextureGroup
    }
}

#[derive(Debug, Serialize_repr, Deserialize_repr, SmartDefault)]
#[repr(i8)]
pub enum GenerateMipMaps {
    True = -1,
    #[default]
    False = 0,
}
