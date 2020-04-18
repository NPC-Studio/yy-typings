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
        Sprite {
            name,
            texture_group_id,
            ..Default::default()
        }
    }

    fn layer(mut self, f: impl Fn(SpriteId) -> Layer) -> Self {
        self.layers.push(f(self.id));
        self
    }

    fn frame(mut self, f: impl Fn(&Sprite) -> Frame) -> Self {
        self.frames.push(f(&self));
        self
    }

    /// Sets the Bbox Mode and the desired size. Note: right now, we do not support
    /// `BboxMode::Automatic` in anyway. It is identical to `BboxMode::Manual` for now.
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

    fn collision_kind(mut self, collision_kind: CollisionKind) -> Self {
        self.colkind = collision_kind;
        if self.colkind != CollisionKind::Precise {
            self.sepmasks = false;
        }
        self
    }

    fn mask_per_frame(mut self, mask_per_frame: bool) -> Self {
        if self.colkind == CollisionKind::Precise {
            self.sepmasks = mask_per_frame;
        } else {
            self.sepmasks = false;
        }

        self
    }

    fn coltolerance(mut self, col_tolerance: u8) -> Self {
        self.coltolerance = col_tolerance;
        self
    }

    fn edge_filtering(mut self, edge_filtering: bool) -> Self {
        self.edge_filtering = edge_filtering;
        self
    }

    fn for_3d(mut self, for3d: bool) -> Self {
        self.for3d = for3d;
        self
    }

    fn origin(mut self, origin: OriginUtility, locked: bool) -> Self {
        match origin {
            OriginUtility::Custom { x, y } => {
                self.origin = Origin::Custom;
                self.xorig = x;
                self.yorig = y;
            }
            OriginUtility::TopLeft => {
                self.origin = Origin::TopLeft;
                self.xorig = 0;
                self.yorig = 0;
            }
            OriginUtility::TopCenter => {
                self.origin = Origin::TopCenter;
                self.xorig = (self.width.get() / 2) as isize;
                self.yorig = 0;
            }
            OriginUtility::TopRight => {
                self.origin = Origin::TopRight;
                self.xorig = (self.width.get() - 1) as isize;
                self.yorig = 0;
            }
            OriginUtility::MiddleLeft => {
                self.origin = Origin::MiddleLeft;
                self.xorig = 0;
                self.yorig = (self.height.get() / 2) as isize;
            }
            OriginUtility::MiddleCenter => {
                self.origin = Origin::MiddleCenter;
                self.xorig = (self.width.get() / 2) as isize;
                self.yorig = (self.height.get() / 2) as isize;
            }
            OriginUtility::MiddleRight => {
                self.origin = Origin::MiddleRight;
                self.xorig = (self.width.get() / 2) as isize;
                self.yorig = (self.height.get() / 2) as isize;
            }
            OriginUtility::BottomLeft => {
                self.origin = Origin::BottomLeft;
                self.xorig = 0;
                self.yorig = (self.height.get() - 1) as isize;
            }
            OriginUtility::BottomCenter => {
                self.origin = Origin::BottomCenter;
                self.xorig = (self.width.get() / 2) as isize;
                self.yorig = (self.height.get() - 1) as isize;
            }
            OriginUtility::BottomRight => {
                self.origin = Origin::BottomRight;
                self.xorig = (self.width.get() - 1) as isize;
                self.yorig = (self.height.get() - 1) as isize;
            }
        }

        self.origin_locked = locked;

        self
    }

    fn playback_speed(mut self, speed: f64) -> Self {
        self.playback_speed = speed;
        self
    }

    fn playback_speed_type(mut self, speed_type: PlaybackSpeed) -> Self {
        self.playback_speed_type = speed_type;
        self
    }

    fn premultiply_alpha(mut self, premultiply_alpha: bool) -> Self {
        self.premultiply_alpha = premultiply_alpha;
        self
    }

    fn tile(mut self, tile: (bool, bool)) -> Self {
        self.h_tile = tile.0;
        self.v_tile = tile.1;
        self
    }

    fn dimensions(mut self, width: NonZeroUsize, height: NonZeroUsize) -> Self {
        self.width = width;
        self.height = height;
        self
    }
}

pub trait FrameExt {
    fn new(sprite: &Sprite) -> Self;
}

impl FrameExt for Frame {
    fn new(sprite: &Sprite) -> Self {
        let id = FrameId::new();
        Self {
            id,
            mvc: "1.0".to_string(),
            sprite_id: sprite.id,
            model_name: Default::default(),

            composite_image: Image::new(id, LayerId::default()),
            images: sprite
                .layers
                .iter()
                .map(|layer| Image::new(id, layer.id))
                .collect(),
        }
    }
}

pub trait ImageExt {
    fn new(frame_id: FrameId, layer_id: LayerId) -> Self;
}

impl ImageExt for Image {
    fn new(frame_id: FrameId, layer_id: LayerId) -> Self {
        Self {
            id: ImageId::new(),
            model_name: Default::default(),
            mvc: "1.0".to_owned(),
            frame_id,
            layer_id,
        }
    }
}

pub trait LayerExt {
    fn new(sprite_id: SpriteId) -> Self;
}

impl LayerExt for Layer {
    fn new(sprite_id: SpriteId) -> Self {
        Self {
            id: LayerId::new(),
            model_name: ConstGmImageLayer::GmImageLayer,
            mvc: "1.0".to_string(),
            sprite_id,
            blend_mode: 0,
            is_locked: false,
            name: "default".to_string(),
            opacity: 100,
            visible: true,
        }
    }
}

use anyhow::Context;
use std::path::{Path, PathBuf};
impl YyResource for Sprite {
    fn relative_filepath(&self) -> PathBuf {
        Path::new(&format!("sprites/{name}/{name}.yy", name = self.name)).to_owned()
    }

    fn id(&self) -> SpriteId {
        self.id
    }

    fn yy_resource_type(&self) -> ResourceType {
        self.model_name.into()
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

            let image_layer_id = self
                .layers
                .first()
                .ok_or_else(|| anyhow::anyhow!("All Sprites *must* have a single Layer!"))?
                .id
                .inner()
                .to_string();

            let final_layer_path = folder_path.join(&image_layer_id).with_extension("png");
            image
                .save(&final_layer_path)
                .with_context(|| format!("We couldn't save an Image to {:?}", final_layer_path))?;
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
