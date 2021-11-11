use assert_cmd::prelude::*; // Add methods on commands
use std::process::Command; // Run programs

#[test]
fn state_command_nominal_case() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yacr")?;
    cmd.arg("state").arg("id123");
    cmd.assert().success();
    Ok(())
}

#[test]
fn create_command_nominal_case() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yacr")?;
    cmd.arg("create").arg("id123").arg("test/path");
    cmd.assert().success();
    Ok(())
}

#[test]
fn start_command_nominal_case() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yacr")?;
    cmd.arg("start").arg("id123");
    cmd.assert().success();
    Ok(())
}

#[test]
fn delete_command_nominal_case() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("yacr")?;
    cmd.arg("delete").arg("id123");
    cmd.assert().success();
    Ok(())
}
