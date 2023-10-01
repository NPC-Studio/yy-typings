use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct AudioGroup {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::GmAudioGroup, String, 1, 3>,
    #[default(-1)]
    pub targets: isize,
}

gm_const!(GmAudioGroup -> "GmAudioGroup");
