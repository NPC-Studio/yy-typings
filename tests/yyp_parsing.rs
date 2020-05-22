use include_dir::{include_dir, Dir, DirEntry};
use yy_boss::{utils::TrailingCommaUtility, yy_typings::Yyp};

#[test]
fn trivial_yyp_parse() {
    let all_sprites: Dir = include_dir!("tests/examples/yyp_examples");
    let mut tcu = TrailingCommaUtility::new();
    

    for sprite_file in all_sprites.find("**/*.yyp").unwrap() {
        match sprite_file {
            DirEntry::File(file) => {
                let our_str = std::str::from_utf8(file.contents()).unwrap();
                let our_str = tcu.clear_trailing_comma(our_str);
                let _: Yyp = serde_json::from_str(&our_str).unwrap();
            }
            _ => {}
        }
    }
}
