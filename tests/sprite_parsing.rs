use anyhow::Result;
use include_dir::{include_dir, Dir, DirEntry};
use pretty_assertions::assert_eq;
use std::{num::NonZeroUsize, path::Path};
use yy_boss::{
    boss::{Bbox, BboxModeUtility, OriginUtility, SpriteExt},
    utils::TrailingCommaUtility,
    yy_typings::resources::sprite::*,
};

#[test]
fn trivial_sprite_parsing() -> Result<()> {
    let all_sprites: Dir = include_dir!("tests/examples/sprite_examples");
    let mut tcu = TrailingCommaUtility::new();

    for sprite_file in all_sprites.find("**/*.yy").unwrap() {
        match sprite_file {
            DirEntry::File(file) => {
                let our_str = std::str::from_utf8(file.contents())?;
                let our_str = tcu.clear_trailing_comma(our_str);
                let _: Sprite = serde_json::from_str(&our_str)?;
            }
            _ => {}
        }
    }

    Ok(())
}

#[test]
fn deep_check() {
    let sprite = include_str!("./examples/sprite_examples/test0.yy");
    let sprite: Sprite =
        serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(sprite)).unwrap();

    let expected_sprite = Sprite {
        bbox_mode: BBoxMode::Automatic,
        collision_kind: CollisionKind::Rectangle,
        separate_masks: false,
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
        texture_group_id: TextureGroupPath {
            name: "Default".to_string(),
            path: Path::new("texturegroups/Default").to_owned(),
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
                                    uuid::Uuid::parse_str("7ac201d7-c56c-400f-8f97-c42ad0d98fba")
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
                                    uuid::Uuid::parse_str("d457b008-1cb9-43a5-8024-7576b09a0421")
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
                ..Default::default()
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
            ..Default::default()
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
            path: Path::new("folders/Sprites.yy").to_owned(),
        },
        name: "spr_jack".to_string(),
        ..Sprite::default()
    };

    assert_eq!(sprite, expected_sprite);
}

#[test]
fn deep_check_builder() {
    let sprite = include_str!("./examples/sprite_examples/test0.yy");
    let sprite: Sprite =
        serde_json::from_str(&TrailingCommaUtility::clear_trailing_comma_once(sprite)).unwrap();

    let expected_sprite = Sprite::new("spr_jack", "Default")
        .parent(ViewPath {
            name: "Sprites".to_string(),
            path: Path::new("folders/Sprites.yy").to_owned(),
        })
        .with(|spr| {
            // Align with `test.0.yy` file...
            let mut layer: &mut Layer = &mut spr.layers[0];
            layer.display_name = "Layer 1 (2)".to_string();
            layer.name = LayerId::with_string("37cbf63f-f6a9-4b91-a9fd-3537b374e9db");

            spr.sequence.visible_range = Some(VisibleRange { x: 0.0, y: 0.0 });
        })
        .dimensions(
            NonZeroUsize::new(48).unwrap(),
            NonZeroUsize::new(48).unwrap(),
        )
        .bbox_mode(|_, _| {
            BboxModeUtility::Automatic(Bbox {
                top_left: (7, 5),
                bottom_right: (38, 47),
            })
        })
        .collision_kind(CollisionKind::Rectangle)
        .origin(OriginUtility::Custom { x: 23, y: 42 }, false)
        .frame(FrameId::with_string("7669a695-40b5-47eb-a089-f81c0f6be6b8"))
        .frame(FrameId::with_string("a52625ab-2499-4b46-9785-58ee50a1b048"))
        .with(|spr| {
            let track: &mut Track = &mut spr.sequence.tracks[0];
            track.keyframes.keyframes[0].id =
                SpriteSequenceId::with_string("7ac201d7-c56c-400f-8f97-c42ad0d98fba");
            track.keyframes.keyframes[1].id =
                SpriteSequenceId::with_string("d457b008-1cb9-43a5-8024-7576b09a0421");
        });

    assert_eq!(sprite, expected_sprite)
}
