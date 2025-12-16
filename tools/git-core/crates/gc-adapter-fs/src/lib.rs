use async_trait::async_trait;
use gc_core::ports::{FileSystemPort, Result, CoreError};
use std::path::Path;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub struct TokioFileSystem;

#[async_trait]
impl FileSystemPort for TokioFileSystem {
    async fn create_dir(&self, path: &str) -> Result<()> {
        if !Path::new(path).exists() {
            fs::create_dir_all(path).await.map_err(CoreError::Io)?;
        }
        Ok(())
    }

    async fn write_file(&self, path: &str, content: &str) -> Result<()> {
        let mut file = fs::File::create(path).await.map_err(CoreError::Io)?;
        file.write_all(content.as_bytes()).await.map_err(CoreError::Io)?;
        Ok(())
    }

    async fn read_file(&self, path: &str) -> Result<String> {
        let content = fs::read_to_string(path).await.map_err(CoreError::Io)?;
        Ok(content)
    }

    async fn exists(&self, path: &str) -> Result<bool> {
        Ok(Path::new(path).exists())
    }
}
