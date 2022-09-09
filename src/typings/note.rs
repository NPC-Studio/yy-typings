use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

/// This is a bodge to handle the fact that we don't currently have support for
/// many of the Gms2 yy-files. Eventually, we'd like to support all of them, but
/// downstream crates need to have some basic support until then. For now, this
/// can be used for all top level files, providing the simplest of support.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(flatten)]
    pub common_data: crate::CommonData<ConstGmNote>,
    pub parent: crate::ViewPath,
}

/// The constant of the GMNote. This is a hack.
#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmNote {
    #[serde(rename = "GMNotes")]
    #[default]
    Const,
}
