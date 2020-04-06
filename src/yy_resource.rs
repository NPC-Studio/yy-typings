use super::yyp::YypResourceKeyId;
use std::path::PathBuf;

pub trait YyResource {
    fn relative_filepath(&self) -> PathBuf;
    fn yy_resource_id<T: Into<YypResourceKeyId>>(&self) -> T;
}
