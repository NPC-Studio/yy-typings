use serde::{Deserialize, Serialize};

create_guarded_uuid!(YypResourceKeyId);
create_guarded_uuid!(YypResourceId);

/// Represents a resource entry in a YYP
#[derive(Debug, Serialize, Deserialize)]
pub struct YypResource {
    /// This resource entry GUID (not the GUID of the resource itself). Appears to serve no
    /// purpose.
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

    /// GUID of the resource
    pub id: YypResourceId,

    /// Contains the relative backslash-escaped path to the resource's .yy file
    pub resource_path: String,

    /// Describes the resource type
    pub resource_type: ResourceType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConstGmResourceInfo {
    #[serde(rename = "GMResourceInfo")]
    GmResourceInfo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub enum ResourceType {
    #[serde(rename = "GMObject")]
    GmObject,
    #[serde(rename = "GMIncludedFile")]
    GmIncludedFile,
    #[serde(rename = "GMExtension")]
    GmExtension,
    #[serde(rename = "GMExtensionFile")]
    GmExtensionFile,
    #[serde(rename = "GMExtensionConstant")]
    GmExtensionConstant,
    #[serde(rename = "GMFont")]
    GmFont,
    #[serde(rename = "GMNote")]
    GmNote,
    #[serde(rename = "GMOption")]
    GmOption,
    #[serde(rename = "GMPath")]
    GmPath,
    #[serde(rename = "GMRoom")]
    GmRoom,
    #[serde(rename = "GMScript")]
    GmScript,
    #[serde(rename = "GMShader")]
    GmShader,
    #[serde(rename = "GMSound")]
    GmSound,
    #[serde(rename = "GMSprite")]
    GmSprite,
    #[serde(rename = "GMTileSet")]
    GmTileSet,
    #[serde(rename = "GMFolder")]
    GmFolder,
    #[serde(rename = "GMTimeline")]
    GmTimeline,
    #[serde(rename = "GMMainOptions")]
    GmMainOptions,
    #[serde(rename = "GMWindowsOptions")]
    GmWindowsOptions,
    #[serde(rename = "GMLinuxOptions")]
    GmLinuxOptions,
    #[serde(rename = "GMMacOptions")]
    GmMacOptions,
    #[serde(rename = "GMNotes")]
    GmNotes,
    #[serde(rename = "GMHtml5Options")]
    GmHtml5Options,
}
