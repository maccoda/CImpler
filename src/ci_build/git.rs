//! Module contains all git related structs and functions
use git2::Repository;
use url;

/// ssh or https URL of a git repository
#[derive(Debug, Serialize, Deserialize)]
pub struct GitUrl(String);

impl GitUrl {
    fn path(&self) -> String {
        let mut full = url::Url::parse(&self.0)
            .map(|x| x.path().to_owned())
            .expect("Invalid git URL");
        full.remove(0);
        full
    }
}
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
    let cloned_dir = url.path();
    let repo = Repository::clone(&url.0, cloned_dir)?;
    repo.set_head_detached(::git2::Oid::from_str(&commit.0)?)?;
    Ok(())
}
