use super::{
    sprite_constants::*, Frame, Layer, ResourceVersion, SpriteSequence, Tags, TexturePath, ViewPath,
};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use smart_default::SmartDefault;
use std::num::NonZeroUsize;

#[derive(Debug, Serialize, Deserialize, SmartDefault, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Sprite {
    pub bbox_mode: BBoxMode,
    pub collision_kind: CollisionKind,
    #[serde(skip)]
    #[deprecated = "removed in Gms2.3.167. Instead, use `PrecisePerFrame`"]
    pub separate_masks: bool,

    /// The type of sprite, whether a bitmap or a vector sprite.
    /// Right now, we don't handle this intelligently.
    #[serde(rename = "type")]
    pub resource_sprite_type: usize,

    pub origin: Origin,
    pub pre_multiply_alpha: bool,
    pub edge_filtering: bool,
    pub collision_tolerance: u8,
    /// The precision for Vector sprites. Its default is `2.525`, a number
    /// which is very odd in my opinion.
    #[default(2.525)]
    pub swf_precision: f64,

    #[serde(rename = "bbox_left")]
    pub bbox_left: isize,
    #[serde(rename = "bbox_right")]
    pub bbox_right: isize,
    #[serde(rename = "bbox_top")]
    pub bbox_top: isize,
    #[serde(rename = "bbox_bottom")]
    pub bbox_bottom: isize,

    #[serde(rename = "HTile")]
    pub h_tile: bool,
    #[serde(rename = "VTile")]
    pub v_tile: bool,
    #[serde(rename = "For3D")]
    pub for3d: bool,

    #[default(NonZeroUsize::new(1).unwrap())]
    pub width: NonZeroUsize,
    #[default(NonZeroUsize::new(1).unwrap())]
    pub height: NonZeroUsize,

    /// This is the Path to the Texture Group Id.
    pub texture_group_id: TexturePath,

    /// This is probably always null, unless you make a swatch,
    /// but why are you doing that! Just don't do that. Easy.
    pub swatch_colours: serde_json::Value,
    pub grid_x: usize,
    pub grid_y: usize,

    /// Each frame within this Sprite File.
    pub frames: Vec<Frame>,

    /// The sequence assigned to each sprite.
    pub sequence: SpriteSequence,

    /// Each layer within each Frame for this Sprite File. Unless users use
    /// the built-in Sprite Editor, this will likely always have a .len() of
    /// 1, as there is always *one* layer. Layers are shared between frames.
    pub layers: Vec<Layer>,

    /// Defines the parent of the YY folder path.
    pub parent: ViewPath,

    /// Version string. Right now, this is loosely typed and ignored,
    /// but will be used in the future to aid in parsing.
    pub resource_version: ResourceVersion,

    /// The human readable name of the resource, such as `spr_player`.
    pub name: String,
    /// These are the tags assigned in the GMS2 Editor to each sprite.
    pub tags: Tags,
    /// ModelName. Always GMSprite.
    pub resource_type: ConstGmSprite,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Debug,
    SmartDefault,
    Copy,
    Clone,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum CollisionKind {
    Precise,
    #[default]
    Rectangle,
    Ellipse,
    Diamond,
    PrecisePerFrame,
    RotatedRectangle,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Debug,
    SmartDefault,
    Copy,
    Clone,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum BBoxMode {
    #[default]
    Automatic,
    FullImage,
    Manual,
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    PartialEq,
    Debug,
    SmartDefault,
    Copy,
    Clone,
    Eq,
    Ord,
    PartialOrd,
)]
#[repr(u8)]
pub enum Origin {
    #[default]
    TopLeft,
    TopCenter,
    TopRight,
    MiddleLeft,
    MiddleCenter,
    MiddleRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    Custom,
}

#[cfg(test)]
mod tests {
    use crate::{sprite_yy::*, utils::TrailingCommaUtility};
    use include_dir::{include_dir, Dir, DirEntry};
    use pretty_assertions::assert_eq;
    use std::{num::NonZeroUsize, path::Path};

    #[test]
    fn trivial_sprite_parsing() {
        let all_sprites: Dir = include_dir!("data/sprites");
        let tcu = TrailingCommaUtility::new();

        for sprite_file in all_sprites.find("**/*.yy").unwrap() {
            if let DirEntry::File(file) = sprite_file {
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                serde_json::from_str::<Sprite>(&our_str).unwrap();
            }
        }
    }

    #[test]
    fn deep_check() {
        let sprite = include_str!("../../../data/sprites/test0.yy");
        let sprite: Sprite =
            serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(sprite)).unwrap();

