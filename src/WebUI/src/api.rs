use gloo_net::http::Request;
use js_sys::encode_uri_component;
use serde::de::DeserializeOwned;
use serde::Serialize;

fn api_base() -> String {
    let Some(window) = web_sys::window() else {
        return "http://localhost:8101".to_owned();
    };

    let location = window.location();
    let hostname = location.hostname().unwrap_or_default();
    let port = location.port().unwrap_or_default();
    let host = location.host().unwrap_or_default();
    let protocol = location.protocol().unwrap_or_else(|_| "http:".to_owned());

    if port == "8101" || port == "8102" {
        format!("{protocol}//{host}")
    } else if hostname == "localhost" || hostname == "127.0.0.1" {
        "http://localhost:8101".to_owned()
    } else {
        format!("{protocol}//{host}")
    }
}

pub(crate) async fn get_json<T>(path: &str) -> Result<T, String>
where
    T: DeserializeOwned,
{
    Request::get(&format!("{}{}", api_base(), path))
        .send()
        .await
        .map_err(|error| error.to_string())?
        .json::<T>()
        .await
        .map_err(|error| error.to_string())
}

pub(crate) fn encode_query_value(value: &str) -> String {
    encode_uri_component(value).into()
}

pub(crate) async fn post_json<B>(path: &str, body: &B) -> Result<(), String>
where
    B: Serialize,
{
    Request::post(&format!("{}{}", api_base(), path))
        .header("content-type", "application/json")
        .json(body)
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;

    Ok(())
}
