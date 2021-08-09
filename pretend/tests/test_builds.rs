#[test]
#[cfg(not(tarpaulin))]
fn test_builds() {
    let version = rustc_version::version().unwrap();

    let t = trybuild::TestCases::new();

    if version.minor < 54 {
        t.compile_fail("tests/builds-pre-1.54/*.rs");
    } else {
        t.compile_fail("tests/builds/*.rs");
    }
}
