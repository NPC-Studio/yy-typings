use super::yyp::{YypResourceKeyId, ResourceType};
use std::path::PathBuf;

pub trait YyResource {
    /// Get the relative filepath from the directory of the YYP
    /// to the resource yy file. For a sprite called `spr_player`,
    /// that path would be `sprites/spr_player/spr_player.yy`.
    fn relative_filepath(&self) -> PathBuf;

    /// Convert a local Resource type, such as `SpriteId` into
    /// a `YypResourceKeyId`. Generally, each Guarded UUID should
    /// already implement it.
    fn yy_resource_id<T: Into<YypResourceKeyId>>(&self) -> T;

    fn yy_resource_type<T: Into<ResourceType>>(&self) -> T;
}
