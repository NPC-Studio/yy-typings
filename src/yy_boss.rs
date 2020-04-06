use super::{yyp::*, YyResource, YyResult};
use std::path::PathBuf;

#[derive(Debug)]
pub struct YyBoss {
    yyp: Yyp,
    dirty: bool,
    path: PathBuf,
    dirty_resources: Vec<YypResourceId>,
}

impl YyBoss {
    pub fn new(path_to_yyp: PathBuf) -> YyResult<YyBoss> {
        let yy_file = std::fs::read_to_string(&path_to_yyp)?;
        let yyp: Yyp = serde_json::from_str(&yy_file)?;

        Ok(Self {
            yyp,
            path: path_to_yyp,
            dirty: false,
            dirty_resources: vec![],
        })
    }

    pub fn add_new_resource(
        &mut self,
        new_resource: &impl YyResource,
        config_deltas: Option<Vec<String>>,
    ) {
        let new_resource = YypResource {
            key: new_resource.yy_resource_id(),
            value: YypResourceValue {
                config_deltas,
                id: YypResourceId::new(),
                resource_path: new_resource.relative_filepath(),
                resource_type: new_resource.yy_resource_type(),
            },
        };
        
        // Mark it as Dirty...
        self

        // Update the Resource
        self.yyp.resources.push(new_resource);
        
    }
}
