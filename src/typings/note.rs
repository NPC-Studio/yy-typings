use crate::{ResourceVersion, Tags, ViewPath};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

/// This is a bodge to handle the fact that we don't currently have support for
/// many of the Gms2 yy-files. Eventually, we'd like to support all of them, but
/// downstream crates need to have some basic support until then. For now, this
/// can be used for all top level files, providing the simplest of support.
#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    /// The parent in the Gms2 virtual file system, ie. the parent which
    /// a user would see in the Navigation Pane in Gms2. This has no
    /// relationship to the actual operating system's filesystem.
    pub parent: ViewPath,

    /// The resource version of this yy file. At default 1.1.
    #[default(ResourceVersion::new(1, 1))]
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,

    /// The tags given to the object.
    pub tags: Tags,

    /// Const id tag of the note, given by Gms2.
    pub resource_type: ConstGmNote,
}

/// The constant of the GMNote. This is a hack.
#[derive(Debug, Copy, Serialize, Deserialize, SmartDefault, PartialEq, Eq, Clone)]
pub enum ConstGmNote {
    #[serde(rename = "GMNotes")]
    #[default]
    Const,
}
