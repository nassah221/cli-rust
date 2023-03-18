use std::fs;

use assert_cmd::Command;
use predicates::prelude::predicate;

#[test]
fn dies_no_args() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn run_tests() -> TestResult {
    struct Test<'a> {
        outfile: &'a str,
        cmd_args: &'a [&'a str],
    }
    let tests: Vec<Test> = vec![
        Test {
            outfile: "tests/expected/hello1.txt",
            cmd_args: &["Hello there"],
        },
        Test {
            outfile: "tests/expected/hello2.txt".into(),
            cmd_args: &["Hello", "there"],
        },
        Test {
            outfile: "tests/expected/hello1.n.txt".into(),
            cmd_args: &["Hello  there", "-n"],
        },
        Test {
            outfile: "tests/expected/hello2.n.txt".into(),
            cmd_args: &["Hello", "there", "-n"],
        },
    ];

    for test in tests.iter() {
        let expected = fs::read_to_string(test.outfile)?;
        let mut cmd = Command::cargo_bin("echor").unwrap();
        cmd.args(test.cmd_args).assert().success().stdout(expected);
    }

    Ok(())
}
