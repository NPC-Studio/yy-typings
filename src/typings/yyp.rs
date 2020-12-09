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

    /// Allows for experimental JS editing. Unfinished or legacy feature. It's a secret.
    pub is_ecma: bool,

    /// Unknown property, usually an empty string.
    pub tutorial_path: String,

    /// Lists all known configs. Note that this top level
    /// config will **always** have the `name` `"Default"`.
    pub configs: YypConfig,

    /// This is the order rooms are loaded in. The first room
    /// is the default room which GMS2 will load on GameStart.
    #[serde(rename = "RoomOrderNodes")]
    pub room_order: Vec<RoomOrderId>,
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
    pub const DEFAULT_VERSION: &'static str = "2.3.1.536";
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

/// A YYP Folder. These form a graph, but **each path is a full path from the root**.
/// Therefore, to create a tree, one must walk from the root to the final destination.
#[derive(
    Debug, Serialize, Deserialize, Eq, Clone, SmartDefault, Ord, PartialOrd, PartialEq, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct YypFolder {
    /// The full path from the root to the virtual folder location. The first
    /// part of the path is always `folders`. For top level folders, will look like
    /// `"Folders/Fonts.yy"`, for example.
    pub folder_path: ViewPathLocation,

    /// The order within the subfolder. If custom ordering is added, then this will be the order as
    /// the resources appear within the tree structure. Otherwise, it is meaningless, and Gms2 appears
    /// to not keep in tracked or coherent.
    pub order: usize,

    /// The resource version of this Resource. Currently `"1.0"`.
    pub resource_version: ResourceVersion,
    /// The human-readable name of this Folder. The last part of the `folder_path` and this name
    /// should agree. Human readable names include examples such as "Sprites", "Light Data", or
    /// "Really Good Tiles".
    pub name: String,

    /// Apparently tags can be placed here, even though they definitely can't. Don't do that.
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

// #[cfg(test)]
// mod tests {
//     use crate::{
//         texture_group::TextureGroup, utils::TrailingCommaUtility, AudioGroup, FilesystemPath,
//         RoomOrderId, ViewPathLocation, Yyp, YypConfig, YypFolder, YypIncludedFile, YypMetaData,
//         YypResource,
//     };
//     use include_dir::{include_dir, Dir, DirEntry};
//     use pretty_assertions::assert_eq;
//     use std::path::Path;

//     #[test]
//     fn trivial_yyp_parse() {
//         let all_yyps: Dir = include_dir!("data/yyps");
//         let tcu = TrailingCommaUtility::new();

//         for sprite_file in all_yyps.find("**/*.yyp").unwrap() {
//             if let DirEntry::File(file) = sprite_file {
//                 let our_str = std::str::from_utf8(file.contents()).unwrap();
//                 let our_str = tcu.clear_trailing_comma(our_str);
//                 let _: Yyp = serde_json::from_str(&our_str).unwrap();
//             }
//         }
//     }

//     #[test]
//     fn deep_compare() {
//         let test0_yyp = TrailingCommaUtility::clear_trailing_comma_once(include_str!(
//             "./../../data/yyps/yyp_test0.yyp"
//         ));

//         let test0_yyp: Yyp = serde_json::from_str(&test0_yyp).unwrap();

//         let test_yyp: Yyp = Yyp {
//             resources: vec![
//                 YypResource {
//                     id: FilesystemPath {
//                         name: "Test4".to_string(),
//                         path: Path::new(&format!("objects/{0}/{0}.yy", "Test4")).to_owned(),
//                     },
//                     order: 0,
//                 },
//                 YypResource {
//                     id: FilesystemPath {
//                         name: "Object2".to_string(),
//                         path: Path::new(&format!("objects/{0}/{0}.yy", "Object2")).to_owned(),
//                     },
//                     order: 1,
//                 },
//                 YypResource {
//                     id: FilesystemPath {
//                         name: "Room2".to_string(),
//                         path: Path::new(&format!("rooms/{0}/{0}.yy", "Room2")).to_owned(),
//                     },
//                     order: 1,
//                 },
//                 YypResource {
//                     id: FilesystemPath {
//                         name: "Room1".to_string(),
//                         path: Path::new(&format!("rooms/{0}/{0}.yy", "Room1")).to_owned(),
//                     },
//                     order: 0,
//                 },
//             ],
//             options: vec![
//                 FilesystemPath {
//                     name: "Amazon Fire".to_string(),
//                     path: Path::new("options/amazonfire/options_amazonfire.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "Android".to_string(),
//                     path: Path::new("options/android/options_android.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "HTML5".to_string(),
//                     path: Path::new("options/html5/options_html5.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "iOS".to_string(),
//                     path: Path::new("options/ios/options_ios.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "Linux".to_string(),
//                     path: Path::new("options/linux/options_linux.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "macOS".to_string(),
//                     path: Path::new("options/mac/options_mac.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "Main".to_string(),
//                     path: Path::new("options/main/options_main.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "PlayStation 4".to_string(),
//                     path: Path::new("options/ps4/options_ps4.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "Switch".to_string(),
//                     path: Path::new("options/switch/options_switch.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "tvOS".to_string(),
//                     path: Path::new("options/tvos/options_tvos.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "Windows".to_string(),
//                     path: Path::new("options/windows/options_windows.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "Windows UWP".to_string(),
//                     path: Path::new("options/windowsuap/options_windowsuap.yy").to_owned(),
//                 },
//                 FilesystemPath {
//                     name: "Xbox One".to_string(),
//                     path: Path::new("options/xboxone/options_xboxone.yy").to_owned(),
//                 },
//             ],
//             is_dn_d_project: false,
//             is_ecma: false,
//             tutorial_path: String::new(),
//             configs: YypConfig {
//                 name: "Default".to_string(),
//                 children: vec![
//                     YypConfig {
//                         name: "TestConfig".to_string(),
//                         children: vec![
//                             YypConfig {
//                                 name: "SubChild".to_string(),
//                                 children: vec![],
//                             },
//                             YypConfig {
//                                 name: "NewConfig1".to_string(),
//                                 children: vec![],
//                             },
//                         ],
//                     },
//                     YypConfig {
//                         name: "TestConfig2".to_string(),
//                         children: vec![],
//                     },
//                 ],
//             },
//             room_order: vec![
//                 RoomOrderId {
//                     room_id: FilesystemPath {
//                         name: "Room1".to_string(),
//                         path: Path::new(&format!("rooms/{0}/{0}.yy", "Room1")).to_owned(),
//                     },
//                 },
//                 RoomOrderId {
//                     room_id: FilesystemPath {
//                         name: "Room2".to_string(),
//                         path: Path::new(&format!("rooms/{0}/{0}.yy", "Room2")).to_owned(),
//                     },
//                 },
//             ],
//             folders: vec![
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Sprites.yy".to_string()),
//                     name: "Sprites".to_owned(),
//                     order: 1,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Tile Sets.yy".to_string()),
//                     name: "Tile Sets".to_owned(),
//                     order: 2,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Sounds.yy".to_string()),
//                     name: "Sounds".to_owned(),
//                     order: 3,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Paths.yy".to_string()),
//                     name: "Paths".to_owned(),
//                     order: 4,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Scripts.yy".to_string()),
//                     name: "Scripts".to_owned(),
//                     order: 5,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Shaders.yy".to_string()),
//                     name: "Shaders".to_owned(),
//                     order: 6,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Fonts.yy".to_string()),
//                     name: "Fonts".to_owned(),
//                     order: 7,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Timelines.yy".to_string()),
//                     name: "Timelines".to_owned(),
//                     order: 8,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Objects.yy".to_string()),
//                     name: "Objects".to_owned(),
//                     order: 9,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Rooms.yy".to_string()),
//                     name: "Rooms".to_owned(),
//                     order: 10,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Sequences.yy".to_string()),
//                     name: "Sequences".to_owned(),
//                     order: 11,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Animation Curves.yy".to_string()),
//                     name: "Animation Curves".to_owned(),
//                     order: 12,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Notes.yy".to_string()),
//                     name: "Notes".to_owned(),
//                     order: 13,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/Extensions.yy".to_string()),
//                     name: "Extensions".to_owned(),
//                     order: 14,
//                     ..YypFolder::default()
//                 },
//                 YypFolder {
//                     folder_path: ViewPathLocation("folders/group1.yy".to_string()),
//                     name: "group1".to_owned(),
//                     order: 15,
//                     ..YypFolder::default()
//                 },
//             ],
//             audio_groups: vec![AudioGroup::default()],
//             texture_groups: vec![TextureGroup::default()],
//             included_files: vec![
//                 YypIncludedFile {
//                     name: ".DS_Store".to_owned(),
//                     ..YypIncludedFile::default()
//                 },
//                 YypIncludedFile {
//                     name: "main.rtf".to_owned(),
//                     ..YypIncludedFile::default()
//                 },
//                 YypIncludedFile {
//                     name: "subfolder.rtf".to_owned(),
//                     file_path: Path::new("datafiles/test").to_owned(),
//                     ..YypIncludedFile::default()
//                 },
//                 YypIncludedFile {
//                     name: ".DS_Store".to_owned(),
//                     file_path: Path::new("datafiles/test").to_owned(),
//                     ..YypIncludedFile::default()
//                 },
//             ],
//             meta_data: YypMetaData::default(),
//             name: "test2".to_string(),
//             ..Yyp::default()
//         };

//         assert_eq!(test0_yyp, test_yyp);
//     }
// }
