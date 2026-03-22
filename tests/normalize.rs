use shirokane::elaborate::elaborate_module;
use shirokane::eval::normalize_in_module;
use shirokane::parser::parse_module;

#[test]
fn normalizes_type_id_to_identity_function() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x

typeId : Type -> Type
typeId = id Type
"#;

    let surface = parse_module(source).unwrap();
    let core = elaborate_module(&surface).unwrap();
    let term = &core.definitions[1].body;

    assert_eq!(normalize_in_module(&core, term).pretty(), "\\(x : Type) -> x");
}