        let expected_sprite = Sprite {
            bbox_mode: BBoxMode::Automatic,
            collision_kind: CollisionKind::Rectangle,
            resource_sprite_type: 0,
            origin: Origin::Custom,
            pre_multiply_alpha: false,
            edge_filtering: false,
            collision_tolerance: 0,
            swf_precision: 2.525,
            bbox_left: 7,
            bbox_right: 38,
            bbox_top: 5,
            bbox_bottom: 47,
            h_tile: false,
            v_tile: false,
            for3d: false,
            width: NonZeroUsize::new(48).unwrap(),
            height: NonZeroUsize::new(48).unwrap(),
            texture_group_id: TexturePath {
                name: "Default".to_string(),
                path: TexturePathLocation("texturegroups/Default".to_string()),
            },
            swatch_colours: serde_json::Value::Null,
            grid_x: 0,
            grid_y: 0,
            frames: vec![
                Frame {
                    composite_image: Image {
                        frame_id: FilesystemPath {
                            name: "7669a695-40b5-47eb-a089-f81c0f6be6b8".to_string(),
                            path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                        },
                        layer_id: None,
                        name: Some("composite".to_string()),
                        ..Image::default()
                    },
                    images: vec![Image {
                        frame_id: FilesystemPath {
                            name: "7669a695-40b5-47eb-a089-f81c0f6be6b8".to_string(),
                            path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                        },
                        layer_id: Some(FilesystemPath {
                            name: "37cbf63f-f6a9-4b91-a9fd-3537b374e9db".to_string(),
                            path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                        }),
                        name: None,
                        ..Image::default()
                    }],
                    parent: FilesystemPath {
                        name: "spr_jack".to_string(),
                        path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                    },
                    name: FrameId::with_id(
                        uuid::Uuid::parse_str("7669a695-40b5-47eb-a089-f81c0f6be6b8").unwrap(),
                    ),
                    ..Default::default()
                },
                Frame {
                    composite_image: Image {
                        frame_id: FilesystemPath {
                            name: "a52625ab-2499-4b46-9785-58ee50a1b048".to_string(),
                            path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                        },
                        layer_id: None,
                        name: Some("composite".to_string()),
                        ..Image::default()
                    },
                    images: vec![Image {
                        frame_id: FilesystemPath {
                            name: "a52625ab-2499-4b46-9785-58ee50a1b048".to_string(),
                            path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                        },
                        layer_id: Some(FilesystemPath {
                            name: "37cbf63f-f6a9-4b91-a9fd-3537b374e9db".to_string(),
                            path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                        }),
                        name: None,
                        ..Image::default()
                    }],
                    parent: FilesystemPath {
                        name: "spr_jack".to_string(),
                        path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                    },
                    name: FrameId::with_id(
                        uuid::Uuid::parse_str("a52625ab-2499-4b46-9785-58ee50a1b048").unwrap(),
                    ),
                    ..Default::default()
                },
            ],
            sequence: SpriteSequence {
                sprite_id: FilesystemPath {
                    name: "spr_jack".to_string(),
                    path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                },
                playback_speed: 15.0,
                playback_speed_type: PlaybackSpeed::FramesPerSecond,
                length: 2.0,
                tracks: vec![Track {
                    keyframes: {
                        SpriteKeyframes {
                            keyframes: vec![
                                SpriteKeyframe {
                                    id: SpriteSequenceId::with_id(
                                        uuid::Uuid::parse_str(
                                            "7ac201d7-c56c-400f-8f97-c42ad0d98fba",
                                        )
                                        .unwrap(),
                                    ),
                                    key: 0.0,
                                    channels: Channels {
                                        zero: SpriteZeroChannel {
                                            id: FilesystemPath {
                                                name: "7669a695-40b5-47eb-a089-f81c0f6be6b8"
                                                    .to_string(),
                                                path: Path::new("sprites/spr_jack/spr_jack.yy")
                                                    .to_owned(),
                                            },
                                            ..Default::default()
                                        },
                                    },
                                    ..SpriteKeyframe::default()
                                },
                                SpriteKeyframe {
                                    id: SpriteSequenceId::with_id(
                                        uuid::Uuid::parse_str(
                                            "d457b008-1cb9-43a5-8024-7576b09a0421",
                                        )
                                        .unwrap(),
                                    ),
                                    key: 1.0,
                                    channels: Channels {
                                        zero: SpriteZeroChannel {
                                            id: FilesystemPath {
                                                name: "a52625ab-2499-4b46-9785-58ee50a1b048"
                                                    .to_string(),
                                                path: Path::new("sprites/spr_jack/spr_jack.yy")
                                                    .to_owned(),
                                            },
                                            ..SpriteZeroChannel::default()
                                        },
                                    },
                                    ..SpriteKeyframe::default()
                                },
                            ],
                            ..SpriteKeyframes::default()
                        }
                    },
                    ..Track::default()
                }],
                visible_range: Some(VisibleRange { x: 0.0, y: 0.0 }),
                lock_origin: false,
                backdrop_width: 1920,
                backdrop_height: 1080,
                backdrop_x_offset: 0.0,
                backdrop_y_offset: 0.0,
                xorigin: 23,
                yorigin: 42,
                parent: FilesystemPath {
                    name: "spr_jack".to_string(),
                    path: Path::new("sprites/spr_jack/spr_jack.yy").to_owned(),
                },
                ..SpriteSequence::default()
            },
            layers: vec![Layer {
                visible: true,
                is_locked: false,
                blend_mode: 0,
                opacity: 100.0,
                display_name: "Layer 1 (2)".to_string(),
                name: LayerId::with_string("37cbf63f-f6a9-4b91-a9fd-3537b374e9db"),
                ..Layer::default()
            }],
            parent: ViewPath {
                name: "Sprites".to_string(),
                path: ViewPathLocation("folders/Sprites.yy".to_string()),
            },
            name: "spr_jack".to_string(),
            ..Sprite::default()
        };

        assert_eq!(sprite, expected_sprite);
    }
}
