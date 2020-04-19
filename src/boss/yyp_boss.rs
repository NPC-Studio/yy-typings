use super::{
    folder_graph::*,
    yy_typings::{
        resources::{folder::*, sprite::*, texture_group::*, ResourceType},
        yyp::*,
    },
    GmFolderExt, SpriteImageBuffer, TextureGroupController, TextureGroupExt, YyResource,
};
use anyhow::{Context, Result};
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
    folders: YyResourceHandler<GmFolder>,
    texture_group_controller: TextureGroupController,
    folder_graph: FolderGraph,
    resource_names: HashSet<String>,
    dirty: bool,
}

impl YypBoss {
    /// Creates a new YyBoss Manager. Note that it performs the
    /// fs itself here. See `with_components` if handling this in
    /// some other way.
    pub fn new(path_to_yyp: PathBuf) -> Result<YypBoss> {
        let yy_file = fs::read_to_string(&path_to_yyp)?;
        let yyp: Yyp = serde_json::from_str(&yy_file)?;

        // Load in TextureGroups...*shudder*
        let texture_group_controller = {
            let options_path = path_to_yyp
                .parent()
                .unwrap()
                .join("options/main/inherited/options_main.inherited.yy");
            let options_text = fs::read_to_string(options_path)
                .with_context(|| "We couldn't read our main options file")?;

            TextureGroup::parse_options_file(&options_text)?
        };

        let mut yyp_boss = Self {
            yyp,
            absolute_path: path_to_yyp,
            dirty: false,
            sprites: YyResourceHandler::new(),
            folders: YyResourceHandler::new(),
            texture_group_controller,
            folder_graph: FolderGraph::new(),
            resource_names: HashSet::new(),
        };

        // Load in Sprites
        for sprite_resource in yyp_boss
            .yyp
            .resources
            .iter()
            .filter(|value| value.value.resource_type == ResourceType::GmSprite)
        {
            let sprite_resource: &YypResource = sprite_resource;
            let sprite_path = yyp_boss
                .absolute_path
                .parent()
                .unwrap()
                .join(&sprite_resource.value.resource_path.replace("\\", "/"));

            let sprite_yy: Sprite = deserialize(&sprite_path)
                .with_context(|| format!("Error importing sprite with Path {:#?}", sprite_path))?;

            let frame_buffers: Vec<_> = sprite_yy
                .frames
                .iter()
                .filter_map(|frame: &Frame| {
                    let path_to_image = sprite_path
                        .parent()
                        .unwrap()
                        .join(Path::new(&frame.id.inner().to_string()).with_extension("png"));

                    match image::open(&path_to_image) {
                        Ok(image) => Some((frame.id, image.to_rgba())),
                        Err(e) => {
                            log::error!("We couldn't read {:?} -- {}", path_to_image, e);
                            None
                        }
                    }
                })
                .collect();

            yyp_boss.resource_names.insert(sprite_yy.name.clone());
            yyp_boss.sprites.add_new_clean(sprite_yy, frame_buffers);
        }

        // Load in Views
        fn import_view(
            view: &YypResource,
            absolute_dir: &Path,
            folder_graph: &mut FolderGraph,
            resource_handler: &mut YyResourceHandler<GmFolder>,
            resources: &Vec<YypResource>,
        ) -> Result<LeafId> {
            let view_path = absolute_dir.join(&view.value.resource_path.replace("\\", "/"));

            let view_yy: GmFolder = deserialize(&view_path)
                .with_context(|| format!("Error importing view with path {:#?}", view_path))?;

            let leaf_id = folder_graph.instantiate_node(view_yy.id.into());
            resource_handler.add_new_clean(view_yy.clone(), leaf_id);

            for child in view_yy.children.iter() {
                if let Some(resource) = resources.iter().find(|r| r.key == *child) {
                    let new_branch = match resource.value.resource_type {
                        ResourceType::GmFolder => import_view(
                            resource,
                            absolute_dir,
                            folder_graph,
                            resource_handler,
                            resources,
                        )?,
                        _ => folder_graph.instantiate_node((*child).into()),
                    };

                    leaf_id.append(new_branch, folder_graph);
                }
            }

            Ok(leaf_id)
        }

        for view in yyp_boss
            .yyp
            .resources
            .iter()
            .filter(|v| v.value.resource_type == ResourceType::GmFolder)
        {
            let view_path = yyp_boss
                .absolute_path
                .parent()
                .unwrap()
                .join(&view.value.resource_path.replace("\\", "/"));

            let view_yy: GmFolder = deserialize(&view_path)
                .with_context(|| format!("Error importing view with path {:#?}", view_path))?;

            if view_yy.is_default_view && view_yy.filter_type == "root" {
                import_view(
                    view,
                    yyp_boss.absolute_path.parent().unwrap(),
                    &mut yyp_boss.folder_graph,
                    &mut yyp_boss.folders,
                    &yyp_boss.yyp.resources,
                )?;

                break;
            }
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
        mut sprite: Sprite,
        associated_data: Vec<(FrameId, SpriteImageBuffer)>,
        folder_id: GmFolderId,
    ) {
        let sprite_id = sprite.id;
        self.add_new_resource(&mut sprite, None);
        self.sprites.add_new(sprite, associated_data);

        self.append_under_folder(folder_id, sprite_id);
    }

    pub fn add_folder(&mut self, folder: GmFolder, folder_id: GmFolderId) {
        let id = folder.id;

        // We add it "unnamed" because folder names can be duplicates of each other,
        // so we don't want to track their names.
        self.add_new_unnamed_resource(&folder, None);
        let leaf_id = self.append_under_folder(folder_id, id);

        self.folders.add_new(folder, leaf_id);
    }

    pub fn texture_group_controller(&self) -> &TextureGroupController {
        &self.texture_group_controller
    }

    /// Adds a new Resource to be tracked by the YYP. The Resource also will
    /// need to serialize themselves and any additional files which they manage.
    ///
    /// This might include serializing sprites or sprite frames for Sprites, or `.gml`
    /// files for scripts or objects.
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
            self.yyp
                .resources
                .sort_by(|lr, rr| lr.key.inner().cmp(&rr.key.inner()));

            info!("Resources Serialized...");
            // Check if Sprite is Dirty and Serialize that:
            self.sprites
                .serialize(&self.absolute_path.parent().unwrap())?;
            self.folders
                .serialize(&self.absolute_path.parent().unwrap())?;

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
        // New Resource:
        let new_yy_resource = YypResource {
            key: new_resource.id().into(),
            value: YypResourceValue {
                config_deltas,
                id: YypResourceId::new(),
                resource_path: new_resource
                    .relative_filepath()
                    .to_string_lossy()
                    .to_string(),
                resource_type: new_resource.yy_resource_type(),
            },
        };

        // Update the Resource
        self.yyp.resources.push(new_yy_resource);
        self.dirty = true;
    }
}

