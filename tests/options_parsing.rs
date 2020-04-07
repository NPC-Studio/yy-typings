use pretty_assertions::assert_eq;
use yy_boss::{
    boss::texture_group_ext::TextureGroupExt, yy_typings::resources::texture_group::TextureGroup,
};

#[test]
fn test_regex_parsing() {
    let options_text = include_str!("./examples/options_example.yy");

    let read_values = TextureGroup::parse_options_file(options_text).unwrap();

    assert_eq!(read_values.len(), 3);
}
