use include_dir::{include_dir, Dir, DirEntry};
use std::path::Path;
use yy_boss::{boss::YypBoss, utils::TrailingCommaUtility, yy_typings::Yyp};

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
