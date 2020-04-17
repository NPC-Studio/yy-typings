use super::{ResourceType, YypResourceKeyId};
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;

create_guarded_uuid!(GmFolderId);

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct GmFolder {
    /// The UUID of this Folder. It will also be the same as the Name.
    pub id: GmFolderId,

    /// The modelname "GMFolder"
    pub model_name: ConstGmFolder,

    /// Appears to be 1.1 at time of writing.
    pub mvc: String,

    /// This is the UUID of the Folder. It should *always* match the `id` field.
    pub name: GmFolderId,

    /// These are the children ID views which sit below the views in the tree.
    pub children: Vec<YypResourceKeyId>,

    /// `filter_type` is used for search and filtering purpoes in GMS2's ide.
    /// Only certain values should be placed here, and they almost certainly are the
    /// strings of the ResourceType, but I have not thoroughly checked yet.
    pub filter_type: String,

    /// This is the human-readable name of the folder which would appear in the IDE.
    ///
    /// Examples are "scripts", "Npc Sprites", etc. User facing.
    pub folder_name: String,

    /// There can be multiple "root" or "user" views, but the absolute Default view will have this
    /// flag set to true. All other "root" views created by users will not.
    pub is_default_view: bool,

    /// This field is used to aid in localization of the GMS2 IDE. Only certain values are allowed.
    pub localised_folder_name: LocalisedFolderName,
}

impl GmFolder {
    pub const MVC: &'static str = "1.1";
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, SmartDefault, PartialEq, Eq)]
pub enum ConstGmFolder {
    #[serde(rename = "GMFolder")]
    #[default]
    GmFolder,
}

impl From<ConstGmFolder> for ResourceType {
    fn from(_: ConstGmFolder) -> Self {
        Self::GmFolder
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum LocalisedFolderName {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "ResourceTree_Configs")]
    ResourceTreeConfigs,
    #[serde(rename = "ResourceTree_Extensions")]
    ResourceTreeExtensions,
    #[serde(rename = "ResourceTree_Fonts")]
    ResourceTreeFonts,
    #[serde(rename = "ResourceTree_IncludedFiles")]
    ResourceTreeIncludedFiles,
    #[serde(rename = "ResourceTree_Notes")]
    ResourceTreeNotes,
    #[serde(rename = "ResourceTree_Objects")]
    ResourceTreeObjects,
    #[serde(rename = "ResourceTree_Options")]
    ResourceTreeOptions,
    #[serde(rename = "ResourceTree_Paths")]
    ResourceTreePaths,
    #[serde(rename = "ResourceTree_Rooms")]
    ResourceTreeRooms,
    #[serde(rename = "ResourceTree_Scripts")]
    ResourceTreeScripts,
    #[serde(rename = "ResourceTree_Shaders")]
    ResourceTreeShaders,
    #[serde(rename = "ResourceTree_Sounds")]
    ResourceTreeSounds,
    #[serde(rename = "ResourceTree_Sprites")]
    ResourceTreeSprites,
    #[serde(rename = "ResourceTree_Tilesets")]
    ResourceTreeTilesets,
    #[serde(rename = "ResourceTree_Timelines")]
    ResourceTreeTimelines,
}
