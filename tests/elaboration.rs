use shirokane::elaborate::elaborate_module;
use shirokane::parser::parse_module;

#[test]
fn elaborates_identity_signature_and_body() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x
"#;

    let surface = parse_module(source).unwrap();
    let core = elaborate_module(&surface).unwrap();

    assert_eq!(core.definitions.len(), 1);
    assert_eq!(core.definitions[0].name, "id");
    assert_eq!(core.definitions[0].ty.pretty(), "(A : Type) -> (_ : A) -> A");
    assert_eq!(core.definitions[0].body.pretty(), "\\(A : Type) -> \\(x : A) -> x");
}