impl YypBoss {
    pub fn root_sprite_folder(&self) -> Option<GmFolderId> {
        let main_root: &Leaf = self.folder_graph.iter_roots().nth(0).unwrap();
        main_root.children(&self.folder_graph).find_map(|child| {
            let inner: YypResourceKeyId = *child.inner();
            self.folders
                .resources
                .get(&inner.into())
                .and_then(|folder_data| {
                    if folder_data.yy_resource.folder_name == "sprites" {
                        Some(folder_data.yy_resource.id)
                    } else {
                        None
                    }
                })
        })
    }

    /// This gets the Root Sprite folder; if it doesn't exist, it creates it.
    pub fn root_sprite_folder_infallible(&mut self) -> GmFolderId {
        if let Some(root) = self.root_sprite_folder() {
            root
        } else {
            let root_folder_id = self.get_root_folder();

            let new_folder =
                GmFolder::new("sprites".to_string()).filter_type("GMSprites".to_string());

            let id = new_folder.id;
            let new_leaf_id = self.append_under_folder(root_folder_id, new_folder.id);
            self.folders.add_new(new_folder, new_leaf_id);

            id
        }
    }

    pub fn children_at_branch(
        &self,
        resource_key: &YypResourceKeyId,
    ) -> Option<impl Iterator<Item = &YypResourceKeyId>> {
        self.folders
            .resources
            .get(&(*resource_key).into())
            .map(|folder_data| {
                let leaf = folder_data.associated_data;
                self.folder_graph[leaf]
                    .children(&self.folder_graph)
                    .map(|id| id.inner())
            })
    }

    pub fn parent_at_branch(&self, resource_key: &YypResourceKeyId) -> Option<GmFolderId> {
        self.folder_graph
            .iter()
            .find(|graph_node: &&Leaf| graph_node.inner() == resource_key)
            .and_then(|leaf| {
                let leaf: &Leaf = leaf;

                leaf.parent().map(|leaf_id| {
                    let leaf_id = self.folder_graph.get(leaf_id).unwrap();
                    let resource_key = *leaf_id.inner();

                    assert!(self.folders.resources.get(&resource_key.into()).is_some());
                    resource_key.into()
                })
            })
    }

