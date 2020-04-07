use anyhow::Result;
use pretty_assertions::assert_eq;
use std::path::{Path, PathBuf};
use yy_boss::boss::{sprite_ext::*, YypBoss};
use yy_boss::yy_typings::resources::sprite::Sprite;

#[test]
fn trivial_case() {
    let yyp_example: &str = include_str!("./examples/sprite.yy");

    let parse: Result<Sprite, _> = serde_json::from_str(yyp_example);
    assert!(parse.is_ok())
}

#[test]
fn adding_sprite_to_yyp() -> Result<()> {
    let mut new_yyp_boss = YypBoss::new(
        Path::new("./examples/test_project/yy_boss_test/yy_boss_test.yyp").to_owned(),
    )?;

    let mut new_sprite = Sprite::new("juniper_test.png".to_string());
    // new_sprite.

    // new_yyp_boss.add_sprite()

    // Ok(())
}
