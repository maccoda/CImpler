/// This module will handle all of the git related calls, particularly the merging
/// prior to actually running the test. Also this will have any other git related
/// actions that may turn up.
use std::process::Command;

// TODO Sort out how can get rustfmt to organize my imports

// TODO complete this when things are better
pub fn git_merge(from_branch: String) {
    let output = Command::new("git").arg("merge").arg(from_branch).output();
    println!("{:?}", output);
}

#[test]
fn test_git_merge() {
    git_merge(String::from("blahblah"));
    assert!(false);
}