//! Version command - Manage protocol version

use anyhow::Result;
use console::style;
use std::path::Path;

use crate::config::{Config, VERSION_FILE};
use crate::utils::{print_header, print_info, print_success};

/// Run the version command
pub async fn run(bump: Option<String>, set: Option<String>) -> Result<()> {
    let config = Config::load()?;

    match (bump, set) {
        (Some(bump_type), _) => {
            bump_version(&config.version, &bump_type)?;
        }
        (_, Some(new_version)) => {
            set_version(&new_version)?;
        }
        (None, None) => {
            show_version(&config)?;
        }
    }

    Ok(())
}

fn show_version(config: &Config) -> Result<()> {
    print_header("📊 Version Information");

    println!();
    println!("  {} {}",
        style("Protocol Version:").dim(),
        style(&config.version).green().bold()
    );

    // Check for latest
    println!();
    print_info("Run 'git-core upgrade' to check for updates");

    Ok(())
}

fn bump_version(current: &str, bump_type: &str) -> Result<()> {
    use crate::config::ProtocolVersion;

    let mut version = ProtocolVersion::parse(current)?;

    match bump_type.to_lowercase().as_str() {
        "major" => {
            version.major += 1;
            version.minor = 0;
            version.patch = 0;
        }
        "minor" => {
            version.minor += 1;
            version.patch = 0;
        }
        "patch" => {
            version.patch += 1;
        }
        _ => {
            anyhow::bail!("Invalid bump type: {}. Use major, minor, or patch", bump_type);
        }
    }

    let new_version = version.to_string();
    set_version(&new_version)?;

    println!();
    println!("  {} → {}",
        style(current).yellow(),
        style(&new_version).green().bold()
    );

    Ok(())
}

fn set_version(version: &str) -> Result<()> {
    // Validate version format
    crate::config::ProtocolVersion::parse(version)?;

    std::fs::write(VERSION_FILE, version)?;
    print_success(&format!("Version set to {}", version));

    Ok(())
}
