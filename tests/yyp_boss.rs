use include_dir::{include_dir, Dir, DirEntry};
use pretty_assertions::assert_eq;
use std::path::Path;
use yy_boss::{
    boss::{SpriteExt, YypBoss},
    utils::TrailingCommaUtility,
    yy_typings::{
        sprite::{FrameId, Sprite},
        Yyp,
    },
};

const PATH_TO_TEST_PROJ: &'static str = "tests/examples/yyp_boss/test_proj/anchor.yyp";

/// The purpose of this test is to make sure that the YypBoss can
/// take in YYPs without breaking those YYPs.
#[test]
fn no_mangle_yyp() {
    // We cannot save this string into a constant, due to issues with the
    // `include_dir` macro.
    let root_path = Path::new("tests/examples/yyp_boss/project_database");
    let all_yyps: Dir = include_dir!("tests/examples/yyp_boss/project_database");
    let tcu = TrailingCommaUtility::new();

    for yyps in all_yyps.find("**/*.yyp").unwrap().skip(1) {
        match yyps {
            DirEntry::File(file) => {
                let path = root_path.join(file.path);
                let parsed_yyp: Yyp = YypBoss::new(&path).unwrap().into();

                let original = std::str::from_utf8(file.contents).unwrap();
                let original_pure_parsed_yyp: Yyp =
                    serde_json::from_str(&tcu.clear_trailing_comma(original)).unwrap();

                assert_eq!(parsed_yyp, original_pure_parsed_yyp);
            }
            _ => {}
        }
    }
}

const IMAGE_PATH: &'static str = "tests/examples/yyp_boss/test_spr_add.png";

#[test]
fn add_sprite_to_yyp() {
    let mut yyp_boss = YypBoss::new(Path::new(PATH_TO_TEST_PROJ)).unwrap();
    let new_view = yyp_boss
        .add_folder("Sprites".to_string(), yyp_boss.root_path())
        .unwrap();

    let single_frame_id = FrameId::new();
    let sprite = Sprite::new("test_spr_add", "Default")
        .parent(new_view)
        .frame(single_frame_id)
        .bbox_mode(|_, _| yy_boss::boss::BboxModeUtility::FullImage);

    let frame_buffer = image::open(IMAGE_PATH).unwrap().to_rgba();
    yyp_boss.add_sprite(sprite, vec![(single_frame_id, frame_buffer)]);
}
