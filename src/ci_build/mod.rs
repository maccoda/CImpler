//! Module that works with the build process. This is the section related to
//! anything involving the build configuration or build process and execution
//! within the shell.
use std::path::Path;
use std::io;

use serde_yaml;

use file_utils;

use super::{ErrorKind, Result};

mod git_cmd;
mod cmd_exec;


#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfiguration {
    before_build: Option<Vec<CommandString>>,
    build: Option<Vec<CommandString>>,
    after_build: Option<Vec<CommandString>>,
}

impl BuildConfiguration {
    fn from<P: AsRef<Path>>(
        file_name: P,
    ) -> ::std::result::Result<BuildConfiguration, serde_yaml::Error> {
        serde_yaml::from_str(&file_utils::read_from_file(file_name))
        // let raw = RawConfiguration::from(file_name)?;
        // Ok(BuildConfiguration(raw))
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct CommandString(String);

impl From<String> for CommandString {
    fn from(f: String) -> CommandString {
        CommandString(f)
    }
}

impl<'a> From<&'a str> for CommandString {
    fn from(f: &str) -> CommandString {
        CommandString(f.to_owned())
    }
}

/// Parse a YAML file to produce the `BuildConfig`. Will return error if the
/// config does not match the expected format
pub fn parse_build_file<P: AsRef<Path>>(
    file_name: P,
) -> ::std::result::Result<BuildConfiguration, serde_yaml::Error> {
    BuildConfiguration::from(file_name)
}

pub fn exec_before_build(config: BuildConfiguration) -> Result<()> {
    exec_cmd_string(config.before_build)
}

fn exec_cmd_string(cmd_string: Option<Vec<CommandString>>) -> Result<()> {
    if let Some(cmds) = cmd_string {
        for cmd in cmds {
            cmd_exec::execute(cmd)?;
        }
        Ok(())
    } else {
        Err(ErrorKind::CmdFail("Command not present".into()).into())
    }
    // TODO Should have a different error type here io doesn't make sense
}




#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_build_file() {
        use test_utils;
        let config = super::parse_build_file("tests/resources/test_config.yml").unwrap();
        test_utils::compare_vec(vec!["ls".into()], config.before_build.unwrap());
        test_utils::compare_vec(
            vec![
                "echo \"Hello there\" > test.txt".into(),
                "cat test.txt".into(),
            ],
            config.build.unwrap(),
        );
        test_utils::compare_vec(vec!["cat test.txt".into()], config.after_build.unwrap());
    }
}
