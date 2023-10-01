use serde::{Deserialize, Serialize};

/// This is a bodge to handle the fact that we don't currently have support for
/// many of the Gms2 yy-files. Eventually, we'd like to support all of them, but
/// downstream crates need to have some basic support until then. For now, this
/// can be used for all top level files, providing the simplest of support.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::GmNote>,
    pub parent: crate::ViewPath,
}

gm_const!(GmNote -> "GMNotes");
