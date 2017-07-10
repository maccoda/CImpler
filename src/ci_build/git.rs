//! Module contains all git related structs and functions
use git2::Repository;

/// ssh or https URL of a git repository
#[derive(Debug, Serialize, Deserialize)]
pub struct GitUrl(String);
// TODO just thinking that we should probably just use URL to ensure the URL is
// correct form

/// Representation of a git commit. This can either map to a commit hash or a
/// tag.
// #[derive(Debug, Serialize, Deserialize)]
// pub enum GitCommit {
//     Commit(CommitHash),
//     Tag(String),
// }
//
// #[derive(Debug, Serialize, Deserialize)]
// pub struct CommitHash(u64);
#[derive(Debug, Serialize, Deserialize)]
pub struct GitCommit(String);

/// Initialize the repository by cloning from the provided URL and moving the
/// head to the specified commit or tag.
pub fn clone_and_checkout(url: GitUrl, commit: &GitCommit) -> Result<(), ::git2::Error> {
    // TODO Need to determine a good way for the cloned directory name
    let cloned_dir = ".";
    let repo = Repository::clone(&url.0, cloned_dir)?;
    repo.set_head(&(commit.0))?;
    Ok(())
}
