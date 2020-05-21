use anyhow::Result;
use yy_boss::{utils::TrailingCommaUtility, yy_typings::resources::sprite::*};

#[test]
fn trivial_sprite_parsing() -> Result<()> {
    let yy_example = TrailingCommaUtility::clear_trailing_comma_once(include_str!(
        "./examples/sprite_examples/jack_sprite.yy"
    ));

    let spr: Result<Sprite, _> = serde_json::from_str(&yy_example);
    println!("{:#?}", spr);

    spr?;

    Ok(())
}
