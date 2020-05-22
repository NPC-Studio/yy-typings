use super::yy_typings::{
    resources::{sprite::*, ResourceType},
    yyp::YypResourceKeyId,
};
use super::YyResource;
use image::{ImageBuffer, Rgba};
use std::num::NonZeroUsize;

pub type SpriteImageBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub trait SpriteExt {
    fn harness(self, edit: impl Fn(&mut Self)) -> Self;
    fn new(name: &str, texture_group_id: &str) -> Sprite;
    fn parent(self, parent: ViewPath) -> Sprite;
    fn bbox_mode(self, f: impl Fn(isize, isize) -> BboxModeUtility) -> Self;
    fn layer(self, f: impl Fn(&mut Sprite) -> Layer) -> Self;
    fn frame(self, f: impl Fn(&mut Sprite) -> Frame) -> Self;
    fn collision_kind(self, collision_kind: CollisionKind) -> Self;
    fn origin(self, origin: OriginUtility, locked: bool) -> Self;
    fn playback_speed(self, speed: f64) -> Self;
    fn playback_speed_type(self, speed_type: PlaybackSpeed) -> Self;
    fn dimensions(self, width: NonZeroUsize, height: NonZeroUsize) -> Self;
}

impl SpriteExt for Sprite {
    fn harness(mut self, edit: impl Fn(&mut Self)) -> Self {
        edit(&mut self);
        self
    }

    fn new(name: &str, texture_group_id: &str) -> Sprite {
        Sprite {
            name: name.to_string(),
            texture_group_id: TextureGroupPath {
                path: Path::new(&format!("texturegroups/{}", texture_group_id)).to_owned(),
                name: texture_group_id.to_string(),
            },
            ..Sprite::default()
        }
    }

    fn parent(self, parent: ViewPath) -> Sprite {
        self.harness(|me| me.parent = parent.clone())
    }

    fn bbox_mode(mut self, f: impl Fn(isize, isize) -> BboxModeUtility) -> Self {
        let bbox_util = f(self.width.get() as isize, self.height.get() as isize);
        self.bbox_mode = bbox_util.into();

        let bbox = match bbox_util {
            BboxModeUtility::Automatic(bbox) | BboxModeUtility::Manual(bbox) => bbox,
            BboxModeUtility::FullImage => {
                let width = self.width.get() as isize;
                let height = self.height.get() as isize;

                Bbox {
                    top_left: (0, 0),
                    bottom_right: (width, height),
                }
            }
        };

        self.bbox_left = bbox.top_left.0;
        self.bbox_top = bbox.top_left.1;
        self.bbox_right = bbox.bottom_right.0;
        self.bbox_bottom = bbox.bottom_right.1;
        self
    }
    fn layer(self, f: impl Fn(&mut Sprite) -> Layer) -> Self {
        todo!()
    }
    fn frame(self, f: impl Fn(&mut Sprite) -> Frame) -> Self {
        todo!()
    }
    fn collision_kind(self, collision_kind: CollisionKind) -> Self {
        self.harness(|me| {
            me.collision_kind = collision_kind;
            if me.collision_kind != CollisionKind::Precise {
                me.separate_masks = false;
            }
        })
    }
    fn origin(self, origin: OriginUtility, locked: bool) -> Self {
        todo!()
    }
    fn playback_speed(self, speed: f64) -> Self {
        todo!()
    }
    fn playback_speed_type(self, speed_type: PlaybackSpeed) -> Self {
        todo!()
    }
    fn dimensions(self, width: NonZeroUsize, height: NonZeroUsize) -> Self {
        todo!()
    }
}

pub trait FrameExt {
    fn new(sprite: &mut Sprite) -> Self;
}

impl FrameExt for Frame {
    fn new(sprite: &mut Sprite) -> Self {
        todo!()
    }
}

pub trait ImageExt {
    fn new(frame_id: FrameId, layer_id: LayerId) -> Self;
}

impl ImageExt for Image {
    fn new(frame_id: FrameId, layer_id: LayerId) -> Self {
        todo!()
    }
}

pub trait LayerExt {
    fn new(sprite_id: String) -> Self;
}

impl LayerExt for Layer {
    fn new(sprite_id: String) -> Self {
        todo!()
    }
}

use anyhow::Context;
use std::path::{Path, PathBuf};
impl YyResource for Sprite {
    fn name(&self) -> &str {
        &self.name
    }

    fn set_name(&mut self, name: String) {
        self.name = name;
    }

    fn relative_filepath(&self) -> PathBuf {
        Path::new(&format!("sprites/{name}/{name}.yy", name = self.name)).to_owned()
    }

    fn id(&self) -> YypResourceKeyId {
        todo!()
    }

    fn yy_resource_type(&self) -> ResourceType {
        self.resource_type.into()
    }

    fn serialize_associated_data(
        &self,
        directory_path: &Path,
        data: &Self::AssociatedData,
    ) -> anyhow::Result<()> {
        let layers_path = directory_path.join("layers");
        if layers_path.exists() == false {
            std::fs::create_dir(&layers_path)?;
        }

        for (frame_id, image) in data {
            let inner_id_string = frame_id.inner().to_string();
            let image: &ImageBuffer<_, _> = image;

            // Make the Core Image:
            let path = directory_path.join(&inner_id_string).with_extension("png");
            image.save(&path).with_context(|| {
                format!("We couldn't serialize the Core Image at path {:?}", path)
            })?;

            // Make the folder and layer image:
            let folder_path = layers_path.join(&inner_id_string);
            if folder_path.exists() == false {
                std::fs::create_dir(&folder_path)?;
            }

            todo!()

            // let image_layer_id = self
            //     .layers
            //     .first()
            //     .ok_or_else(|| anyhow::anyhow!("All Sprites *must* have a single Layer!"))?
            //     .id
            //     .inner()
            //     .to_string();

            // let final_layer_path = folder_path.join(&image_layer_id).with_extension("png");
            // image
            //     .save(&final_layer_path)
            //     .with_context(|| format!("We couldn't save an Image to {:?}", final_layer_path))?;
        }

        Ok(())
    }

    type Key = YypResourceKeyId;
    type AssociatedData = Vec<(FrameId, SpriteImageBuffer)>;
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Bbox {
    pub top_left: (isize, isize),
    pub bottom_right: (isize, isize),
}

#[derive(Debug, Copy, Clone, strum_macros::EnumIter, strum_macros::Display)]
pub enum OriginUtility {
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom { x: isize, y: isize },
}

#[derive(Debug, Copy, Clone, strum_macros::EnumIter, strum_macros::Display)]
pub enum BboxModeUtility {
    Automatic(Bbox),
    FullImage,
    Manual(Bbox),
}

impl From<BboxModeUtility> for BBoxMode {
    fn from(o: BboxModeUtility) -> Self {
        match o {
            BboxModeUtility::Automatic(_) => BBoxMode::Automatic,
            BboxModeUtility::FullImage => BBoxMode::FullImage,
            BboxModeUtility::Manual(_) => BBoxMode::Manual,
        }
    }
}
