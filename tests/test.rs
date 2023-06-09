use std::process::Command;

struct Rule {
    check: String,
    name: String,
}

#[test]
fn staticcheck() {
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
