use async_trait::async_trait;
use gc_core::ports::{SystemPort, Result, CoreError};
use tokio::process::Command;

pub struct TokioSystem;

#[async_trait]
impl SystemPort for TokioSystem {
    async fn check_command(&self, name: &str) -> Result<bool> {
        // On Windows we check with Get-Command or where.exe, on Linux which
        let output = if cfg!(target_os = "windows") {
             Command::new("where")
                .arg(name)
                .output()
                .await
                .map_err(CoreError::Io)?
        } else {
             Command::new("which")
                .arg(name)
                .output()
                .await
                .map_err(CoreError::Io)?
        };
        Ok(output.status.success())
    }

    async fn run_command(&self, name: &str, args: &[String]) -> Result<()> {
        let status = Command::new(name)
            .args(args)
            .status()
            .await
            .map_err(CoreError::Io)?;

        if status.success() {
            Ok(())
        } else {
            Err(CoreError::System(format!("Command {} failed with {:?}", name, status)))
        }
    }

    async fn run_command_output(&self, name: &str, args: &[String]) -> Result<String> {
        let output = Command::new(name)
            .args(args)
            .output()
            .await
            .map_err(CoreError::Io)?;

        if output.status.success() {
            String::from_utf8(output.stdout).map_err(|e| CoreError::System(e.to_string()))
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            Err(CoreError::System(format!("Command {} failed: {}", name, stderr)))
        }
    }
}
