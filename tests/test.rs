#[macro_use]
extern crate duct;

use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn test() {
    let output = cmd!("cargo", "run", "--quiet", "tests")
        .unchecked()
        .stderr_to_stdout()
        .stdout_capture()
        .run()
        .unwrap();

    assert_eq!(
        String::from_utf8(output.stdout).unwrap(),
        fs::read_to_string("tests/output.golden").unwrap()
    );

    assert_eq!(output.status.success(), false);
}
