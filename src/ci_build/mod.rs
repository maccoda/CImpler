//! Module that works with the build process. This is the section related to
//! anything involving the build configuration or build process and execution
//! within the shell.
use std::path::Path;

use serde_yaml;

use file_utils;

use super::{ErrorKind, Result};
use self::git::{GitCommit, GitUrl};

pub mod git;
mod cmd_exec;

/// Structure of the build file that will be maintained in the users repository
#[derive(Serialize, Deserialize, Debug)]
pub struct UserBuildConfiguration {
    before_build: Option<Vec<CommandString>>,
    build: Option<Vec<CommandString>>,
    after_build: Option<Vec<CommandString>>,
}

impl UserBuildConfiguration {
    /// Construct the build file type from the specified path
    fn from<P: AsRef<Path>>(
        file_name: P,
    ) -> ::std::result::Result<UserBuildConfiguration, serde_yaml::Error> {
        serde_yaml::from_str(&file_utils::read_from_file(file_name))
    }
}
/// Build file composed of the `UserBuildConfiguration` along with some
/// metadata about the build and repository
#[derive(Serialize, Deserialize, Debug)]
pub struct BuildConfiguration {
    // Added from database
    repo_url: GitUrl,
    commit: GitCommit,
    user_build: UserBuildConfiguration,
}

impl BuildConfiguration {
    /// Construct the build file type from the specified path
    fn from<P: AsRef<Path>>(
        file_name: P,
    ) -> ::std::result::Result<BuildConfiguration, serde_yaml::Error> {
        serde_yaml::from_str(&file_utils::read_from_file(file_name))
    }
}

/// Typing of a spaced command string to be provided to a shell.
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
pub fn parse_user_build_file<P: AsRef<Path>>(
    file_name: P,
) -> ::std::result::Result<UserBuildConfiguration, serde_yaml::Error> {
    UserBuildConfiguration::from(file_name)
}

/// Struct that handles the execution of the `BuildConfiguration`
#[derive(Debug)]
pub struct CiBuilder {
    configuration: BuildConfiguration,
}

impl CiBuilder {
    /// Constructs a new builder from the provided configuration
    pub fn new(configuration: BuildConfiguration) -> CiBuilder {
        CiBuilder { configuration }
    }

    /// Execute the build file
    pub fn exec_build(self) -> Result<()> {
        // Start with getting the repository
        git::clone_and_checkout(self.configuration.repo_url, &self.configuration.commit)?;
        exec_cmd_string(self.configuration.user_build.before_build)
            .and(exec_cmd_string(self.configuration.user_build.build))
            .and(exec_cmd_string(self.configuration.user_build.after_build))
    }
}

fn exec_cmd_string(full_cmd: Option<Vec<CommandString>>) -> Result<()> {
    if let Some(cmds) = full_cmd {
        for cmd in cmds {
            let exit_status = cmd_exec::execute(&cmd)?;
            if !exit_status.success() {
                return Err(
                    ErrorKind::CmdFail(format!(
                        "Failed with exit code {}",
                        exit_status.code().unwrap_or(-1)
                    )).into(),
                );
            }
        }
        return Ok(());
    }
    // Assume that no field is mandatory
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_build_file() {
        use test_utils;
        let config = super::parse_user_build_file("tests/resources/test_user_config.yml").unwrap();
        test_utils::compare_vec(vec!["ls".into()], config.before_build.unwrap());
        test_utils::compare_vec(
            vec!["touch test.txt".into(), "cat test.txt".into()],
            config.build.unwrap(),
        );
        test_utils::compare_vec(
            vec![
                "echo Let's look at the build file".into(),
                "cat Cargo.toml".into(),
            ],
            config.after_build.unwrap(),
        );
    }
}
