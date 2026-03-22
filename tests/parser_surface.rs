use shirokane::parser::parse_module;

#[test]
fn parses_typed_identity_definition() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x
"#;

    let module = parse_module(source).expect("module should parse");

    assert_eq!(module.definitions.len(), 1);
    assert_eq!(module.definitions[0].name, "id");
}
