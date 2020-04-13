use super::{
    sprite_ext::SpriteImageBuffer,
    texture_group_ext::{TextureGroupController, TextureGroupExt},
    yy_typings::{
        resources::{sprite::*, texture_group::*, ResourceType},
        yyp::*,
    },
    YyResource,
};
use anyhow::{Context, Result};
use log::info;
use std::collections::HashMap;
use std::{
    fs,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct YypBoss {
    yyp: Yyp,
    dirty: bool,
    absolute_path: PathBuf,
    sprites: YyResourceHandler<Sprite>,
    texture_group_controller: TextureGroupController,
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
            texture_group_controller,
        };

        // Load in Sprites
        for sprite_resources in yyp_boss
            .yyp
            .resources
            .iter()
            .filter(|value| value.value.resource_type == ResourceType::GmSprite)
        {
            let sprite_resource: &YypResource = sprite_resources;
            let sprite_path = yyp_boss
                .absolute_path
                .parent()
                .unwrap()
                .join(&sprite_resource.value.resource_path.replace("\\", "/"));

            let sprite_yy: Sprite = deserialize(&sprite_path).with_context(|| {
                format!(
                    "Sprite path is {:#?} and exists: {}",
                    sprite_path,
                    sprite_path.exists()
                )
            })?;

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

            yyp_boss.sprites.add_new_clean(sprite_yy, frame_buffers);
        }

        Ok(yyp_boss)
    }

    pub fn absolute_path(&self) -> &Path {
        &self.absolute_path
    }

    /// Add a sprite into the YYP Boss. It is not immediately serialized,
    /// but will be serialized the next time the entire YYP Boss is.
    pub fn add_sprite(
        &mut self,
        sprite: Sprite,
        associated_data: Vec<(FrameId, SpriteImageBuffer)>,
    ) {
        self.add_new_resource(&sprite, None);
        self.sprites.add_new(sprite, associated_data);
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

    pub fn serialize(&mut self) -> Result<()> {
        if self.dirty {
            self.yyp
                .resources
                .sort_by(|lr, rr| lr.key.inner().cmp(&rr.key.inner()));

            info!("Resources Serialized...");
            // Check if Sprite is Dirty and Serialize that:
            self.sprites
                .serialize(&self.absolute_path.parent().unwrap())?;

            // Serialize Ourselves:
            serialize(&self.absolute_path, &self.yyp)?;

            self.dirty = false;
        }

        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct YyResourceData<T: YyResource> {
    pub yy_resouce: T,
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
                yy_resouce: value,
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

                let yy_path = project_path.join(&resource.yy_resouce.relative_filepath());

                if let Some(parent_dir) = yy_path.parent() {
                    fs::create_dir_all(parent_dir)?;
                    info!("About to Serialize Associated Data...");
                    T::serialize_associated_data(
                        &resource.yy_resouce,
                        parent_dir,
                        &resource.associated_data,
                    )?;
                }
                serialize(&yy_path, &resource.yy_resouce)?;
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
