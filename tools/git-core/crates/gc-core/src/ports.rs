use async_trait::async_trait;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Git Error: {0}")]
    Git(String),
    #[error("GitHub Error: {0}")]
    GitHub(String),
    #[error("System Error: {0}")]
    System(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;

#[async_trait]
pub trait GitPort: Send + Sync {
    async fn init(&self) -> Result<()>;
    async fn status(&self) -> Result<bool>; // true if clean
    async fn remote_url(&self) -> Result<Option<String>>;
    async fn commit(&self, msg: &str) -> Result<()>;
    async fn push(&self) -> Result<()>;
}

#[async_trait]
pub trait GitHubPort: Send + Sync {
    async fn check_auth(&self) -> Result<String>; // returns username
    async fn create_repo(&self, name: &str, private: bool) -> Result<()>;
    async fn create_issue(&self, title: &str, body: &str, labels: &[String]) -> Result<()>;
    async fn create_label(&self, name: &str, color: &str, desc: &str) -> Result<()>;
    async fn get_file_content(&self, owner: &str, repo: &str, branch: &str, path: &str) -> Result<String>;
    async fn get_pr_diff(&self, owner: &str, repo: &str, pr_number: u64) -> Result<String>;
    async fn post_comment(&self, owner: &str, repo: &str, issue_number: u64, body: &str) -> Result<()>;
}

#[async_trait]
pub trait FileSystemPort: Send + Sync {
    async fn create_dir(&self, path: &str) -> Result<()>;
    async fn write_file(&self, path: &str, content: &str) -> Result<()>;
    async fn read_file(&self, path: &str) -> Result<String>;
    async fn exists(&self, path: &str) -> Result<bool>;
}

#[async_trait]
pub trait SystemPort: Send + Sync {
    async fn check_command(&self, name: &str) -> Result<bool>;
    async fn run_command(&self, name: &str, args: &[String]) -> Result<()>;
    async fn run_command_output(&self, name: &str, args: &[String]) -> Result<String>;
}
