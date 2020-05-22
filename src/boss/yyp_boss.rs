use super::{
    utils::TrailingCommaUtility,
    yy_typings::{sprite::*, Yyp},
    SpriteImageBuffer, YyResource,
};
use anyhow::{format_err, Context, Result};
use log::info;
use std::collections::HashMap;
use std::{
    collections::HashSet,
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct YypBoss {
    yyp: Yyp,
    absolute_path: PathBuf,
    sprites: YyResourceHandler<Sprite>,
    pub folder_graph: FolderGraph,
    resource_names: HashSet<String>,
    tcu: TrailingCommaUtility,
    dirty: bool,
}

impl YypBoss {
    /// Creates a new YyBoss Manager and performs startup file reading.
    pub fn new(path_to_yyp: &Path) -> Result<YypBoss> {
        let tcu = TrailingCommaUtility::new();
        let yyp_file = fs::read_to_string(&path_to_yyp)
            .with_context(|| format!("Path given: {:?}", path_to_yyp))?;
        let yyp: Yyp = serde_json::from_str(&tcu.clear_trailing_comma(&yyp_file))?;

        let mut yyp_boss = Self {
            yyp,
            absolute_path: path_to_yyp.to_owned(),
            dirty: false,
            sprites: YyResourceHandler::new(),
            folder_graph: FolderGraph::root(),
            resource_names: HashSet::new(),
            tcu,
        };

        // Load in Folders
        for new_folder in yyp_boss.yyp.folders.iter() {
            let mut folder_graph = &mut yyp_boss.folder_graph;

            for section in new_folder.folder_path.iter().skip(1) {
                let section_name = section.to_string_lossy();
                let section_name = if section_name.ends_with(".yy") {
                    new_folder.name.clone()
                } else {
                    section_name.to_string()
                };

                let maybe_subfolder = folder_graph
                    .subfolders
                    .iter_mut()
                    .find(|sf| sf.name == section_name);

                if maybe_subfolder.is_none() {
                    folder_graph.subfolders.push(FolderGraph::new(section_name));
                    folder_graph = folder_graph.subfolders.last_mut().unwrap();
                } else {
                    folder_graph = folder_graph
                        .subfolders
                        .iter_mut()
                        .find(|sf| sf.name == section_name)
                        .unwrap();
                }
            }
        }

        // Load in Sprites
        for sprite_resource in yyp_boss
            .yyp
            .resources
            .iter()
            .filter(|value| value.id.path.starts_with("sprites"))
        {
            let sprite_resource: &YypResource = sprite_resource;
            let sprite_path = yyp_boss
                .absolute_path
                .parent()
                .unwrap()
                .join(&sprite_resource.id.path);

            let sprite_yy: Sprite = yyp_boss
                .deserialize_yyfile(&sprite_path)
                .with_context(|| format!("Error importing sprite with Path {:#?}", sprite_path))?;

            let frame_buffers: Vec<_> = sprite_yy
                .frames
                .iter()
                .filter_map(|frame: &Frame| {
                    let path_to_image = sprite_path
                        .parent()
                        .unwrap()
                        .join(Path::new(&frame.name.inner().to_string()).with_extension("png"));

                    match image::open(&path_to_image) {
                        Ok(image) => Some((frame.name, image.to_rgba())),
                        Err(e) => {
                            log::error!("We couldn't read {:?} -- {}", path_to_image, e);
                            None
                        }
                    }
                })
                .collect();

            YypBoss::add_file_iteratable(
                &mut yyp_boss.folder_graph,
                sprite_yy.parent.clone(),
                sprite_yy.filesystem_path(),
            )?;
            yyp_boss.resource_names.insert(sprite_yy.name.clone());
            yyp_boss.sprites.add_new_startup(sprite_yy, frame_buffers);
        }

        Ok(yyp_boss)
    }

    pub fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    /// Add a sprite into the YYP Boss. It is not immediately serialized,
    /// but will be serialized the next time the entire YYP Boss is.
    ///
    /// Please note -- the name of the Sprite MIGHT change if that name already exists!
    pub fn add_sprite(
        &mut self,
        _sprite: Sprite,
        _associated_data: Vec<(FrameId, SpriteImageBuffer)>,
    ) {
        todo!()
        // let sprite_id = sprite.id;
        // self.add_new_resource(&mut sprite, None);
        // self.sprites.add_new(sprite, associated_data);

        // self.append_under_folder(folder_id, sprite_id);
    }

    pub fn add_folder(&mut self, folder_name: String, parent: Option<String>) {
        // let id = folder.id;

        // // We add it "unnamed" because folder names can be duplicates of each other,
        // // so we don't want to track their names.
        // self.add_new_unnamed_resource(&folder, None);
        // let leaf_id = self.append_under_folder(folder_id, id);

        // self.folders.add_new(folder, leaf_id);
    }

    /// Adds a file to the Virtual File System.
    pub fn add_file(
        &mut self,
        parent_path: ViewPath,
        filesystem_path: FilesystemPath,
    ) -> Result<()> {
        YypBoss::add_file_iteratable(&mut self.folder_graph, parent_path, filesystem_path)
    }

    /// Stupid ass borrow checker
    fn add_file_iteratable(
        mut folder: &mut FolderGraph,
        parent_path: ViewPath,
        filesystem_path: FilesystemPath,
    ) -> Result<()> {
        for path in parent_path.path.iter().skip(1) {
            let path_name = path.to_string_lossy();
            let path_name = if let Some(pos) = path_name.find(".yy") {
                std::borrow::Cow::Borrowed(&path_name[..pos])
            } else {
                path_name
            };

            folder = folder
                .subfolders
                .iter_mut()
                .find(|sf| sf.name == path_name)
                .ok_or_else(|| format_err!("Couldn't find subfolder {}", path_name))?;
        }
        folder.files.push(filesystem_path);

        Ok(())
    }

    /// Adds a new Resource to be tracked by the YYP. The Resource also will
    /// need to serialize themselves and any additional files which they manage.
    ///
    /// This might include serializing sprites or sprite frames for Sprites, or `.gml`
    /// files for scripts or objects.
    #[allow(dead_code)]
    fn add_new_resource(
        &mut self,
        new_resource: &mut impl YyResource,
        config_deltas: Option<Vec<String>>,
    ) {
        let mut iteration_count = 0;
        let mut name = new_resource.name().to_owned();
        while self.resource_names.contains(&name) {
            name = format!("{}_{}", name, iteration_count);
            iteration_count += 1;
        }
        if name != new_resource.name() {
            new_resource.set_name(name.clone());
        }
        self.resource_names.insert(name);

        self.add_new_unnamed_resource(new_resource, config_deltas);
    }

    pub fn serialize(&mut self) -> Result<()> {
        if self.dirty {
            // self.yyp
            //     .resources
            //     .sort_by(|lr, rr| lr.key.inner().cmp(&rr.key.inner()));

            info!("Resources Serialized...");
            // Check if Sprite is Dirty and Serialize that:
            self.sprites
                .serialize(&self.absolute_path.parent().unwrap())?;
            // self.folders
            //     .serialize(&self.absolute_path.parent().unwrap())?;

            // Serialize Ourselves:
            serialize(&self.absolute_path, &self.yyp)?;

            self.dirty = false;
        }

        Ok(())
    }

    fn add_new_unnamed_resource(
        &mut self,
        new_resource: &impl YyResource,
        config_deltas: Option<Vec<String>>,
    ) {
        // // New Resource:
        // let new_yy_resource = YypResource {
        //     key: new_resource.id().into(),
        //     value: YypResourceValue {
        //         config_deltas,
        //         id: YypResourceId::new(),
        //         resource_path: new_resource
        //             .relative_filepath()
        //             .to_string_lossy()
        //             .to_string(),
        //         resource_type: new_resource.yy_resource_type(),
        //     },
        // };

        // Update the Resource
        // self.yyp.resources.push(new_yy_resource);
        self.dirty = true;
    }
}

impl YypBoss {
    pub fn root_sprite_folder(&self) -> Option<String> {
        // let main_root: &Leaf = self.folder_graph.iter_roots().nth(0).unwrap();
        // main_root.children(&self.folder_graph).find_map(|child| {
        //     let inner: YypResourceKeyId = *child.inner();
        //     self.folders
        //         .resources
        //         .get(&inner.into())
        //         .and_then(|folder_data| {
        //             if folder_data
        //                 .yy_resource
        //                 .folder_name
        //                 .eq_ignore_ascii_case("sprites")
        //             {
        //                 Some(folder_data.yy_resource.id)
        //             } else {
        //                 None
        //             }
        //         })
        // })
        None
    }

    // pub fn children_at_branch(
    //     &self,
    //     resource_key: &YypResourceKeyId,
    // ) -> Option<impl Iterator<Item = &YypResourceKeyId>> {
    //     self.folders
    //         .resources
    //         .get(&(*resource_key).into())
    //         .map(|folder_data| {
    //             let leaf = folder_data.associated_data;
    //             self.folder_graph[leaf]
    //                 .children(&self.folder_graph)
    //                 .map(|id| id.inner())
    //         })
    // }

    // pub fn parent_at_branch(&self, resource_key: &YypResourceKeyId) -> Option<GmFolderId> {
    //     self.folder_graph
    //         .iter()
    //         .find(|graph_node: &&Leaf| graph_node.inner() == resource_key)
    //         .and_then(|leaf| {
    //             let leaf: &Leaf = leaf;

    //             leaf.parent().map(|leaf_id| {
    //                 let leaf_id = self.folder_graph.get(leaf_id).unwrap();
    //                 let resource_key = *leaf_id.inner();

    //                 assert!(self.folders.resources.get(&resource_key.into()).is_some());
    //                 resource_key.into()
    //             })
    //         })
    // }

    // pub fn key_info(&self, resource_key: &YypResourceKeyId) -> Option<(String, ResourceType)> {
    //     self.yyp
    //         .resources
    //         .iter()
    //         .find(|v| v.key == *resource_key)
    //         .and_then(|key| match key.value.resource_type {
    //             ResourceType::GmSprite => self
    //                 .sprites
    //                 .resources
    //                 .get(&(*resource_key).into())
    //                 .map(|sprite| (sprite.yy_resource.name.clone(), ResourceType::GmSprite)),
    //             ResourceType::GmFolder => {
    //                 self.folders
    //                     .resources
    //                     .get(&(*resource_key).into())
    //                     .map(|folder| {
    //                         (
    //                             folder.yy_resource.folder_name.clone(),
    //                             ResourceType::GmFolder,
    //                         )
    //                     })
    //             }
    //             _ => None,
    //         })
    // }

    // pub fn pretty_print_views(&self) {
    //     self.folder_graph.print_tree(|leaf| {
    //         if let Some(resource) = self.yyp.resources.iter().find(|r| r.key == *leaf.inner()) {
    //             let resource: &YypResource = resource;
    //             let path = self
    //                 .absolute_path
    //                 .parent()
    //                 .unwrap()
    //                 .join(&resource.value.resource_path.replace("\\", "/"));

    //             match resource.value.resource_type {
    //                 ResourceType::GmFolder => {
    //                     if let Ok(file) = deserialize_json(&path) {
    //                         if let serde_json::Value::Object(map) = file {
    //                             if let serde_json::Value::String(name) = &map["folderName"] {
    //                                 println!("{}", name);
    //                             } else {
    //                                 println!("<err: unknown {:?}>", resource.value.resource_type);
    //                             }
    //                         } else {
    //                             println!("<err: unexpected json>")
    //                         }
    //                     } else {
    //                         println!("<err: unknown json>");
    //                     }
    //                 }
    //                 _ => {
    //                     if let Ok(file) = deserialize_json(&path) {
    //                         if let serde_json::Value::Object(map) = file {
    //                             if let serde_json::Value::String(name) = &map["name"] {
    //                                 println!("{}", name);
    //                             } else {
    //                                 println!("<err: unknown {:?}>", resource.value.resource_type);
    //                             }
    //                         } else {
    //                             println!("<err: unexpected json>")
    //                         }
    //                     } else {
    //                         println!("<err: unknown json>");
    //                     }
    //                 }
    //             }
    //         } else {
    //             println!("<err: unknown resource>")
    //         }
    //     });
    // }

    // fn append_under_folder(
    //     &mut self,
    //     folder_id: GmFolderId,
    //     id: impl Into<YypResourceKeyId>,
    // ) -> LeafId {
    //     let resource_id: YypResourceKeyId = id.into();

    //     // Make as Dirty
    //     self.folders.dirty = true;
    //     self.folders.dirty_resources.push(folder_id);

    //     // Get the Folder and append the resource id to it.
    //     let folder_data = self.folders.resources.get_mut(&folder_id).unwrap();
    //     folder_data.yy_resource.children.push(resource_id);

    //     // Get the LeafID for our abstract FolderGraph, and then append a new node to it.
    //     let leaf: LeafId = folder_data.associated_data;
    //     let new_leaf_id = self.folder_graph.instantiate_node(resource_id);
    //     leaf.append(new_leaf_id, &mut self.folder_graph);

    //     // Return the Leaf Id
    //     new_leaf_id
    // }

    // fn get_root_folder(&self) -> GmFolderId {
    //     let main_root: &Leaf = self.folder_graph.iter_roots().nth(0).unwrap();
    //     let main_folder_id = *main_root.inner();
    //     self.folders
    //         .resources
    //         .get(&main_folder_id.into())
    //         .unwrap()
    //         .yy_resource
    //         .id
    // }
}

/// Utilities
impl YypBoss {
    fn deserialize_yyfile<T>(&self, path: &Path) -> Result<T>
    where
        for<'de> T: serde::Deserialize<'de>,
    {
        let file_string = fs::read_to_string(path)?;
        let data = serde_json::from_str(&self.tcu.clear_trailing_comma(&file_string))?;
        Ok(data)
    }
}

#[derive(Debug, Default)]
pub struct YyResourceData<T: YyResource> {
    pub yy_resource: T,
    pub associated_data: T::AssociatedData,
}

#[derive(Debug, Default)]
pub struct YyResourceHandler<T: YyResource> {
    dirty: bool,
    resources: HashMap<FilesystemPath, YyResourceData<T>>,
    dirty_resources: Vec<FilesystemPath>,
}

impl<T: YyResource> YyResourceHandler<T> {
    pub fn new() -> Self {
        Self {
            dirty: false,
            resources: HashMap::new(),
            dirty_resources: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn add_new(&mut self, value: T, associated_data: T::AssociatedData) {
        self.dirty_resources.push(value.filesystem_path());
        self.dirty = true;
        self.add_new_startup(value, associated_data);
    }

    /// This is the same as `add_new` but it doesn't dirty the resource. It is used
    /// for startup operations, where we're loading in assets from disk.
    fn add_new_startup(&mut self, value: T, associated_data: T::AssociatedData) {
        self.resources.insert(
            value.filesystem_path(),
            YyResourceData {
                yy_resource: value,
                associated_data,
            },
        );
    }

    pub fn serialize(&mut self, project_path: &Path) -> Result<()> {
        if self.dirty {
            while let Some(dirty_resource) = self.dirty_resources.pop() {
                let resource = self
                    .resources
                    .get(&dirty_resource)
                    .expect("This should always be valid.");

                let yy_path = project_path.join(&resource.yy_resource.filesystem_path().path);

                if let Some(parent_dir) = yy_path.parent() {
                    fs::create_dir_all(parent_dir)?;
                    T::serialize_associated_data(
                        &resource.yy_resource,
                        parent_dir,
                        &resource.associated_data,
                    )?;
                }
                serialize(&yy_path, &resource.yy_resource)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct FolderGraph {
    pub name: String,
    pub subfolders: Vec<FolderGraph>,
    pub files: Vec<FilesystemPath>,
}

impl FolderGraph {
    pub fn root() -> FolderGraph {
        FolderGraph::new("folders".to_string())
    }

    pub fn new(name: String) -> FolderGraph {
        FolderGraph {
            name,
            subfolders: vec![],
            files: vec![],
        }
    }
}

fn serialize(absolute_path: &Path, data: &impl serde::Serialize) -> Result<()> {
    let data = serde_json::to_string_pretty(data)?;
    fs::write(absolute_path, data)?;
    Ok(())
}

// fn deserialize_json(path: &Path) -> Result<serde_json::Value> {
//     let file_string = fs::read_to_string(path)?;
//     let data = serde_json::from_str(&file_string)?;
//     Ok(data)
// }
