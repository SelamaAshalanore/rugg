extern crate staticanalyzer;

#[test]
fn test_realization() {
    assert_eq!(
        staticanalyzer::rs2dot("tests/examples/realization.rs"),
        r#"digraph ast {
    A[label="{A|a: T|a(a: T) -> Self\la(&self) -> Option<T>}"][shape="record"];
    B[label="{Interface\lB|a(&self) -> Option<T>"][shape="record"];
    B -> A[label=""][style="dashed"][arrowhead="onormal"];
}
"#);
}