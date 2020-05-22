use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct AudioGroup {
    #[default(461609314234257646)]
    pub targets: usize,
    #[default("1.0".to_string())]
    pub resource_version: String,
    #[default("audiogroup_default".to_string())]
    pub name: String,
    pub resource_type: ConstGmAudioGroup,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, SmartDefault)]
pub enum ConstGmAudioGroup {
    #[serde(rename = "GMAudioGroup")]
    #[default]
    Const,
}
