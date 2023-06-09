use assert_cmd::Command;
use predicates::prelude::*;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PROG: &str = "catr";
const EMPTY: &str = "tests/inputs/empty.txt";
const SPIDER: &str = "tests/inputs/spider.txt";
const FOX: &str = "tests/inputs/fox.txt";
const BUSTLE: &str = "tests/inputs/bustle.txt";

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

fn run(args: &[&str], expected: &str) -> TestResult {
    let expected = fs::read_to_string(expected)?;
    Command::cargo_bin(PROG)?
        .args(args)
        .assert()
        .success()
        .stdout(expected);

    Ok(())
}

fn run_stdin(input_file: &str, args: &[&str], expected_file: &str) -> TestResult {
    let input = fs::read_to_string(input_file)?;
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin(PROG)?
        .args(args)
        .write_stdin(input)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn bustle() -> TestResult {
    run(&[BUSTLE], "tests/expected/bustle.txt.out")
}

#[test]
fn bustle_n() -> TestResult {
    run(&["-n", BUSTLE], "tests/expected/bustle.txt.n.out")
}

#[test]
fn bustle_b() -> TestResult {
    run(&["-b", BUSTLE], "tests/expected/bustle.txt.b.out")
}

#[test]
fn bustle_stdin() -> TestResult {
    run_stdin(BUSTLE, &["-"], "tests/expected/bustle.txt.stdin.out")
}

#[test]
fn bustle_stdin_n() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-n", "-"],
        "tests/expected/bustle.txt.n.stdin.out",
    )
}

#[test]
fn bustle_stdin_b() -> TestResult {
    run_stdin(
        BUSTLE,
        &["-b", "-"],
        "tests/expected/bustle.txt.b.stdin.out",
    )
}

#[test]
fn empty() -> TestResult {
    run(&[EMPTY], "tests/expected/empty.txt.out")
}

#[test]
fn empty_n() -> TestResult {
    run(&["-n", EMPTY], "tests/expected/empty.txt.n.out")
}

#[test]
fn empty_b() -> TestResult {
    run(&["-b", EMPTY], "tests/expected/empty.txt.b.out")
}

#[test]
fn fox() -> TestResult {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_n() -> TestResult {
    run(&["-n", FOX], "tests/expected/fox.txt.n.out")
}

#[test]
fn fox_b() -> TestResult {
    run(&["-b", FOX], "tests/expected/fox.txt.b.out")
}

#[test]
fn spiders() -> TestResult {
    run(&[SPIDER], "tests/expected/spider.txt.out")
}

#[test]
fn spiders_n() -> TestResult {
    run(&["-n", SPIDER], "tests/expected/spider.txt.n.out")
}

#[test]
fn spiders_b() -> TestResult {
    run(&["-b", SPIDER], "tests/expected/spider.txt.b.out")
}

#[test]
fn all() -> TestResult {
    run(&[SPIDER, FOX, BUSTLE], "tests/expected/all.out")
}

#[test]
fn all_n() -> TestResult {
    run(&[SPIDER, FOX, BUSTLE, "-n"], "tests/expected/all.n.out")
}

#[test]
fn all_b() -> TestResult {
    run(&[SPIDER, FOX, BUSTLE, "-b"], "tests/expected/all.b.out")
}
