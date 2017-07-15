//! Module contains all git related structs and functions
use git2::Repository;
use url;

/// ssh or https URL of a git repository
#[derive(Debug, Serialize, Deserialize)]
pub struct GitUrl(String);

impl GitUrl {
    /// Returns the file path for where the git repository should be cloned into
    fn path(&self) -> String {
        let mut last = self.0
            .split("/")
            .last()
            .expect("Invalid git URL")
            .to_owned();
        let length = last.len();
        last.split_off(length - ".git".len());
        last
    }
}

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
    let cloned_dir = url.path();
    println!("Cloning into {:?}", cloned_dir);
    let repo = Repository::clone(&url.0, cloned_dir)?;
    repo.set_head_detached(::git2::Oid::from_str(&commit.0)?)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::GitUrl;
    #[test]
    fn test_path() {
        let git = GitUrl("https://gitlab.com/maccoda/cimpler.git".to_owned());
        assert_eq!("cimpler", git.path());
        let git = GitUrl("../maccoda/cimpler.git".to_owned());
        assert_eq!("cimpler", git.path());
    }
}
