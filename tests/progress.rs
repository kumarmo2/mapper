#[test]
fn tests() {
    let ts = trybuild::TestCases::new();
    ts.pass("tests/generate_empty_implementation.rs");
    ts.pass("tests/single_source_single_field.rs");
    ts.pass("tests/single_source_multiple_field.rs");
    ts.pass("tests/source_has_more_fields.rs");
    ts.pass("tests/two_sources.rs");
    ts.pass("tests/single_source_use_fields.rs");
    ts.pass("tests/multiple_source_use_fields.rs");
    ts.pass("tests/multiple_source_with_only_one_use_field.rs");
    ts.pass("tests/source_with_qualified_name.rs");
    ts.pass("tests/with_lifetimes.rs");
    ts.pass("tests/use-fn.rs");
    ts.pass("tests/nested.rs");
}
