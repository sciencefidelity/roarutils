use std::{borrow::Cow, fs, path::Path};

use anyhow::Result;
use assert_cmd::Command;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::{Rng, distributions::Alphanumeric};
use serial_test::serial;

const PRG: &str = "find";

fn gen_bad_file() -> String {
    loop {
        let filename: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[serial]
#[test]
fn skips_bad_dir() -> Result<()> {
    let bad = gen_bad_file();
    let expected = format!("{}: .* [(]os error [23][)]", &bad);
    Command::cargo_bin(PRG)?
        .arg(&bad)
        .assert()
        .success()
        .stderr(predicate::str::is_match(expected)?);
    Ok(())
}

#[serial]
#[test]
fn dies_bad_name() -> Result<()> {
    Command::cargo_bin(PRG)?
        .args(["--name", "*.csv"])
        .assert()
        .failure()
        .stderr(predicate::str::contains("error: invalid value '*.csv'"));
    Ok(())
}

#[serial]
#[test]
fn dies_bad_type() -> Result<()> {
    let expected = "error: invalid value 'x' for '--type [<TYPE>...]'";
    Command::cargo_bin(PRG)?
        .args(["--type", "x"])
        .assert()
        .failure()
        .stderr(predicate::str::contains(expected));
    Ok(())
}

#[cfg(windows)]
fn format_file_name(expected_file: &str) -> Cow<'_, str> {
    // Equivalent to: Cow::Owned(format!("{expected_file}.windows"))
    format!("{expected_file}.windows").into()
}

#[cfg(not(windows))]
fn format_file_name(expected_file: &str) -> Cow<'_, str> {
    // Equivalent to: Cow::Borrowed(expected_file)
    expected_file.into()
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let file = format_file_name(expected_file);
    let contents = fs::read_to_string(file.as_ref())?;
    let mut expected: Vec<&str> = contents.split('\n').filter(|s| !s.is_empty()).collect();
    expected.sort_unstable();

    let cmd = Command::cargo_bin(PRG)?.args(args).assert().success();
    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;
    let mut lines: Vec<&str> = stdout.split('\n').filter(|s| !s.is_empty()).collect();
    lines.sort_unstable();

    assert_eq!(lines, expected);

    Ok(())
}

#[serial]
#[test]
fn path1() -> Result<()> {
    run(&["tests/inputs"], "tests/expected/path1.txt")
}

#[serial]
#[test]
fn path_a() -> Result<()> {
    run(&["tests/inputs/a"], "tests/expected/path_a.txt")
}

#[serial]
#[test]
fn path_a_b() -> Result<()> {
    run(&["tests/inputs/a/b"], "tests/expected/path_a_b.txt")
}

#[serial]
#[test]
fn path_d() -> Result<()> {
    run(&["tests/inputs/d"], "tests/expected/path_d.txt")
}

#[serial]
#[test]
fn path_a_b_d() -> Result<()> {
    run(
        &["tests/inputs/a/b", "tests/inputs/d"],
        "tests/expected/path_a_b_d.txt",
    )
}

#[serial]
#[test]
fn type_f() -> Result<()> {
    run(&["tests/inputs", "-t", "f"], "tests/expected/type_f.txt")
}

#[serial]
#[test]
fn type_f_path_a() -> Result<()> {
    run(
        &["tests/inputs/a", "-t", "f"],
        "tests/expected/type_f_path_a.txt",
    )
}

#[serial]
#[test]
fn type_f_path_a_b() -> Result<()> {
    run(
        &["tests/inputs/a/b", "--type", "f"],
        "tests/expected/type_f_path_a_b.txt",
    )
}

#[serial]
#[test]
fn type_f_path_d() -> Result<()> {
    run(
        &["tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_d.txt",
    )
}

#[serial]
#[test]
fn type_f_path_a_b_d() -> Result<()> {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "f"],
        "tests/expected/type_f_path_a_b_d.txt",
    )
}

#[serial]
#[test]
fn type_d() -> Result<()> {
    run(&["tests/inputs", "-t", "d"], "tests/expected/type_d.txt")
}

#[serial]
#[test]
fn type_d_path_a() -> Result<()> {
    run(
        &["tests/inputs/a", "-t", "d"],
        "tests/expected/type_d_path_a.txt",
    )
}

#[serial]
#[test]
fn type_d_path_a_b() -> Result<()> {
    run(
        &["tests/inputs/a/b", "--type", "d"],
        "tests/expected/type_d_path_a_b.txt",
    )
}

#[serial]
#[test]
fn type_d_path_d() -> Result<()> {
    run(
        &["tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_d.txt",
    )
}

#[serial]
#[test]
fn type_d_path_a_b_d() -> Result<()> {
    run(
        &["tests/inputs/a/b", "tests/inputs/d", "--type", "d"],
        "tests/expected/type_d_path_a_b_d.txt",
    )
}

#[serial]
#[test]
fn type_l() -> Result<()> {
    run(&["tests/inputs", "-t", "l"], "tests/expected/type_l.txt")
}

#[serial]
#[test]
fn type_f_l() -> Result<()> {
    run(
        &["tests/inputs", "-t", "l", "f"],
        "tests/expected/type_f_l.txt",
    )
}

#[serial]
#[test]
fn name_csv() -> Result<()> {
    run(
        &["tests/inputs", "-n", ".*[.]csv"],
        "tests/expected/name_csv.txt",
    )
}

#[serial]
#[test]
fn name_csv_mp3() -> Result<()> {
    run(
        &["tests/inputs", "-n", ".*[.]csv", "-n", ".*[.]mp3"],
        "tests/expected/name_csv_mp3.txt",
    )
}

#[serial]
#[test]
fn name_txt_path_a_d() -> Result<()> {
    run(
        &["tests/inputs/a", "tests/inputs/d", "--name", ".*.txt"],
        "tests/expected/name_txt_path_a_d.txt",
    )
}

#[serial]
#[test]
fn name_a() -> Result<()> {
    run(&["tests/inputs", "-n", "a"], "tests/expected/name_a.txt")
}

#[serial]
#[test]
fn type_f_name_a() -> Result<()> {
    run(
        &["tests/inputs", "-t", "f", "-n", "a"],
        "tests/expected/type_f_name_a.txt",
    )
}

#[serial]
#[test]
fn type_d_name_a() -> Result<()> {
    run(
        &["tests/inputs", "--type", "d", "--name", "a"],
        "tests/expected/type_d_name_a.txt",
    )
}

#[serial]
#[test]
fn path_g() -> Result<()> {
    run(&["tests/inputs/g.csv"], "tests/expected/path_g.txt")
}

#[serial]
#[test]
#[cfg(not(windows))]
fn unreadable_dir() -> Result<()> {
    let dirname = "tests/inputs/cant-touch-this";
    if !Path::new(dirname).exists() {
        fs::create_dir(dirname)?;
    }

    std::process::Command::new("chmod")
        .args(["000", dirname])
        .status()
        .expect("failed");

    let cmd = Command::cargo_bin(PRG)?
        .arg("tests/inputs")
        .assert()
        .success();
    fs::remove_dir(dirname)?;

    let out = cmd.get_output();
    let stdout = String::from_utf8(out.stdout.clone())?;

    assert_eq!(stdout.split('\n').filter(|s| !s.is_empty()).count(), 17);

    let stderr = String::from_utf8(out.stderr.clone())?;
    assert!(stderr.contains("cant-touch-this: Permission denied"));
    Ok(())
}