    pub fn key_info(&self, resource_key: &YypResourceKeyId) -> Option<(String, ResourceType)> {
        self.yyp
            .resources
            .iter()
            .find(|v| v.key == *resource_key)
            .and_then(|key| match key.value.resource_type {
                ResourceType::GmSprite => self
                    .sprites
                    .resources
                    .get(&(*resource_key).into())
                    .map(|sprite| (sprite.yy_resource.name.clone(), ResourceType::GmSprite)),
                ResourceType::GmFolder => {
                    self.folders
                        .resources
                        .get(&(*resource_key).into())
                        .map(|folder| {
                            (
                                folder.yy_resource.folder_name.clone(),
                                ResourceType::GmFolder,
                            )
                        })
                }
                _ => None,
            })
    }

    pub fn pretty_print_views(&self) {
        self.folder_graph.print_tree(|leaf| {
            if let Some(resource) = self.yyp.resources.iter().find(|r| r.key == *leaf.inner()) {
                let resource: &YypResource = resource;
                let path = self
                    .absolute_path
                    .parent()
                    .unwrap()
                    .join(&resource.value.resource_path.replace("\\", "/"));

                match resource.value.resource_type {
                    ResourceType::GmFolder => {
                        if let Ok(file) = deserialize_json(&path) {
                            if let serde_json::Value::Object(map) = file {
                                if let serde_json::Value::String(name) = &map["folderName"] {
                                    println!("{}", name);
                                } else {
                                    println!("<err: unknown {:?}>", resource.value.resource_type);
                                }
                            } else {
                                println!("<err: unexpected json>")
                            }
                        } else {
                            println!("<err: unknown json>");
                        }
                    }
                    _ => {
                        if let Ok(file) = deserialize_json(&path) {
                            if let serde_json::Value::Object(map) = file {
                                if let serde_json::Value::String(name) = &map["name"] {
                                    println!("{}", name);
                                } else {
                                    println!("<err: unknown {:?}>", resource.value.resource_type);
                                }
                            } else {
                                println!("<err: unexpected json>")
                            }
                        } else {
                            println!("<err: unknown json>");
                        }
                    }
                }
            } else {
                println!("<err: unknown resource>")
            }
        });
    }

    fn append_under_folder(
        &mut self,
        folder_id: GmFolderId,
        id: impl Into<YypResourceKeyId>,
    ) -> LeafId {
        let resource_id: YypResourceKeyId = id.into();

        // Make as Dirty
        self.folders.dirty = true;
        self.folders.dirty_resources.push(folder_id);

        // Get the Folder and append the resource id to it.
        let folder_data = self.folders.resources.get_mut(&folder_id).unwrap();
        folder_data.yy_resource.children.push(resource_id);

        // Get the LeafID for our abstract FolderGraph, and then append a new node to it.
        let leaf: LeafId = folder_data.associated_data;
        let new_leaf_id = self.folder_graph.instantiate_node(resource_id);
        leaf.append(new_leaf_id, &mut self.folder_graph);

        // Return the Leaf Id
        new_leaf_id
    }

    fn get_root_folder(&self) -> GmFolderId {
        let main_root: &Leaf = self.folder_graph.iter_roots().nth(0).unwrap();
        let main_folder_id = *main_root.inner();
        self.folders
            .resources
            .get(&main_folder_id.into())
            .unwrap()
            .yy_resource
            .id
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
    resources: HashMap<T::Key, YyResourceData<T>>,
    dirty_resources: Vec<T::Key>,
}

impl<T: YyResource> YyResourceHandler<T> {
    pub fn new() -> Self {
        Self {
            dirty: false,
            resources: HashMap::new(),
            dirty_resources: Vec::new(),
        }
    }

    pub fn add_new(&mut self, value: T, associated_data: T::AssociatedData) {
        self.dirty_resources.push(value.id());
        self.dirty = true;
        self.add_new_clean(value, associated_data);
    }

    fn add_new_clean(&mut self, value: T, associated_data: T::AssociatedData) {
        self.resources.insert(
            value.id(),
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

                let yy_path = project_path.join(&resource.yy_resource.relative_filepath());

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

fn serialize(absolute_path: &Path, data: &impl serde::Serialize) -> Result<()> {
    let data = serde_json::to_string_pretty(data)?;
    fs::write(absolute_path, data)?;
    Ok(())
}

fn deserialize<T>(path: &Path) -> Result<T>
where
    for<'de> T: serde::Deserialize<'de>,
{
    let file_string = fs::read_to_string(path)?;
    let data = serde_json::from_str(&file_string)?;
    Ok(data)
}

fn deserialize_json(path: &Path) -> Result<serde_json::Value> {
    let file_string = fs::read_to_string(path)?;
    let data = serde_json::from_str(&file_string)?;
    Ok(data)
}
