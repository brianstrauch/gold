use pretty_assertions::assert_eq;
use std::{fs, process::Command};

#[test]
fn test() {
    let golden = fs::read_to_string("tests/output.golden").unwrap();

    let output = Command::new("cargo")
        .arg("run")
        .arg("tests")
        .output()
        .unwrap();
    let stdout = String::from_utf8(output.stdout).unwrap();

    assert_eq!(golden, stdout);
}
