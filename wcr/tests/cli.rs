use assert_cmd::Command;
use std::fs;
use wcr::MyResult;

const PRG: &str = "wcr";
const FOX: &str = "tests/inputs/fox.txt";
const EMPTY: &str = "tests/inputs/empty.txt";
const ATLAMAL: &str = "tests/inputs/atlamal.txt";
// const ALL: &str = "tests/inputs/*.txt";

// ---------- helper function to run commands ----------
fn run(args: &[&str], expected_file: &str) -> MyResult<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = Command::cargo_bin(PRG)?.args(args).output().expect("fail");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);

    Ok(())
}

// ---------- tests for fox.txt ----------
#[test]
fn fox() -> MyResult<()> {
    run(&[FOX], "tests/expected/fox.txt.out")
}

#[test]
fn fox_bytes() -> MyResult<()> {
    run(&["--bytes", FOX], "tests/expected/fox.txt.c.out")
}

#[test]
fn fox_chars() -> MyResult<()> {
    run(&["--chars", FOX], "tests/expected/fox.txt.m.out")
}

#[test]
fn fox_words() -> MyResult<()> {
    run(&["--words", FOX], "tests/expected/fox.txt.w.out")
}

#[test]
fn fox_lines() -> MyResult<()> {
    run(&["--lines", FOX], "tests/expected/fox.txt.l.out")
}

#[test]
fn fox_words_bytes() -> MyResult<()> {
    run(&["-w", "-c", FOX], "tests/expected/fox.txt.wc.out")
}

#[test]
fn fox_words_lines() -> MyResult<()> {
    run(&["-w", "-l", FOX], "tests/expected/fox.txt.wl.out")
}

#[test]
fn fox_bytes_lines() -> MyResult<()> {
    run(&["-l", "-c", FOX], "tests/expected/fox.txt.cl.out")
}

// ---------- tests for all files ----------
#[test]
fn test_all() -> MyResult<()> {
    run(&[EMPTY, FOX, ATLAMAL], "tests/expected/all.out")
}

// test doesn't work if the input includes wildcard characters.
// #[test]
// fn test_all_with_standard_input() -> MyResult<()> {
//     run(&[ALL], "tests/expected/all.standard_input.out")
// }