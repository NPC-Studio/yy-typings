// use pretty_assertions::{assert_eq, assert_ne};
use yy_boss::utils::TrailingCommaUtility;

#[test]
fn trivial_trailing_commas() {
    let input = TrailingCommaUtility::clear_trailing_comma_once(&"{member,}");
    assert_eq!(input, "{member}");
}

#[test]
fn trailing_commas_test() {
    let mut tcu = TrailingCommaUtility::new();
    test_harness(&mut tcu, "{member,}", "{member}");
    test_harness(&mut tcu, "{member, }", "{member }");
    test_harness(&mut tcu, "{member ,}", "{member }");
    test_harness(&mut tcu, "{member\t\n,\n\t  \t}", "{member\t\n\n\t  \t}");
    test_harness(&mut tcu, "{{member},}", "{{member}}");
    test_harness(&mut tcu, "{member}", "{member}");

    test_harness(
        &mut tcu,
        include_str!("./examples/trailing_comma/sprite_trailing.yy"),
        include_str!("./examples/trailing_comma/sprite_no_trailing.yy"),
    );
}

fn test_harness(tcu: &mut TrailingCommaUtility, input: &str, output: &str) {
    assert_eq!(tcu.clear_trailing_comma(&input), output);
}
