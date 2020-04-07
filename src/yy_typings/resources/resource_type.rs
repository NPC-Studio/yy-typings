use serde::{Deserialize, Serialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
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
