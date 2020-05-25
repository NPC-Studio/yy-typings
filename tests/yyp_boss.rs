use include_dir::{include_dir, Dir, DirEntry};
// use pretty_assertions::assert_eq;
use std::path::Path;
use yy_boss::{
    boss::{SpriteExt, YypBoss},
    utils::TrailingCommaUtility,
    yy_typings::{
        sprite::{FrameId, Layer, LayerId, Sprite, SpriteKeyframe, SpriteSequenceId, Track},
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
                let parsed_yyp = YypBoss::new(&path).unwrap().yyp().clone();

                let original = std::str::from_utf8(file.contents).unwrap();
                let original_pure_parsed_yyp: Yyp =
                    serde_json::from_str(&tcu.clear_trailing_comma(original)).unwrap();

                assert_eq!(parsed_yyp, original_pure_parsed_yyp);
            }
            _ => {}
        }
    }
}

#[test]
fn add_sprite_to_yyp() {
    const IMAGE_PATH: &'static str = "tests/examples/yyp_boss/test_spr_add.png";
    const PROOF_PATH: &'static str = "tests/examples/yyp_boss/proofs/sprite_add/anchor.yyp";

    let mut yyp_boss = YypBoss::new(Path::new(PATH_TO_TEST_PROJ)).unwrap();
    let new_view = yyp_boss
        .add_folder("Sprites".to_string(), yyp_boss.root_path())
        .unwrap();

    let single_frame_id = FrameId::with_string("a63f66a2-abbc-43e4-a1fc-0e17dc1a30c0");
    let sprite = Sprite::with_layer(
        "test_spr_add",
        "Default",
        Layer {
            name: LayerId::with_string("9df2e079-6445-4bc9-b094-8be99b784f16"),
            ..Layer::default()
        },
    )
    .parent(new_view)
    .frame(single_frame_id)
    .with(|spr| {
        let track: &mut Track = &mut spr.sequence.tracks[0];
        let kf: &mut SpriteKeyframe = &mut track.keyframes.keyframes[0];
        kf.id = SpriteSequenceId::with_string("eb4e1436-fd08-462b-af20-eea01b4d86f1");
    })
    .bbox_mode(|_, _| yy_boss::boss::BboxModeUtility::FullImage);

    let frame_buffer = image::open(IMAGE_PATH).unwrap().to_rgba();
    yyp_boss.add_sprite(sprite, vec![(single_frame_id, frame_buffer)]);

    let proof_yyp_boss = YypBoss::new(Path::new(PROOF_PATH)).unwrap();

    // Assert the our YYPs are the Same...
    let our_yyp = yyp_boss.yyp();
    let proof_yyp = proof_yyp_boss.yyp();
    assert_eq!(our_yyp, proof_yyp, "Yyps are not the Same!");

    let our_graph = yyp_boss.root_folder();
    let other_graph = proof_yyp_boss.root_folder();
    assert_eq!(our_graph, other_graph, "Folder Graphs are not the Same!");
}
