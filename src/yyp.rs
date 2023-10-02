use super::{texture_group::TextureGroup, AudioGroup, FilesystemPath, ViewPathLocation};
use serde::{Deserialize, Serialize};
use serde_json::ser::CompactFormatter;
use smart_default::SmartDefault;
use std::{
    hash::Hash,
    path::{Path, PathBuf},
};

/// GMS2 project file typings
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct Yyp {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Project, String, 1, 7>,

    /// The Audio Groups present within the project. Relationship to
    /// the inherited.yy is unclear
    #[serde(rename = "AudioGroups")]
    pub audio_groups: Vec<AudioGroup>,

    /// Lists all known configs. Note that this top level
    /// config will **always** have the `name` `"Default"`.
    pub configs: YypConfig,

    /// Denotes whether this project uses drag and drop or not
    pub default_script_type: i32,

    /// This represents all the Views in the Project, which will
    /// have resource paths within them.
    #[serde(rename = "Folders")]
    pub folders: Vec<YypFolder>,

    /// The included files within the projects.
    #[serde(rename = "IncludedFiles")]
    pub included_files: Vec<YypIncludedFile>,

    /// Allows for experimental JS editing. Unfinished or legacy feature. It's a
    /// secret.
    pub is_ecma: bool,

    /// Not entirely sure what this is -- probably for their upcoming library work.
    #[serde(rename = "LibraryEmitters")]
    pub library_emitters: Vec<serde_json::Value>,

    /// The MetaData for the project.
    #[serde(rename = "MetaData")]
    pub meta_data: YypMetaData,

    /// Contains all project resources, ordered by KeyID.
    pub resources: Vec<YypResource>,

    /// This is the order rooms are loaded in. The first room
    /// is the default room which GMS2 will load on GameStart.
    #[serde(rename = "RoomOrderNodes")]
    pub room_order_nodes: Vec<RoomOrderId>,

    /// The Texture groups present within the project. Relationship to
    /// the inherited.yy is unclear
    #[serde(rename = "TextureGroups")]
    pub texture_groups: Vec<TextureGroup>,
}

impl Yyp {
    pub const DEFAULT_VERSION: &'static str = "2023.6.0.92";
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
}

/// A description of a Config. Note that Configs form
/// an acyclical graph by their children, so this tree could get quite large.
///
/// The first node within the YypConfig tree is **always** "Default".
/// It may have no children.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
pub struct YypConfig {
    pub children: Vec<YypConfig>,
    #[default("Default".to_string())]
    pub name: String,
}

/// A YYP Folder. These form a graph, but **each path is a full path from the
/// root**. Therefore, to create a tree, one must walk from the root to the
/// final destination.
#[derive(
    Debug, Serialize, Deserialize, Eq, Clone, SmartDefault, Ord, PartialOrd, PartialEq, Hash,
)]
#[serde(rename_all = "camelCase")]
pub struct YypFolder {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::Folder>,

    /// The full path from the root to the virtual folder location. The first
    /// part of the path is always `folders`. For top level folders, will look
    /// like `"Folders/Fonts.yy"`, for example.
    pub folder_path: ViewPathLocation,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct YypIncludedFile {
    #[serde(flatten)]
    pub common_data: crate::CommonData<consts::IncludedFile>,
    #[serde(rename = "CopyToMask")]
    #[default(-1)]
    pub copy_to_mask: isize,
    #[default(Path::new("datafiles").to_owned())]
    pub file_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone, SmartDefault)]
#[serde(rename_all = "camelCase")]
pub struct RoomOrderId {
    pub room_id: FilesystemPath,
}

gm_const!(
    Project -> "GMProject",
    Folder -> "GMFolder",
    IncludedFile -> "GMIncludedFile"
);

use std::io;

#[derive(Debug)]
pub struct YypFormatter {
    current_indent: usize,
    has_value: bool,
    indent: &'static [u8],
}

impl Default for YypFormatter {
    fn default() -> Self {
        Self::new(b"  ")
    }
}

impl YypFormatter {
    pub fn new(indent: &'static [u8]) -> Self {
        Self {
            current_indent: 0,
            has_value: false,
            indent,
        }
    }

    fn indent<W>(&self, wr: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        for _ in 0..self.current_indent {
            wr.write_all(self.indent)?;
        }

        Ok(())
    }

    fn use_compact(&self)
}

impl serde_json::ser::Formatter for YypFormatter {
    #[inline]
    fn begin_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"[")
    }

    #[inline]
    fn end_array<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        self.current_indent -= 1;

        if self.has_value {
            writer.write_all(b"\n")?;
            self.indent(writer)?;
        }

        writer.write_all(b"]")
    }

    #[inline]
    fn begin_array_value<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_array_value(writer, first)?;

            return Ok(());
        }
        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        self.indent(writer)?;
        Ok(())
    }

    #[inline]
    fn end_array_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.end_array_value(writer)?;

            return Ok(());
        }

        self.has_value = true;
        Ok(())
    }

    #[inline]
    fn begin_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object(writer)?;

            return Ok(());
        }

        self.current_indent += 1;
        self.has_value = false;
        writer.write_all(b"{")
    }

    #[inline]
    fn end_object<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.end_object(writer)?;

            return Ok(());
        }

        self.current_indent -= 1;

        if self.has_value {
            writer.write_all(b"\n")?;
            self.indent(writer)?;
        }

        writer.write_all(b"}")
    }

    #[inline]
    fn begin_object_key<W>(&mut self, writer: &mut W, first: bool) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_key(writer, first)?;

            return Ok(());
        }

        if first {
            writer.write_all(b"\n")?;
        } else {
            writer.write_all(b",\n")?;
        }
        self.indent(writer)?;

        Ok(())
    }

    fn end_object_key<W>(&mut self, _writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        Ok(())
    }

    #[inline]
    fn begin_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.begin_object_value(writer)?;

            return Ok(());
        }
        writer.write_all(b": ")
    }

    #[inline]
    fn end_object_value<W>(&mut self, writer: &mut W) -> io::Result<()>
    where
        W: ?Sized + io::Write,
    {
        if self.use_compact() {
            CompactFormatter.end_object_value(writer)?;

            return Ok(());
        }
        self.has_value = true;
        Ok(())
    }
}

fn serialize_yyp(value: &Yyp) -> String {
    let mut writer = Vec::with_capacity(128);
    let mut ser = serde_json::ser::Serializer::with_formatter(&mut writer, YypFormatter::default());
    value.serialize(&mut ser).unwrap();

    unsafe {
        // We do not emit invalid UTF-8.
        String::from_utf8_unchecked(writer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t() {
        let x = include_str!("../../../Gms2/SwordAndField/FieldsOfMistria.yyp");
        let x = crate::TrailingCommaUtility::clear_trailing_comma_once(x);
        let json: Yyp = serde_json::from_str(x.as_ref()).unwrap();

        let o = serialize_yyp(&json);

        panic!("{}", o);
    }
}
