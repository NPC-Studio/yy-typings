use yy_boss::yyp::Yyp;

#[test]
fn trivial_case() {
    let yyp_example: &str = include_str!("./examples/example.yyp");

    let parse: Result<Yyp, _> = serde_json::from_str(yyp_example);
    println!("{:#?}", parse);
    assert!(parse.is_ok())
}
