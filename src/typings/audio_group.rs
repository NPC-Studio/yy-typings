use crate::ResourceVersion;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct AudioGroup {
    #[default(-1)]
    pub targets: isize,
    #[default(ResourceVersion::new(1, 3))]
    pub resource_version: ResourceVersion,
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
