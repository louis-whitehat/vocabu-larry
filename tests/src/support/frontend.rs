use std::process::Command;

use anyhow::{anyhow, Result};

pub async fn build_frontend() -> Result<()> {
    let repo_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or_else(|| anyhow!("tests crate should live under the repository root"))?;
    let webui_dir = repo_root.join("src").join("WebUI");

    let status = Command::new("trunk")
        .arg("build")
        .arg("--release")
        .current_dir(&webui_dir)
        .status()?;

    if !status.success() {
        return Err(anyhow!("frontend build failed"));
    }

    Ok(())
}