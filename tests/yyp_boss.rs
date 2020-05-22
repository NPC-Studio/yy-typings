use include_dir::{include_dir, Dir, DirEntry};
use std::path::Path;
use yy_boss::{boss::YypBoss, yy_typings::ViewPath};

#[test]
fn trivial_yyp_boss() {
    let root_path = Path::new("tests/examples/yyp_boss");
    let all_yyps: Dir = include_dir!("tests/examples/yyp_boss");

    for yyps in all_yyps.find("**/*.yyp").unwrap().skip(1) {
        match yyps {
            DirEntry::File(file) => {
                let path = root_path.join(file.path);
                let mut yyp_boss = YypBoss::new(&path).unwrap();

                let new_view = yyp_boss
                    .add_folder(
                        "where we droppin squad".to_string(),
                        ViewPath {
                            name: "folders/Library".to_string(),
                            path: Path::new("folders/Library").to_owned(),
                        },
                    )
                    .unwrap();

                println!("New View: {:#?}", new_view);
                println!("{:#?}", yyp_boss.root_folder());
                println!("{:#?}", yyp_boss.folder(&new_view));

                break;
            }
            _ => {}
        }
    }
    panic!("ignore");
}
