use std::{
    env,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::Duration,
};

use anyhow::{anyhow, bail, Context, Result};

fn main() -> Result<()> {
    let command = env::args().nth(1).unwrap_or_default();

    match command.as_str() {
        "acceptance" => run_acceptance(),
        "local" => run_local(),
        "build-frontend" => build_frontend(),
        "build-backend" => build_backend(),
        "build-acceptance" => build_acceptance(),
        _ => {
            print_usage();
            if command.is_empty() {
                Ok(())
            } else {
                Err(anyhow!("unknown xtask command: {command}"))
            }
        }
    }
}

fn run_acceptance() -> Result<()> {
    ensure_wasm_target()?;
    build_frontend()?;
    build_backend()?;
    build_acceptance()?;

    let mut command = Command::new(acceptance_binary_path());
    command.env("ACCEPTANCE_BACKEND_BIN", backend_binary_path());
    command.current_dir(workspace_root());
    run_command(command, "run acceptance suite")
}

fn run_local() -> Result<()> {
    ensure_wasm_target()?;
    build_frontend()?;
    build_backend()?;

    let backend_url = "http://127.0.0.1:8101";
    let root = workspace_root();
    let mut child = Command::new(backend_binary_path())
        .current_dir(root.join("src").join("WebApi"))
        .env("NODE_ENV", "production")
        .env("VOCABULARRY_HOME", &root)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .with_context(|| {
            format!(
                "failed to start backend at {}",
                backend_binary_path().display()
            )
        })?;

    thread::sleep(Duration::from_secs(3));
    if let Err(error) = open_in_browser(backend_url) {
        eprintln!("failed to open browser automatically: {error}");
        eprintln!("open {backend_url} manually");
    }

    wait_for_child(&mut child, "local backend")
}

fn build_frontend() -> Result<()> {
    let mut command = Command::new("trunk");
    command
        .arg("build")
        .arg("--release")
        .current_dir(workspace_root().join("src").join("WebUI"));
    run_command(command, "build frontend")
}

fn build_backend() -> Result<()> {
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--manifest-path")
        .arg(workspace_root().join("src").join("WebApi").join("Cargo.toml"))
        .current_dir(workspace_root());
    run_command(command, "build backend")
}

fn build_acceptance() -> Result<()> {
    let mut command = Command::new("cargo");
    command
        .arg("build")
        .arg("--manifest-path")
        .arg(workspace_root().join("tests").join("Cargo.toml"))
        .current_dir(workspace_root());
    run_command(command, "build acceptance runner")
}

fn ensure_wasm_target() -> Result<()> {
    let rustup_available = Command::new("rustup")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();

    match rustup_available {
        Ok(status) if status.success() => {
            let mut command = Command::new("rustup");
            command
                .arg("target")
                .arg("add")
                .arg("wasm32-unknown-unknown");
            run_command(command, "ensure wasm target")
        }
        Ok(_) | Err(_) => Ok(()),
    }
}

fn run_command(mut command: Command, description: &str) -> Result<()> {
    let status = command
        .status()
        .with_context(|| format!("failed to {description}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!(
            "failed to {description}: process exited with {status}"
        ))
    }
}

fn wait_for_child(child: &mut Child, description: &str) -> Result<()> {
    let status = child
        .wait()
        .with_context(|| format!("failed while waiting for {description}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("{description} exited with {status}"))
    }
}

fn open_in_browser(url: &str) -> Result<()> {
    let status = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(["/C", "start", "", url])
            .status()
            .context("failed to invoke Windows browser launcher")?
    } else if cfg!(target_os = "macos") {
        Command::new("open")
            .arg(url)
            .status()
            .context("failed to invoke macOS browser launcher")?
    } else {
        Command::new("xdg-open")
            .arg(url)
            .status()
            .context("failed to invoke Linux browser launcher")?
    };

    if status.success() {
        Ok(())
    } else {
        bail!("browser launcher exited with {status}")
    }
}

fn workspace_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask crate should live under the workspace root")
        .to_path_buf()
}

fn backend_binary_path() -> PathBuf {
    workspace_root()
        .join("target")
        .join("debug")
        .join(executable_name("vocabu-larry-api"))
}

fn acceptance_binary_path() -> PathBuf {
    workspace_root()
        .join("target")
        .join("debug")
        .join(executable_name("vocabu-larry-acceptance"))
}

fn executable_name(base_name: &str) -> String {
    if cfg!(windows) {
        format!("{base_name}.exe")
    } else {
        base_name.to_owned()
    }
}

fn print_usage() {
    eprintln!("Usage: cargo xtask <command>");
    eprintln!("Commands:");
    eprintln!("  acceptance       Build artifacts and run the acceptance suite");
    eprintln!("  local            Build artifacts, start the backend, and open the app");
    eprintln!("  build-frontend   Build the frontend with trunk");
    eprintln!("  build-backend    Build the backend binary");
    eprintln!("  build-acceptance Build the acceptance runner");
}
