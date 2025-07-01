use crate::Result;
use std::process::Command;

pub fn make_args(args: &[&str]) -> Vec<String> {
    args.iter().copied().map(|arg| arg.to_owned()).collect()
}

pub fn execute(bin: &str, args: &[String], vars: Vec<(&str, &str)>) -> Result<()> {
    let _span = error_span!("execute", bin, ?args, ?vars).entered();
    let mut cmd = Command::new(bin);
    cmd.envs(vars).args(args);
    execute_cmd(cmd)
}

pub fn execute_cmd(mut cmd: Command) -> Result<()> {
    let output = cmd.output()?;
    let success = output.status.success();

    let _span = error_span!(
        "output",
        success,
        stdout = %String::from_utf8_lossy(&output.stdout),
        stderr = %String::from_utf8_lossy(&output.stderr)
    )
    .entered();

    debug!(?cmd);
    ensure!(success, "Failed to run cmd.",);

    Ok(())
}
