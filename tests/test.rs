use std::{path::Path, process::Command};

struct Rule {
    check: String,
    name: String,
}

#[test]
fn staticcheck() {
    Command::new("cargo")
        .arg("build")
        .arg("--release")
        .output()
        .unwrap();

    clone("dominikh", "go-tools");

    let rules = [Rule {
        check: String::from("SA1000"),
        name: String::from("CheckRegexps"),
    }];

    for rule in rules {
        let path = format!(
            "tests/go-tools/staticcheck/testdata/src/example.com/{}/{}.go",
            rule.name, rule.name
        );

        let output = Command::new("target/release/gold")
            .arg(&path)
            .output()
            .unwrap();
        let a = String::from_utf8(output.stdout).unwrap();

        let output = Command::new("tests/bin/staticcheck")
            .arg("-checks")
            .arg(&rule.check)
            .arg(&path)
            .output()
            .unwrap();
        let b = String::from_utf8(output.stdout).unwrap();

        assert_eq!(a, b);
    }
}

fn clone(owner: &str, repo: &str) {
    let dir = format!("tests/{}", repo);

    if Path::new(&dir).exists() {
        return;
    }

    Command::new("git")
        .arg("clone")
        .arg(format!("git@github.com:{}/{}.git", owner, repo))
        .arg(dir)
        .output()
        .expect("failed to clone go-tools");

    Command::new("go")
        .arg("build")
        .arg("-C")
        .arg("tests/go-tools")
        .arg("-o")
        .arg("../bin/staticcheck")
        .arg("cmd/staticcheck/staticcheck.go")
        .output()
        .unwrap();
}
