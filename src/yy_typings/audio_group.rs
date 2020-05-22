use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AudioGroup {
    pub targets: usize,
    pub resource_version: String,
    pub name: String,
    pub resource_type: ConstGmAudioGroup,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy)]
pub enum ConstGmAudioGroup {
    #[serde(rename = "GMAudioGroup")]
    Const,
}
