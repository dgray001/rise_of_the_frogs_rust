#[cfg(test)]
mod tests {
  use assert_cmd::prelude::*;
  use std::process::Command;

  #[test]
  fn it_should_run() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rise_of_the_frogs")?;

    let mut app = cmd.spawn()?;
    assert_eq!(app.try_wait()?, None, "Should be running");
    app.kill()?;
    app.wait().expect("Should end quietly");
    let exit_status = app.try_wait()?.expect("No exit status");
    assert_ne!(exit_status.code().expect("No exit code"), 0);
    
    Ok(())
  }

  #[test]
  fn it_can_be_killed() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rise_of_the_frogs")?;

    let mut app = cmd.spawn()?;
    assert_eq!(app.try_wait()?, None, "Should be running");
    app.kill()?;
    app.wait().expect("Should end quietly");
    let exit_status = app.try_wait()?.expect("No exit status");
    assert_ne!(exit_status.code().expect("No exit code"), 0);
    
    Ok(())
  }
}