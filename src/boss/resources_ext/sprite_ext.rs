use super::yy_typings::{
    resources::{sprite::*, texture_group::TextureGroupId, ResourceType},
    yyp::YypResourceKeyId,
};
use super::YyResource;
use image::{ImageBuffer, Rgba};
use std::num::NonZeroUsize;

pub type SpriteImageBuffer = ImageBuffer<Rgba<u8>, Vec<u8>>;

pub trait SpriteExt {
    fn new(name: String, texture_group_id: TextureGroupId) -> Sprite;
    fn bbox_mode(self, f: impl Fn(isize, isize) -> BboxModeUtility) -> Self;
    fn layer(self, f: impl Fn(SpriteId) -> Layer) -> Self;
    fn frame(self, f: impl Fn(&Sprite) -> Frame) -> Self;
    fn collision_kind(self, collision_kind: CollisionKind) -> Self;
    fn mask_per_frame(self, mask_per_frame: bool) -> Self;
    fn coltolerance(self, col_tolerance: u8) -> Self;
    fn edge_filtering(self, edge_filtering: bool) -> Self;
    fn for_3d(self, for3d: bool) -> Self;
    fn origin(self, origin: OriginUtility, locked: bool) -> Self;
    fn playback_speed(self, speed: f64) -> Self;
    fn playback_speed_type(self, speed_type: PlaybackSpeed) -> Self;
    fn premultiply_alpha(self, premultiply_alpha: bool) -> Self;
    fn tile(self, tile: (bool, bool)) -> Self;
    fn dimensions(self, width: NonZeroUsize, height: NonZeroUsize) -> Self;
}

impl SpriteExt for Sprite {
    fn new(name: String, texture_group_id: TextureGroupId) -> Sprite {
        todo!()
    }
    fn bbox_mode(self, f: impl Fn(isize, isize) -> BboxModeUtility) -> Self {
        todo!()
    }
    fn layer(self, f: impl Fn(SpriteId) -> Layer) -> Self {
        todo!()
    }
    fn frame(self, f: impl Fn(&Sprite) -> Frame) -> Self {
        todo!()
    }
    fn collision_kind(self, collision_kind: CollisionKind) -> Self {
        todo!()
    }
    fn mask_per_frame(self, mask_per_frame: bool) -> Self {
        todo!()
    }
    fn coltolerance(self, col_tolerance: u8) -> Self {
        todo!()
    }
    fn edge_filtering(self, edge_filtering: bool) -> Self {
        todo!()
    }
    fn for_3d(self, for3d: bool) -> Self {
        todo!()
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
    fn premultiply_alpha(self, premultiply_alpha: bool) -> Self {
        todo!()
    }
    fn tile(self, tile: (bool, bool)) -> Self {
        todo!()
    }
    fn dimensions(self, width: NonZeroUsize, height: NonZeroUsize) -> Self {
        todo!()
    }
}

pub trait FrameExt {
    fn new(sprite: &Sprite) -> Self;
}

impl FrameExt for Frame {
    fn new(sprite: &Sprite) -> Self {
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
    fn new(sprite_id: SpriteId) -> Self;
}

impl LayerExt for Layer {
    fn new(sprite_id: SpriteId) -> Self {
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

    fn id(&self) -> SpriteId {
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

    type Key = SpriteId;
    type AssociatedData = Vec<(FrameId, SpriteImageBuffer)>;
}

impl Into<YypResourceKeyId> for SpriteId {
    fn into(self) -> YypResourceKeyId {
        YypResourceKeyId::with_id(self.inner())
    }
}

impl Into<SpriteId> for YypResourceKeyId {
    fn into(self) -> SpriteId {
        SpriteId::with_id(self.inner())
    }
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
