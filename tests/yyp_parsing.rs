use yy_boss::{utils::TrailingCommaUtility, yy_typings::Yyp};

#[test]
fn trivial_yyp_parse() {
    let mut tcu = TrailingCommaUtility::new();
    let yyp_example =
        tcu.clear_trailing_comma(include_str!("./examples/yyp_examples/yyp_test0.yyp"));

    let _: Yyp = serde_json::from_str(&yyp_example).unwrap();
}
