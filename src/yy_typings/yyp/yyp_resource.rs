use super::ResourceType;
use serde::{Deserialize, Serialize};

create_guarded_uuid!(YypResourceKeyId);
create_guarded_uuid!(YypResourceId);

/// Represents a resource entry in a YYP
#[derive(Debug, Serialize, Deserialize)]
pub struct YypResource {
    /// This resource UUID. All `.id` files will match up to this!
    #[serde(rename = "Key")]
    pub key: YypResourceKeyId,
    /// Contains resource information
    #[serde(rename = "Value")]
    pub value: YypResourceValue,
}

/// Contains resource information
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct YypResourceValue {
    /// Unknown property, seems to always be an empty array
    #[serde(default)]
    pub config_deltas: Option<Vec<String>>,

    /// GUID of the resource. This is essentially meaningless and has no relationship to
    /// any other Id in the file, as far as I can tell.
    pub id: YypResourceId,

    /// Contains the relative backslash-escaped path to the resource's .yy file
    pub resource_path: std::path::PathBuf,

    /// Describes the resource type
    pub resource_type: ResourceType,
}
