use serde::{Deserialize, Serialize};

use crate::{ResourceVersion, Tags, ViewPath};

/// The common fields in all Resource files.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ResourceData {
    /// The parent in the Gms2 virtual file system, ie. the parent which
    /// a user would see in the Navigation Pane in Gms2. This has no
    /// relationship to the actual operating system's filesystem.
    pub parent: ViewPath,

    /// The resource version of this yy file. At default 1.0.
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,

    /// The tags given to the object.
    pub tags: Tags,
}

#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq, PartialOrd, Ord)]
#[serde(rename_all = "camelCase")]
pub struct ResourceSubData {
    /// The resource version of this yy file. At default 1.0.
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: String,

    /// The tags given to the object.
    pub tags: Tags,
}
