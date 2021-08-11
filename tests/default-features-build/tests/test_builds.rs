#[test]
fn test_builds() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/builds/*.rs");
}
