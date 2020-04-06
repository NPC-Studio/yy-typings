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
    #[serde(rename = "GMSwitchOptions")]
    GmSwitchOptions,
    #[serde(rename = "GMPS4Options")]
    GmPs4Options,
    #[serde(rename = "GMWindowsUAPOptions")]
    GmWindowsUapOptions,
    #[serde(rename = "GMiOSOptions")]
    GmiOSOptions,
    #[serde(rename = "GMtvOSOptions")]
    GmtvOSOptions,
    #[serde(rename = "GMAmazonFireOptions")]
    GmAmazonFireOptions,
    #[serde(rename = "GMAndroidOptions")]
    GmAndroidOptions,
    #[serde(rename = "GMXBoxOneOptions")]
    GmXboxOneOptions,
}
