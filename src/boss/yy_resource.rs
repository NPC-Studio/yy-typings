use super::yy_typings::yyp::{ResourceType, YypResourceKeyId};
use anyhow::Result;
use std::path::{Path, PathBuf};

pub trait YyResource: serde::Serialize + for<'de> serde::Deserialize<'de> {
    type Key: std::hash::Hash + PartialEq + Eq + Into<YypResourceKeyId>;
    type AssociatedData: std::fmt::Debug;

    /// Get the relative filepath from the directory of the YYP
    /// to the resource yy file. For a sprite called `spr_player`,
    /// that path would be `sprites/spr_player/spr_player.yy`.
    fn relative_filepath(&self) -> PathBuf;

    /// Convert a local Resource Id, such as `SpriteId` into
    /// a `YypResourceKeyId`. Generally, each Guarded UUID should
    /// already implement it.
    fn id(&self) -> Self::Key;

    /// Convert each local resource type in the YyResource Type.
    /// Mostly, the types themselves should implement this.
    fn yy_resource_type(&self) -> ResourceType;

    fn serialize_associated_data(
        &self,
        directory_path: &Path,
        data: &Self::AssociatedData,
    ) -> Result<()>;
}
