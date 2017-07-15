//! Integration tests of the ci_build. It will essentially just build itself.
extern crate ci_build;
mod file_utils;

use std::fs;

#[test]
fn integration_ci_build() {
    const DIR: &str = "cimpler";
    // Path needs to be relative from workspace member root
    let path = fs::canonicalize("tests/resources/test_config.yml").unwrap();
    ci_build::perform_build(path).unwrap();

    file_utils::check_dir_exists(DIR);

    // TODO Need to check the build does what it says

    // Clean up the cloned repository
    fs::remove_dir_all(DIR).unwrap();
}
