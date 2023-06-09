use pretty_assertions::assert_eq;
use std::{fs, process::Command};

#[test]
fn test() {
    for rule in ["G0000", "G0001", "SA1000"] {
        let golden = fs::read_to_string(format!("tests/{rule}/{rule}.golden")).unwrap();

        let output = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg(format!("tests/{rule}"))
            .output()
            .unwrap();
        let stdout = String::from_utf8(output.stdout).unwrap();

        assert_eq!(golden, stdout);
    }
}
