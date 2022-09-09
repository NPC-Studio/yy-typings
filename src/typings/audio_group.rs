use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct AudioGroup {
    #[serde(flatten)]
    pub common_data: crate::CommonData<ConstGmAudioGroup>,
    #[default(-1)]
    pub targets: isize,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, SmartDefault)]
pub enum ConstGmAudioGroup {
    #[serde(rename = "GMAudioGroup")]
    #[default]
    Const,
}
