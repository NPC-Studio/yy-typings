use serde::{Deserialize, Serialize};

use crate::ResourceVersion;

/// The common fields in all Resource files.
#[derive(Debug, Serialize, Deserialize, Default, PartialEq, Clone, Eq, PartialOrd, Ord, Hash)]
#[serde(rename_all = "camelCase")]
pub struct CommonData<T, N = String> {
    pub resource_type: T,

    /// The resource version of this yy file. At default 1.0.
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: N,
}
