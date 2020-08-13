use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

/// Paths on the File System.
///
/// If a user joins `path` with a yyp's absolute folder location,
/// the resulting path will always point to a valid file.
///
/// `path` will **always** point to a file, and the file it points to will always
/// have a `.yy` extension. The `name` field and the `file_name` of the `path` will always
/// be equal to each other.
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct FilesystemPath {
    /// The name of the resource which the Path points to. For an `spr_sprite`, for example, it will
    /// be `spr_sprite`.
    pub name: String,
    /// The direct path from the `.yyp` directory to the resource needed on the file system.
    pub path: PathBuf,
}

impl FilesystemPath {
    /// Constructs a new path out of a `base` and a `name`.
    pub fn new(base: &str, name: &str) -> FilesystemPath {
        Self {
            name: name.to_owned(),
            path: FilesystemPath::new_path(base, name),
        }
    }

    /// Constructs a new subpath out of a `base` and a `name`.
    ///
    /// This creates a path of the same form as a FilesystemPath would have, which will be a relative
    /// path from the Yyp directory to the File.
    pub fn new_path(base: &str, name: &str) -> PathBuf {
        Path::new(&format!(
            "{base}/{name}/{name}.yy",
            base = base,
            name = name
        ))
        .to_owned()
    }
}

/// Viewpaths in the virtual file system created by the Folders in the Yyp,
/// deliminated by `/`, and with a ViewPathLocation which ends in `.yy`.\ or in `.yyp`.
///
/// Please note, the `Default` implementation in this type is **never a valid option**. It is
/// provided for convenience, but the default Root for any file not in a folder is as follows:
/// ```
/// # use yy_typings::{ViewPathLocation, ViewPath};
/// # const PROJECT_NAME: &'static str = "A project Name";
/// let root_path = ViewPath {
///     name: PROJECT_NAME.to_string(),
///     path: ViewPathLocation(format!("{}.yyp", PROJECT_NAME))
/// };
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash, PartialOrd, Ord)]
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
///
/// **NB** the default provided here, a Blank String, is never correct in a Gms2 Project. Instead,
/// to describe a file at the root of the project (ie, a file not inside a folder), please use:
/// ```
/// # use yy_typings::ViewPathLocation;
/// # const PROJECT_NAME: &'static str = "A project Name";
/// ViewPathLocation(format!("{}.yyp", PROJECT_NAME));
/// ```
#[derive(Serialize, Deserialize, Default, Debug, Eq, PartialEq, Clone, Hash, Ord, PartialOrd)]
pub struct ViewPathLocation(pub String);

impl ViewPathLocation {
    pub fn inner(&self) -> &str {
        &self.0
    }

    pub fn root_folder() -> ViewPathLocation {
        Self("folders".to_string())
    }

    pub fn root_file(project_name: &str) -> ViewPathLocation {
        Self(format!("{}.yyp", project_name))
    }
}
use std::fmt;
impl fmt::Display for ViewPathLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output: &str = self
            .0
            .trim_start_matches("folders/")
            .trim_end_matches(".yy");

        write!(f, "{}", output)
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
    /// ```no run
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
