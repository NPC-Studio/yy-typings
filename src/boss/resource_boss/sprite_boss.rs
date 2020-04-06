use super::yy_typings::{
    resources::sprite::*,
    yyp::{ResourceType, YypResourceKeyId},
};
use super::YyResource;

impl Sprite {
    pub fn new(name: String) -> Sprite {
        Sprite {
            id: SpriteId::new(),
            mvc: "1.12".to_string(),
            name,
            resource_sprite_type: 0,
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
                let width = self.width as isize;
                let height = self.height as isize;

                self.bbox(Bbox {
                    top_left: (0, 0),
                    bottom_right: (width, height),
                })
            }
            BBoxMode::FullImage => {
                let width = self.width as isize;
                let height = self.height as isize;

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
