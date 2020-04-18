use super::{
    folder_graph::LeafId,
    yy_typings::{
        resources::{folder::*, ResourceType},
        yyp::YypResourceKeyId,
    },
    YyResource,
};

pub trait GmFolderExt {
    fn new(folder_name: String) -> Self;
    fn new_with_id(folder_name: String, game_folder_id: GmFolderId) -> Self;
    fn filter_type(self, filter_type: String) -> Self;
    fn is_default_view(self, is_default: bool) -> Self;
    fn root_folder(self) -> Self;
    fn user_folder(self) -> Self;
    fn localised_folder_name(self, localized_folder_name: LocalisedFolderName) -> Self;
    fn child(self, child: YypResourceKeyId) -> Self;
    fn children(self, children: &[YypResourceKeyId]) -> Self;
}

impl GmFolderExt for GmFolder {
    fn new(folder_name: String) -> Self {
        let id = GmFolderId::new();
        Self {
            id,
            model_name: ConstGmFolder::GmFolder,
            mvc: GmFolder::MVC.to_string(),
            name: id,
            children: vec![],
            filter_type: String::new(),
            folder_name,
            is_default_view: false,
            localised_folder_name: LocalisedFolderName::Empty,
        }
    }
    fn new_with_id(folder_name: String, id: GmFolderId) -> Self {
        Self {
            id,
            model_name: ConstGmFolder::GmFolder,
            mvc: GmFolder::MVC.to_string(),
            name: id,
            children: vec![],
            filter_type: String::new(),
            folder_name,
            is_default_view: false,
            localised_folder_name: LocalisedFolderName::Empty,
        }
    }
    fn filter_type(mut self, filter_type: String) -> Self {
        self.filter_type = filter_type;
        self
    }
    fn is_default_view(mut self, is_default: bool) -> Self {
        self.is_default_view = is_default;
        self
    }
    fn root_folder(mut self) -> Self {
        self.filter_type = "root".to_string();
        self.is_default_view = true;
        self
    }
    fn user_folder(mut self) -> Self {
        self.filter_type = "user".to_string();
        self.is_default_view = false;
        self
    }

    fn localised_folder_name(mut self, localized_folder_name: LocalisedFolderName) -> Self {
        self.localised_folder_name = localized_folder_name;
        self
    }

    fn child(mut self, child: YypResourceKeyId) -> Self {
        self.children.push(child);
        self
    }

    fn children(mut self, children: &[YypResourceKeyId]) -> Self {
        for child in children.into_iter() {
            self.children.push(*child);
        }

        self
    }
}

use std::path::{Path, PathBuf};
impl YyResource for GmFolder {
    fn name(&self) -> &str {
        &self.folder_name
    }

    fn set_name(&mut self, name: String) {
        self.folder_name = name;
    }

    fn relative_filepath(&self) -> PathBuf {
        Path::new(&format!("views/{name}.yy", name = self.name.inner())).to_owned()
    }

    fn id(&self) -> GmFolderId {
        self.id
    }

    fn yy_resource_type(&self) -> ResourceType {
        self.model_name.into()
    }

    fn serialize_associated_data(&self, _: &Path, _: &Self::AssociatedData) -> anyhow::Result<()> {
        Ok(())
    }

    type Key = GmFolderId;
    type AssociatedData = LeafId;
}

impl Into<YypResourceKeyId> for GmFolderId {
    fn into(self) -> YypResourceKeyId {
        YypResourceKeyId::with_id(self.inner())
    }
}

impl Into<GmFolderId> for YypResourceKeyId {
    fn into(self) -> GmFolderId {
        GmFolderId::with_id(self.inner())
    }
}
