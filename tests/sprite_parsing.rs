use anyhow::Result;
use std::num::NonZeroUsize;
use std::path::Path;
use yy_boss::boss::{sprite_ext::*, YypBoss};
use yy_boss::yy_typings::resources::sprite::*;

#[test]
fn trivial_case() {
    let yyp_example: &str = include_str!("./examples/sprite.yy");

    let parse: Result<Sprite, _> = serde_json::from_str(yyp_example);
    assert!(parse.is_ok())
}

#[test]
fn adding_sprite_to_yyp() -> Result<()> {
    let yyp_path =
        Path::new("tests/examples/test_project/yy_boss_test/yy_boss_test.yyp").to_owned();
    let mut new_yyp_boss = YypBoss::new(yyp_path.to_owned())?;

    let new_sprite_yy = Sprite::new(
        "test".to_string(),
        new_yyp_boss.texture_group_controller().default_group().id,
    )
    .dimensions(
        NonZeroUsize::new(818).unwrap(),
        NonZeroUsize::new(827).unwrap(),
    )
    .layer(|sprite_id| Layer::new(sprite_id))
    .frame(|sprite| Frame::new(sprite))
    .origin(OriginUtility::MiddleCenter, true)
    .playback_speed_type(PlaybackSpeed::FramesPerSecond)
    .playback_speed(15.0)
    .collision_kind(CollisionKind::Rectangle)
    .bbox_mode(BBoxMode::Manual)
    .bbox(Bbox {
        top_left: (0, 10),
        bottom_right: (900, 800),
    });

    let frame_buffer = (
        new_sprite_yy.frames[0].id,
        image::open("tests/examples/test.png")?.to_rgba(),
    );


    new_yyp_boss.add_sprite(new_sprite_yy, vec![frame_buffer]);
    new_yyp_boss.serialize()?;

    Ok(())
}
