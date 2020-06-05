use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Paths on the File System.
///
/// If a user joins `path` with a yyp's absolute folder location,
/// the resulting path will always point to a valid file.
///
/// `path` will **always** point to a file, and the file it points to will always
/// have a `.yy` extension. The `name` field and the `file_name` of the `path` will always
/// be equal to each other.
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct FilesystemPath {
    /// The human readable name of the parent. for a `spr_player`, this
    /// might correspond to `Sprites`.
    pub name: String,
    /// The direct path from the `.yyp` directory to the resource needed. This
    /// is not directly related to parentage at all, as GMS2 does not use the FileSystem
    /// for parentage.
    pub path: PathBuf,
}

/// Viewpaths in the virtual file system created by the Folders in the Yyp,
/// deliminated by `/`, and with a ViewPathLocation which ends in `.yy`.
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash)]
pub struct ViewPath {
    /// The human readable name of the parent. for a `spr_player`, this
    /// might correspond to `Sprites`.
    pub name: String,

    /// The path to the view member itself.
    pub path: ViewPathLocation,
}

/// The `path` component will **always** end with **.yy**, even if it describes
/// a virtual folder or file. Given the following Gms2 folder virtual system (*not* operating system file system):
/// ```txt
/// folders/
///     Sprites/
///         spr_enemy.yy
///         spr_player.yy
/// ```
/// In this case, we would see the following `ViewPath`s, in Json form:
/// ```json
/// [
///     { "name": "spr_enemy", "path": "folders/Sprites/spr_enemy.yy" },
///     { "name": "spr_player", "path": "folders/Sprites/spr_player.yy" }
/// ]
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct ViewPathLocation(pub String);

impl ViewPathLocation {
    pub fn inner(&self) -> &str {
        &self.0
    }

    pub fn root() -> ViewPathLocation {
        Self("folders".to_string())
    }
}
use std::fmt;
impl fmt::Display for ViewPathLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for ViewPathLocation {
    fn into(self) -> String {
        self.0
    }
}

/// A unqiue Id for textures. Although it appears as if it could support
/// hierarchies and nesting, **textures ids never actually show that in practice,
/// so this form is likely an artifact of the Yyg internal project structures.**.
/// The first member of this hierarchy is *always* `texturegroups`.
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash)]
pub struct TexturePath {
    /// The human readable name of the parent. For a texture group `Default`,
    /// this name would read `Default`.
    pub name: String,

    /// The path to the texture group, where:
    /// ```norun
    /// assert_eq!(self.path.0, format!("texturegroups/{}", self.name))
    /// ```
    pub path: TexturePathLocation,
}

/// The `path` component will **never** end with .yy, even if it describes
/// a virtual folder or file. This is to say, given the texture groups `Default`, `Crops`,
/// and `Enemies`, we would expect to see the following `TexturePath` vec, in Json:
/// ```json
/// [
///     { "name": "Default", path: "texturegroups/Default" },
///     { "name": "Crops", path: "texturegroups/Crops" },
///     { "name": "Enemies", path: "texturegroups/Enemies" }
/// ]
/// ```
///
/// There are two important things to note about `TextureGroup`:
/// 1. It does not end in `.yy`.
/// 2. It uses `/` as a separator.
/// 3. It always starts with `texturegroups`.
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct TexturePathLocation(pub String);

impl TexturePathLocation {
    /// Access the inner member as a reference.
    pub fn inner(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for TexturePathLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for TexturePathLocation {
    fn into(self) -> String {
        self.0
    }
}
