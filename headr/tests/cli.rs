use std::{fs, io::Read};

use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PROG: &str = "headr";
const EMPTY: &str = "tests/inputs/empty.txt";
const ONE: &str = "tests/inputs/one.txt";
const TWO: &str = "tests/inputs/two.txt";
const THREE: &str = "tests/inputs/three.txt";
const TEN: &str = "tests/inputs/ten.txt";

#[test]
fn usage() -> TestResult {
    for flag in ["-h", "--help"] {
        Command::cargo_bin(PROG)?
            .arg(flag)
            .assert()
            .stdout(predicates::str::contains("Usage"));
    }
    Ok(())
}

fn gen_bad_file() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(7)
        .map(char::from)
        .collect()
}

#[test]
fn skip_bad_file() -> TestResult {
    let bad_file = gen_bad_file();
    let expected = format!("{}: .* [(]os error 2[)]", bad_file);
    Command::cargo_bin(PROG)?
        .arg(bad_file)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);

    Ok(())
}

#[test]
fn illegal_negative_line_arg() -> TestResult {
    let msg = "invalid value '-1' for '-n <lines>': \
    invalid digit found in string";

    Command::cargo_bin(PROG)?
        .arg("-n=-1")
        .arg("tests/inputs/empty.txt")
        .assert()
        .stderr(predicate::str::is_match(msg)?);

    Ok(())
}

#[test]
fn illegal_negative_byte_arg() -> TestResult {
    let msg = "invalid value '-1' for '-c <bytes>': \
    invalid digit found in string";

    Command::cargo_bin(PROG)?
        .arg("-c=-1")
        .arg("tests/inputs/empty.txt")
        .assert()
        .stderr(predicate::str::is_match(msg)?);

    Ok(())
}

#[test]
fn illegal_line_and_byte_usage() -> TestResult {
    let msg = "the argument '-c <bytes>' cannot be used with '-n <lines>'";

    Command::cargo_bin(PROG)?
        .args(&["-c", "1", "-n", "1"])
        .arg("tests/inputs/empty.txt")
        .assert()
        .failure()
        .stderr(predicate::str::is_match(msg)?);

    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> TestResult {
    let mut file = fs::File::open(expected_file)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    let expected = String::from_utf8_lossy(&buf);

    Command::cargo_bin(PROG)?
        .args(args)
        .assert()
        .success()
        .stdout(predicate::eq(expected.as_bytes()));

    Ok(())
}

#[test]
fn one() -> TestResult {
    run(&[ONE], "tests/expected/one.txt.out")
}

#[test]
fn one_n2() -> TestResult {
    run(&[ONE, "-n", "2"], "tests/expected/one.txt.n2.out")
}

#[test]
fn one_n4() -> TestResult {
    run(&[ONE, "-n", "4"], "tests/expected/one.txt.n4.out")
}

#[test]
fn one_c1() -> TestResult {
    run(&[ONE, "-c", "1"], "tests/expected/one.txt.c1.out")
}

#[test]
fn one_c2() -> TestResult {
    run(&[ONE, "-c", "2"], "tests/expected/one.txt.c2.out")
}

#[test]
fn one_c4() -> TestResult {
    run(&[ONE, "-c", "4"], "tests/expected/one.txt.c4.out")
}

#[test]
fn ten() -> TestResult {
    run(&[TEN], "tests/expected/ten.txt.out")
}

#[test]
fn ten_n2() -> TestResult {
    run(&[TEN, "-n", "2"], "tests/expected/ten.txt.n2.out")
}

#[test]
fn ten_n4() -> TestResult {
    run(&[TEN, "-n", "4"], "tests/expected/ten.txt.n4.out")
}

#[test]
fn ten_c1() -> TestResult {
    run(&[TEN, "-c", "1"], "tests/expected/ten.txt.c1.out")
}

#[test]
fn ten_c2() -> TestResult {
    run(&[TEN, "-c", "2"], "tests/expected/ten.txt.c2.out")
}

#[test]
fn ten_c4() -> TestResult {
    run(&[TEN, "-c", "4"], "tests/expected/ten.txt.c4.out")
}

#[test]
fn multiple() -> TestResult {
    run(&[ONE, TWO, THREE, TEN], "tests/expected/all.txt.out")
}
