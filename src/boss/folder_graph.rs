use super::yy_typings::{FilesystemPath, ViewPath};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct FolderGraph {
    pub name: String,
    pub path_to_parent: Option<ViewPath>,
    pub subfolders: Vec<FolderGraph>,
    pub files: Vec<FilesystemPath>,
}

impl FolderGraph {
    pub fn root() -> FolderGraph {
        FolderGraph {
            name: "folders".to_string(),
            path_to_parent: None,
            subfolders: vec![],
            files: vec![],
        }
    }

    pub fn new(name: String, parent: ViewPath) -> FolderGraph {
        FolderGraph {
            name,
            path_to_parent: Some(parent),
            subfolders: vec![],
            files: vec![],
        }
    }

    pub fn view_path(&self) -> ViewPath {
        ViewPath {
            name: self.name.to_string(),
            path: if let Some(parent_path) = &self.path_to_parent {
                parent_path.path.join(&self.name)
            } else {
                Path::new("folders").to_owned()
            },
        }
    }
}
