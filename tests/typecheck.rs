use shirokane::elaborate::elaborate_module;
use shirokane::parser::parse_module;
use shirokane::typecheck::check_module;

#[test]
fn typechecks_identity_and_application_to_type() {
    let source = r#"id : (A : Type) -> A -> A
id = \(A : Type) -> \(x : A) -> x

typeId : Type -> Type
typeId = id Type
"#;

    let surface = parse_module(source).unwrap();
    let core = elaborate_module(&surface).unwrap();
    let checked = check_module(&core).expect("module should typecheck");

    assert_eq!(checked.definitions.len(), 2);
    assert_eq!(checked.definitions[1].ty.pretty(), "(_ : Type) -> Type");
}
