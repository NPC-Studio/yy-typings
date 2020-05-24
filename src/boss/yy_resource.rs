use super::yy_typings::{FilesystemPath, ViewPath};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;

pub trait YyResource: Serialize + for<'de> Deserialize<'de> {
    type AssociatedData: std::fmt::Debug;

    /// Get's the resource's name.
    fn name(&self) -> &str;

    /// Sets the name of the resource.
    fn set_name(&mut self, name: String);

    /// Get the relative filepath from the directory of the YYP
    /// to the resource yy file. For a sprite called `spr_player`,
    /// that path would be `sprites/spr_player/spr_player.yy`.
    fn filesystem_path(&self) -> FilesystemPath;

    fn parent_path(&self) -> ViewPath;

    // fn ensure_associated_data_is_loaded(&mut self);

    fn serialize_associated_data(
        &self,
        directory_path: &Path,
        data: &Self::AssociatedData,
    ) -> Result<()>;
}
