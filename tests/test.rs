use std::{
    io::{self, Write},
    process::Command,
};

struct Rule {
    check: String,
    name: String,
}

#[test]
fn staticcheck() {
    go_install("honnef.co/go/tools/cmd/staticcheck@latest");

    let rules = [Rule {
        check: String::from("SA1000"),
        name: String::from("CheckRegexps"),
    }];

    for rule in rules {
        let path = format!(
            "tests/go-tools/staticcheck/testdata/src/example.com/{}/{}.go",
            rule.name, rule.name
        );

        let output = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .arg(&path)
            .output()
            .unwrap();
        let a = String::from_utf8(output.stdout).unwrap();

        let output = Command::new("staticcheck")
            .arg("-checks")
            .arg(rule.check)
            .arg(&path)
            .output()
            .unwrap();
        let b = String::from_utf8(output.stdout).unwrap();

        assert_eq!(a, b);
    }
}

fn go_install(package: &str) {
    let output = Command::new("go")
        .arg("install")
        .arg(package)
        .output()
        .expect("failed to install");

    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();
}
