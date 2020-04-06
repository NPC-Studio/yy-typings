use super::yy_typings::{
    resources::sprite::*,
    yyp::{ResourceType, YypResourceKeyId},
};
use super::YyResource;
use std::num::NonZeroUsize;

impl Sprite {
    pub fn new(name: String) -> Sprite {
        Sprite {
            name,
            ..Default::default()
        }
    }

    pub fn bbox_mode(mut self, bbox_mode: BBoxMode) -> Self {
        self.bbox_mode = bbox_mode;
        match self.bbox_mode {
            BBoxMode::Automatic => {
                log::warn!(
                    "We cannot correctly handle Automatic Bbox Modes at this time.\n\
                    Setting to the Size of Full Image..."
                );
                let width = self.width.get() as isize;
                let height = self.height.get() as isize;

                self.bbox(Bbox {
                    top_left: (0, 0),
                    bottom_right: (width, height),
                })
            }
            BBoxMode::FullImage => {
                let width = self.width.get() as isize;
                let height = self.height.get() as isize;

                self.bbox(Bbox {
                    top_left: (0, 0),
                    bottom_right: (width, height),
                })
            }
            BBoxMode::Manual => self,
        }
    }

    pub fn bbox(mut self, bbox: Bbox) -> Self {
        if self.bbox_mode == BBoxMode::Manual {
            self.bbox_left = bbox.top_left.0;
            self.bbox_top = bbox.top_left.1;
            self.bbox_right = bbox.bottom_right.0;
            self.bbox_bottom = bbox.bottom_right.1;
        }
        self
    }

    pub fn collision_kind(mut self, collision_kind: CollisionKind) -> Self {
        self.colkind = collision_kind;
        if self.colkind != CollisionKind::Precise {
            self.sepmasks = false;
        }
        self
    }

    pub fn mask_per_frame(mut self, mask_per_frame: bool) -> Self {
        if self.colkind == CollisionKind::Precise {
            self.sepmasks = mask_per_frame;
        } else {
            self.sepmasks = false;
        }

        self
    }

    pub fn coltolerance(mut self, col_tolerance: u8) -> Self {
        self.coltolerance = col_tolerance;
        self
    }

    pub fn edge_filtering(mut self, edge_filtering: bool) -> Self {
        self.edge_filtering = edge_filtering;
        self
    }

    pub fn for_3d(mut self, for3d: bool) -> Self {
        self.for3d = for3d;
        self
    }

    pub fn origin(mut self, origin: OriginUtility, locked: bool) -> Self {
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
                self.xorig = self.width.get() / 2;
                self.yorig = 0;
            }
            OriginUtility::TopRight => {
                self.origin = Origin::TopRight;
                self.xorig = self.width.get() - 1;
                self.yorig = 0;
            }
            OriginUtility::MiddleLeft => {
                self.origin = Origin::MiddleLeft;
                self.xorig = 0;
                self.yorig = self.height.get() / 2;
            }
            OriginUtility::MiddleCenter => {
                self.origin = Origin::MiddleCenter;
                self.xorig = self.width.get() / 2;
                self.yorig = self.height.get() / 2;
            }
            OriginUtility::MiddleRight => {
                self.origin = Origin::MiddleRight;
                self.xorig = self.width.get() / 2;
                self.yorig = self.height.get() / 2;
            }
            OriginUtility::BottomLeft => {
                self.origin = Origin::BottomLeft;
                self.xorig = 0;
                self.yorig = self.height.get() - 1;
            }
            OriginUtility::BottomCenter => {
                self.origin = Origin::BottomCenter;
                self.xorig = self.width.get() / 2;
                self.yorig = self.height.get() - 1;
            }
            OriginUtility::BottomRight => {
                self.origin = Origin::BottomRight;
                self.xorig = self.width.get() - 1;
                self.yorig = self.height.get() - 1;
            }
        }

        self.origin_locked = locked;

        self
    }

    pub fn playback_speed(mut self, speed: f64) -> Self {
        self.playback_speed = speed;
        self
    }

    pub fn playback_speed_type(mut self, speed_type: PlaybackSpeed) -> Self {
        self.playback_speed_type = speed_type;
        self
    }

    pub fn premultiply_alpha(mut self, premultiply_alpha: bool) -> Self {
        self.premultiply_alpha = premultiply_alpha;
        self
    }

    pub fn texture_group_id(mut self, texture_group_id: TextureGroupId) -> Self {
        self.texture_group_id = texture_group_id;
        self
    }

    pub fn tile(mut self, tile: (bool, bool)) -> Self {
        self.h_tile = tile.0;
        self.v_tile = tile.1;
        self
    }

    pub fn dimensions(mut self, dimensions: (NonZeroUsize, NonZeroUsize)) -> Self {
        self.width = dimensions.0;
        self.height = dimensions.1;
        self
    }
}

use std::path::{Path, PathBuf};
impl YyResource for Sprite {
    fn relative_filepath(&self) -> PathBuf {
        Path::new(&format!("sprites/{}", self.name)).to_owned()
    }

    fn id(&self) -> SpriteId {
        self.id
    }

    fn yy_resource_type(&self) -> ResourceType {
        self.model_name.into()
    }

    fn serialize_associated_data(directory_path: &Path, data: &()) -> anyhow::Result<()> {
        unimplemented!()
    }

    type Key = SpriteId;
    type AssociatedData = ();
}

impl Into<YypResourceKeyId> for SpriteId {
    fn into(self) -> YypResourceKeyId {
        YypResourceKeyId::with_id(self.inner())
    }
}

#[derive(Debug, Default)]
pub struct Bbox {
    pub top_left: (isize, isize),
    pub bottom_right: (isize, isize),
}

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
    Custom { x: usize, y: usize },
}
