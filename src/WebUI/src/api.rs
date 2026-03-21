use serde::de::DeserializeOwned;
use serde::Serialize;

pub(crate) fn resolve_browser_api_base() -> String {
    let Some(window) = web_sys::window() else {
        return "http://localhost:8101".to_owned();
    };

    let location = window.location();
    let hostname = location.hostname().unwrap_or_default();
    let port = location.port().unwrap_or_default();
    let host = location.host().unwrap_or_default();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".to_owned());
    let current_origin = format!("{protocol}//{host}");

    if matches!(port.as_str(), "8080" | "8081" | "1420" | "3000")
        && (hostname == "localhost" || hostname == "127.0.0.1")
    {
        "http://localhost:8101".to_owned()
    } else {
        current_origin
    }
}

pub(crate) async fn get_json<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    let response = reqwest::get(format!("{}{}", resolve_browser_api_base(), path))
        .await
        .map_err(|error| error.to_string())?;
    let status = response.status();
    let body = response.text().await.map_err(|error| error.to_string())?;

    if !status.is_success() {
        return Err(body);
    }

    serde_json::from_str::<T>(&body).map_err(|error| error.to_string())
}

pub(crate) async fn post_json<B>(path: &str, body: &B) -> Result<(), String>
where
    B: Serialize,
{
    let response = reqwest::Client::new()
        .post(format!("{}{}", resolve_browser_api_base(), path))
        .json(body)
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if response.status().is_success() {
        return Ok(());
    }

    let body = response.text().await.map_err(|error| error.to_string())?;
    Err(body)
}
