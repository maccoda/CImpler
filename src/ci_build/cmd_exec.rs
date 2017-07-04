//! Module for generic command execution from the vector element expected from
//! the build file.
use std::process::{Command, ExitStatus};

use std::io;

use ci_build::CommandString;

/// Takes a full command as a space separated string and constructs a command to
/// execte, returning the exit status code to simply determine if the command
/// was succesful.
pub fn execute(full_cmd: CommandString) -> Result<ExitStatus, io::Error> {
    let mut cmd_with_args = full_cmd.0.split_whitespace();
    let mut exec = cmd_with_args.next().ok_or(io::ErrorKind::Other).map(
        |cmd| {
            Command::new(cmd)
        },
    )?;
    for arg in cmd_with_args {
        exec.arg(arg);
    }
    let exit_code = exec.status()?;
    Ok(exit_code)
}
