#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/not-a-closure.rs");
    t.compile_fail("tests/is-a-closure.rs");
    t.compile_fail("tests/is-a-closure-with-capture.rs");
}
