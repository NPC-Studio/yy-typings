use super::yyp::{ResourceType, YypResourceKeyId};
use std::path::{Path, PathBuf};

pub trait YyResource: serde::Serialize + for<'de> serde::Deserialize<'de> {
    /// Get the relative filepath from the directory of the YYP
    /// to the resource yy file. For a sprite called `spr_player`,
    /// that path would be `sprites/spr_player/spr_player.yy`.
    fn relative_filepath(&self) -> PathBuf;

    /// Convert a local Resource type, such as `SpriteId` into
    /// a `YypResourceKeyId`. Generally, each Guarded UUID should
    /// already implement it.
    fn yy_resource_id(&self) -> YypResourceKeyId;

    fn yy_resource_type(&self) -> ResourceType;

    fn serialize_additional_files(&self, _directory_path: &Path) -> super::YyResult<()> {
        Ok(())
    }
}
