use super::{yyp::*, YyResource, YyResult};
use std::fs;
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
            path: path_to_yyp,
            dirty: false,
        })
    }

    pub fn add_new_resource(
        &mut self,
        new_resource: &impl YyResource,
        config_deltas: Option<Vec<String>>,
    ) -> YyResult<()> {
        // New Resource:
        let new_yy_resource = YypResource {
            key: new_resource.yy_resource_id(),
            value: YypResourceValue {
                config_deltas,
                id: YypResourceId::new(),
                resource_path: new_resource.relative_filepath().to_owned(),
                resource_type: new_resource.yy_resource_type(),
            },
        };

        // Save the file:
        let serialized = serde_json::to_string_pretty(new_resource)?;
        let new_path = self.path.join(new_resource.relative_filepath());
        fs::write(&new_path, &serialized)?;

        // save the additional file(s):
        if let Some(parent) = new_path.parent() {
            new_resource.serialize_additional_files(parent)?;
        }

        // Update the Resource
        self.yyp.resources.push(new_yy_resource);

        Ok(())
    }
}
