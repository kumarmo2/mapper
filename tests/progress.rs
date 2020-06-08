#[test]
fn tests() {
    let ts = trybuild::TestCases::new();
    ts.pass("tests/generate_empty_implementation.rs");
}
