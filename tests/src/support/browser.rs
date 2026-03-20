use std::{path::PathBuf, process::Stdio, time::Duration};

use anyhow::{anyhow, Context, Result};
use fantoccini::{Client, ClientBuilder};
use portpicker::pick_unused_port;
use serde_json::{json, Map, Value};
use tokio::{
    process::{Child, Command},
    time::sleep,
};

pub struct BrowserSession {
    client: Client,
    driver_process: Option<Child>,
}

impl BrowserSession {
    pub async fn start() -> Result<Self> {
        if let Ok(webdriver_url) = std::env::var("WEBDRIVER_URL") {
            let client = connect_client(&webdriver_url, None).await?;
            return Ok(Self {
                client,
                driver_process: None,
            });
        }

        let driver_binary =
            std::env::var("CHROMEDRIVER_BIN").unwrap_or_else(|_| "chromedriver".to_owned());
        let driver_port =
            pick_unused_port().ok_or_else(|| anyhow!("failed to allocate a WebDriver port"))?;
        let mut driver_process = Command::new(PathBuf::from(driver_binary))
            .arg("--port")
            .arg(driver_port.to_string())
            .arg("--allowed-origins=*")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .with_context(|| {
                "failed to start chromedriver; install it and ensure it is in PATH, set CHROMEDRIVER_BIN, or start chromedriver yourself and set WEBDRIVER_URL"
            })?;

        let webdriver_url = format!("http://127.0.0.1:{driver_port}");
        let client = connect_client(&webdriver_url, Some(&mut driver_process)).await?;

        Ok(Self {
            client,
            driver_process: Some(driver_process),
        })
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

fn detect_chrome_binary() -> Option<String> {
    if let Ok(binary) = std::env::var("CHROME_BIN") {
        if !binary.trim().is_empty() {
            return Some(binary);
        }
    }

    if cfg!(target_os = "windows") {
        for candidate in [
            r"C:\Program Files\Google\Chrome\Application\chrome.exe",
            r"C:\Program Files (x86)\Google\Chrome\Application\chrome.exe",
        ] {
            if std::path::Path::new(candidate).exists() {
                return Some(candidate.to_owned());
            }
        }

        if let Ok(local_app_data) = std::env::var("LOCALAPPDATA") {
            let candidate = PathBuf::from(local_app_data)
                .join("Google")
                .join("Chrome")
                .join("Application")
                .join("chrome.exe");
            if candidate.exists() {
                return Some(candidate.to_string_lossy().into_owned());
            }
        }
    }

    None
}

impl Drop for BrowserSession {
    fn drop(&mut self) {
        if let Some(driver_process) = &mut self.driver_process {
            let _ = driver_process.start_kill();
        }
    }
}

async fn connect_client(
    webdriver_url: &str,
    mut driver_process: Option<&mut Child>,
) -> Result<Client> {
    let chrome_binary = detect_chrome_binary();
    let mut capabilities = Map::<String, Value>::new();
    let mut chrome_args = vec![
        "--headless".to_owned(),
        "--disable-gpu".to_owned(),
        "--window-size=1440,960".to_owned(),
    ];

    if cfg!(target_os = "linux") {
        chrome_args.push("--no-sandbox".to_owned());
    }

    let chrome_options = match chrome_binary {
        Some(binary) => json!({ "args": chrome_args, "binary": binary }),
        None => json!({ "args": chrome_args }),
    };
    capabilities.insert("goog:chromeOptions".to_owned(), chrome_options);

    let mut last_error = None;
    for _ in 0..50 {
        match ClientBuilder::native()
            .capabilities(capabilities.clone())
            .connect(webdriver_url)
            .await
        {
            Ok(client) => return Ok(client),
            Err(error) => {
                last_error = Some(error.to_string());
                sleep(Duration::from_millis(200)).await;
                if let Some(process) = driver_process.as_deref_mut() {
                    if process.try_wait()?.is_some() {
                        return Err(anyhow!(
                            "chromedriver exited before a browser session could be created{}",
                            last_error
                                .as_deref()
                                .map(|message| format!(": {message}"))
                                .unwrap_or_default()
                        ));
                    }
                }
            }
        }
    }

    Err(anyhow!(
        "failed to create a Chrome WebDriver session{}",
        last_error
            .as_deref()
            .map(|message| format!(": {message}"))
            .unwrap_or_default()
    ))
}
