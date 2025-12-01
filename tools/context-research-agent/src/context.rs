use anyhow::Result;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct Dependency {
    pub name: String,
    pub version: String,
    pub ecosystem: Ecosystem,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ecosystem {
    Rust,
    Node,
    Python,
}

#[derive(Deserialize)]
struct CargoToml {
    dependencies: Option<HashMap<String, toml::Value>>,
}

#[derive(Deserialize)]
struct PackageJson {
    dependencies: Option<HashMap<String, String>>,
    #[serde(rename = "devDependencies")]
    dev_dependencies: Option<HashMap<String, String>>,
}

pub async fn analyze_workspace(root: &Path) -> Result<Vec<Dependency>> {
    let mut deps = Vec::new();

    // Check Cargo.toml
    let cargo_path = root.join("Cargo.toml");
    if cargo_path.exists() {
        let content = fs::read_to_string(cargo_path).await?;
        let cargo: CargoToml = toml::from_str(&content)?;
        if let Some(d) = cargo.dependencies {
            for (name, val) in d {
                let version = match val {
                    toml::Value::String(s) => s,
                    toml::Value::Table(t) => t.get("version")
                        .and_then(|v| v.as_str())
                        .unwrap_or("*")
                        .to_string(),
                    _ => "*".to_string(),
                };
                deps.push(Dependency {
                    name,
                    version,
                    ecosystem: Ecosystem::Rust,
                });
            }
        }
    }

    // Check package.json
    let package_path = root.join("package.json");
    if package_path.exists() {
        let content = fs::read_to_string(package_path).await?;
        let pkg: PackageJson = serde_json::from_str(&content)?;

        if let Some(d) = pkg.dependencies {
            for (name, version) in d {
                deps.push(Dependency { name, version, ecosystem: Ecosystem::Node });
            }
        }
        if let Some(d) = pkg.dev_dependencies {
            for (name, version) in d {
                deps.push(Dependency { name, version, ecosystem: Ecosystem::Node });
            }
        }
    }

    // Filter out local paths or workspace references if needed
    // For now, we keep everything that looks like a version

    Ok(deps)
}
