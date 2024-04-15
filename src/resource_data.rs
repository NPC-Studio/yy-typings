use serde::{Deserialize, Serialize};

use crate::ResourceVersion;

/// The common fields in all Resource files.
#[derive(
    Debug,
    Serialize,
    Deserialize,
    smart_default::SmartDefault,
    PartialEq,
    Clone,
    Eq,
    PartialOrd,
    Ord,
    Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct CommonData<
    T: Default,
    N: Default = String,
    const MAJOR: usize = 2,
    const MINOR: usize = 0,
> {
    pub resource_type: T,

    /// The resource version of this yy file. At default 1.0.
    #[default(ResourceVersion::new(MAJOR, MINOR))]
    pub resource_version: ResourceVersion,

    /// The name of the object. This is the human readable name used in the IDE.
    pub name: N,
}

impl<T, N, const MAJOR: usize, const MINOR: usize> CommonData<T, N, MAJOR, MINOR>
where
    T: Default,
    N: Default,
{
    /// Creates a new CommonData
    pub fn new(name: N) -> Self {
        Self {
            name,
            ..Self::default()
        }
    }
}
