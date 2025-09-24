use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;

#[test]
fn no_args_provided() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("entropyviz2")?;

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided",
    ));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("entropyviz2")?;

    cmd.arg("test/file/doesnt/exist");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn valid_file() -> Result<(), Box<dyn std::error::Error>> {
    // Create a test file
    let input_path = "tests/test.bin";
    fs::write(input_path, &[0u8; 1024])?;

    let mut cmd = Command::cargo_bin("entropyviz2")?;
    cmd.arg(input_path)
        .arg("--output-file")
        .arg("tests/out.png");

    cmd.assert().success().stdout(predicate::str::contains(
        "Entropy chart successfully written to tests/out.png",
    ));

    assert!(fs::metadata("tests/out.png").is_ok());

    Ok(())
}

#[test]
fn help_arg_is_passed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("entropyviz2")?;

    cmd.arg("--help");
    cmd.assert().success().stdout(predicate::str::contains(
        "Usage: entropyviz2 [OPTIONS] <INPUT_FILE>",
    ));

    Ok(())
}

#[test]
fn version_arg_is_passed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("entropyviz2")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("entroplot 0.1.0"));

    Ok(())
}
