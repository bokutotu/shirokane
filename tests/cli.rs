use std::process::Command;

#[test]
fn cli_prints_normalized_definitions_for_fixture() {
    let output = Command::new(env!("CARGO_BIN_EXE_shirokane"))
        .arg("tests/fixtures/type_id.sk")
        .output()
        .expect("binary should run");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("typeId : (_ : Type) -> Type"));
    assert!(stdout.contains("typeId = \\(x : Type) -> x"));
}
