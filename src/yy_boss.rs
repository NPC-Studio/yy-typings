use super::{yyp::*, YyResult};
use std::path::PathBuf;

#[derive(Debug)]
pub struct YyBoss {
    yyp: Yyp,
    dirty: bool,
    path: PathBuf,
}

impl YyBoss {
    pub fn new(path_to_yyp: PathBuf) -> YyResult<YyBoss> {
        let yy_file = std::fs::read_to_string(&path_to_yyp)?;
        let yyp: Yyp = serde_json::from_str(&yy_file)?;

        Ok(Self {
            yyp,
            dirty: false,
            path: path_to_yyp,
        })
    }

    // pub fn add_new_resource(new_resource: &)
}
