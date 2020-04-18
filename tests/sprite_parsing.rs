use anyhow::Result;
use std::num::NonZeroUsize;
use std::path::Path;
use yy_boss::boss::*;
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

    let frame_buffer = image::open("tests/examples/test.png")?.to_rgba();

    let new_sprite_yy = Sprite::new(
        "test".to_string(),
        new_yyp_boss.texture_group_controller().default_group().id,
    )
    .dimensions(
        NonZeroUsize::new(frame_buffer.width() as usize).unwrap(),
        NonZeroUsize::new(frame_buffer.height() as usize).unwrap(),
    )
    .layer(Layer::new)
    .frame(Frame::new)
    .origin(OriginUtility::MiddleCenter, true)
    .playback_speed_type(PlaybackSpeed::FramesPerSecond)
    .playback_speed(15.0)
    .collision_kind(CollisionKind::Rectangle)
    .bbox_mode(|width, height| {
        BboxModeUtility::Manual(Bbox {
            top_left: (0, 0),
            bottom_right: (width - 10, height - 10),
        })
    });

    let frame_id = new_sprite_yy.frames[0].id;

    let default_sprite = new_yyp_boss.root_sprite_folder().unwrap();
    new_yyp_boss.add_sprite(
        new_sprite_yy,
        vec![(frame_id, frame_buffer)],
        default_sprite,
    );
    new_yyp_boss.serialize()?;

    Ok(())
}
