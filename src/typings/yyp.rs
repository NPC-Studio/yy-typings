use super::{texture_group::TextureGroup, AudioGroup, FilesystemPath, Tags, ViewPathLocation};
use crate::ResourceVersion;
use serde::{Deserialize, Serialize};
use smart_default::SmartDefault;
use std::{
    hash::Hash,
    path::{Path, PathBuf},
};

/// GMS2 project file typings
#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct Yyp {
    /// Contains all project resources, ordered by KeyID.
    pub resources: Vec<YypResource>,

    #[serde(rename = "Options")]
    pub options: Vec<FilesystemPath>,

    /// Denotes whether this project uses drag and drop or not
    pub is_dn_d_project: bool,

    /// Allows for experimental JS editing. Unfinished or legacy feature. It's a
    /// secret.
    pub is_ecma: bool,

    /// Unknown property, usually an empty string.
    pub tutorial_path: String,

    /// Lists all known configs. Note that this top level
    /// config will **always** have the `name` `"Default"`.
    pub configs: YypConfig,

    /// This is the order rooms are loaded in. The first room
    /// is the default room which GMS2 will load on GameStart.
    #[serde(rename = "RoomOrderNodes")]
    pub room_order_nodes: Vec<RoomOrderId>,
    /// This represents all the Views in the Project, which will
    /// have resource paths within them.
    #[serde(rename = "Folders")]
    pub folders: Vec<YypFolder>,

    /// The Audio Groups present within the project. Relationship to
    /// the inherited.yy is unclear
    #[serde(rename = "AudioGroups")]
    pub audio_groups: Vec<AudioGroup>,

    /// The Texture groups present within the project. Relationship to
    /// the inherited.yy is unclear
    #[serde(rename = "TextureGroups")]
    pub texture_groups: Vec<TextureGroup>,

    /// The included files within the projects.
    #[serde(rename = "IncludedFiles")]
    pub included_files: Vec<YypIncludedFile>,

    /// The MetaData for the project.
    #[serde(rename = "MetaData")]
    pub meta_data: YypMetaData,

    /// The version of the YYP. Currently, that is "1.4"
    #[default(ResourceVersion::new(1, 4))]
    pub resource_version: ResourceVersion,

    /// The actual human-readable name of the Project, such as "Forager"
    /// or "Fields of Mistria" or "Test1122 please work".
    pub name: String,

    /// Somehow, the Tags field, which exists purely due to OOP, I assume.
    /// It should always be empty and does nothing.
    pub tags: Tags,

    /// The ResourceType of the YYP, which is "GMProject"
    pub resource_type: ConstGmProject,
}

impl Yyp {
    pub const DEFAULT_VERSION: &'static str = "3.6.595";
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
pub struct YypMetaData {
    #[serde(rename = "IDEVersion")]
    #[default(Yyp::DEFAULT_VERSION.to_string())]
    pub ide_version: String,
}

/// Represents a resource entry in a YYP
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, Hash, Ord, PartialOrd)]
pub struct YypResource {
    /// This is the path to the Filesystem
    pub id: FilesystemPath,
    /// This is the order of the resource, perhaps within a folder?
    /// Unclear.
    pub order: usize,
}

/// A description of a Config. Note that Configs form
/// an acyclical graph by their children, so this tree could get quite large.
///
/// The first node within the YypConfig tree is **always** "Default".
/// It may have no children.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
pub struct YypConfig {
    #[default("Default".to_string())]
    pub name: String,
    pub children: Vec<YypConfig>,
}

/// A YYP Folder. These form a graph, but **each path is a full path from the
/// root**. Therefore, to create a tree, one must walk from the root to the
/// final destination.
#[derive(
    Debug, Serialize, Deserialize, Eq, Clone, SmartDefault, Ord, PartialOrd, PartialEq, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct YypFolder {
    /// The full path from the root to the virtual folder location. The first
    /// part of the path is always `folders`. For top level folders, will look
    /// like `"Folders/Fonts.yy"`, for example.
    pub folder_path: ViewPathLocation,

    /// The order within the subfolder. If custom ordering is added, then this
    /// will be the order as the resources appear within the tree structure.
    /// Otherwise, it is meaningless, and Gms2 appears to not keep in
    /// tracked or coherent.
    pub order: usize,

    /// The resource version of this Resource. Currently `"1.0"`.
    pub resource_version: ResourceVersion,
    /// The human-readable name of this Folder. The last part of the
    /// `folder_path` and this name should agree. Human readable names
    /// include examples such as "Sprites", "Light Data", or "Really Good
    /// Tiles".
    pub name: String,

    /// Apparently tags can be placed here, even though they definitely can't.
    /// Don't do that.
    pub tags: Tags,

    /// The Resource Type of this folder, which is always `"GMFolder"`.
    pub resource_type: ConstGmFolder,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct YypIncludedFile {
    #[serde(rename = "CopyToMask")]
    #[default(-1)]
    pub copy_to_mask: isize,
    #[default(Path::new("datafiles").to_owned())]
    pub file_path: PathBuf,
    pub resource_version: ResourceVersion,
    pub name: String,
    pub resource_type: ConstGmIncludedFile,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct RoomOrderId {
    pub room_id: FilesystemPath,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, SmartDefault)]
pub enum ConstGmProject {
    #[serde(rename = "GMProject")]
    #[default]
    Const,
}

#[derive(
    Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, SmartDefault, Ord, PartialOrd,
)]
pub enum ConstGmFolder {
    #[serde(rename = "GMFolder")]
    #[default]
    Const,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Copy, SmartDefault)]
pub enum ConstGmIncludedFile {
    #[serde(rename = "GMIncludedFile")]
    #[default]
    Const,
}
