use crate::Result;
use std::process::Command;

pub fn make_args(args: &[&str]) -> Vec<String> {
    args.iter().copied().map(|arg| arg.to_owned()).collect()
}

pub fn execute(bin: &str, args: &[String], vars: Vec<(&str, &str)>) -> Result<()> {
    let mut cmd = Command::new(bin);
    cmd.envs(vars).args(args);
    execute_cmd(cmd)
}

pub fn execute_cmd(mut cmd: Command) -> Result<()> {
    info!(?cmd);
    let status = cmd.status()?;
    ensure!(status.success(), "Failed to run cmd");
    Ok(())
}
