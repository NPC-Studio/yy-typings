use anyhow::Result;
use yy_boss::{utilities::TrailingCommaUtility, yy_typings::resources::sprite::*};

#[test]
fn trivial_sprite_parsing() -> Result<()> {
    let mut yy_example: String =
        include_str!("./examples/sprite_examples/jack_sprite.yy").to_string();
    TrailingCommaUtility::clear_trailing_comma_once(&mut yy_example);

    let spr: Result<Sprite, _> = serde_json::from_str(&yy_example);
    println!("{:#?}", spr);

    spr?;

    Ok(())
}
