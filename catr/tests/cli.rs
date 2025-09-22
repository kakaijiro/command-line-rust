use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use rand::Rng;

type TestResult = Result<(), Box<dyn std::error::Error>>;

fn gen_bad_file() -> String {
    let mut rng = rand::rng();
    let filename: String = (0..7)
        .map(|_| {
            let chars = b"abcdefghijklmnopqrstuvwxyz0123456789";
            chars[rng.random_range(0..chars.len())] as char
        })
        .collect();

    if fs::metadata(&filename).is_err() {
        return filename;
    }
    
    // If file exists, try a different name
    format!("{}_{}", filename, rng.random_range(1000..9999))
}

#[test]
fn skip_bad_file() -> TestResult {
    let bad = gen_bad_file();
    println!("Testing with bad filename: {}", bad);
    let expected = format!("{}: .* [(]os error 2[)]", bad);
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[test]
fn run_with_stdin() -> TestResult {
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.write_stdin("Hello from stdin\n")
    .assert()
    .success()
    .stdout("Hello from stdin\n");
    Ok(())
}

#[test]
fn run_with_empty_file() -> TestResult {
    let expected = fs::read_to_string("tests/expected/empty.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["tests/inputs/empty.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_empty_file_with_number_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/empty.n.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-n","tests/inputs/empty.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_empty_file_with_number_nonblank_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/empty.b.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-b", "tests/inputs/empty.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_one_file() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["tests/inputs/fox.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_one_file_with_number_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/fox.n.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-n","tests/inputs/fox.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_two_files_with_number_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/spiders_and_the-bustle.n.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-n","tests/inputs/spiders.txt", "tests/inputs/the-bustle.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_two_files_with_number_nonblank_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/spiders_and_the-bustle.b.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-b","tests/inputs/spiders.txt", "tests/inputs/the-bustle.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_one_file_of_multiple_lines_with_number_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/spiders.n.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-n","tests/inputs/spiders.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_one_file_of_multiple_lines_with_number_nonblank_lines() -> TestResult {
    let expected = fs::read_to_string("tests/expected/spiders.b.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-b","tests/inputs/spiders.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_one_file_of_multiple_lines_with_number_nonblank_lines_case2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/the-bustle.b.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-b","tests/inputs/the-bustle.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}

#[test]
fn run_with_one_file_of_multiple_lines_with_number_lines_case2() -> TestResult {
    let expected = fs::read_to_string("tests/expected/the-bustle.n.txt")?;
    let mut cmd = Command::cargo_bin("catr")?;
    cmd.args(&["-n","tests/inputs/the-bustle.txt"])
    .assert()
    .success()
    .stdout(expected);
    Ok(())
}