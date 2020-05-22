use include_dir::{include_dir, Dir, DirEntry};
use std::path::Path;
use yy_boss::boss::YypBoss;

#[test]
fn trivial_yyp_boss() {
    let root_path = Path::new("tests/examples/yyp_boss");
    let all_yyps: Dir = include_dir!("tests/examples/yyp_boss");

    for yyps in all_yyps.find("**/*.yyp").unwrap() {
        match yyps {
            DirEntry::File(file) => {
                let path = root_path.join(file.path);
                YypBoss::new(&path).unwrap();
            }
            _ => {}
        }
    }
}
