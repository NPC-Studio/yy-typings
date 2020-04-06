use yy_boss::yy_typings::resources::sprite::Sprite;

#[test]
fn trivial_case() {
    let yyp_example: &str = include_str!("./examples/sprite.yy");

    let parse: Result<Sprite, _> = serde_json::from_str(yyp_example);
    println!("{:#?}", parse);
    assert!(parse.is_ok())
}
