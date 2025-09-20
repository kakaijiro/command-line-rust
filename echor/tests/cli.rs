use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn run_with_inputs_and_outputs(){
    let outfile = "tests/expected/hello1.txt";
    let expected = fs::read_to_string(outfile).unwrap();
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("Hello there").assert().success().stdout(expected);
}

#[test]
fn run_with_expected_files() {
    // run with expected files
    let test_cases = vec![
        ("hello1.txt", vec!["Hello there"]),
        ("hello1.n.txt", vec!["-n", "Hello  there"]),
        ("hello2.txt", vec!["Hello there"]),
        ("hello2.n.txt", vec!["-n", "Hello there"]),
    ];

    for (expected_file, args) in test_cases {
        let outfile = format!("tests/expected/{}", expected_file);
        let expected = fs::read_to_string(&outfile).unwrap();
        
        let mut cmd = Command::cargo_bin("echor").unwrap();
        cmd.args(&args)
            .assert()
            .success()
            .stdout(expected);
    }
}

#[test]
fn dies_no_args(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
    .failure()
    .stderr(predicate::str::contains("Usage:"));
}

#[test]
fn dies_no_args_regex(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.assert()
    .failure()
    .stderr(predicate::str::is_match(r"error:.*required arguments").unwrap());
}

#[test]
fn runs_with_some_required_arg(){
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.arg("hello").assert().success();
}

#[test]
fn runs_with_hello_world() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["Hello", "world"])
        .assert()
        .success()
        .stdout("Hello world\n");
}

#[test]
fn runs_with_single_word() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["Hello"])
        .assert()
        .success()
        .stdout("Hello\n");
}

#[test]
fn runs_with_no_newline() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["-n", "Hello", "world"])
        .assert()
        .success()
        .stdout("Hello world");
}

#[test]
fn runs_with_multiple_words() {
    let mut cmd = Command::cargo_bin("echor").unwrap();
    cmd.args(&["This", "is", "a", "test"])
        .assert()
        .success()
        .stdout("This is a test\n");
}
