use serde::Deserialize;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{encode_query_value, get_json};
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogResponse {
    pub files: Vec<String>,
    pub selected_file: Option<String>,
    pub content: String,
}

pub async fn fetch_logs(file: Option<&str>) -> Result<LogResponse, String> {
    match file {
        Some(file) => get_json(&format!("/api/logs?file={}", encode_query_value(file))).await,
        None => get_json("/api/logs").await,
    }
}

#[function_component(LogsView)]
pub fn logs_view() -> Html {
    let files = use_state(Vec::<String>::new);
    let selected_file = use_state(|| None::<String>);
    let content = use_state(String::new);
    let error_message = use_state(|| None::<String>);

    let load_logs = {
        let files = files.clone();
        let selected_file = selected_file.clone();
        let content = content.clone();
        let error_message = error_message.clone();

        Callback::from(move |file: Option<String>| {
            let files = files.clone();
            let selected_file = selected_file.clone();
            let content = content.clone();
            let error_message = error_message.clone();

            spawn_local(async move {
                match fetch_logs(file.as_deref()).await {
                    Ok(LogResponse {
                        files: next_files,
                        selected_file: next_selected_file,
                        content: next_content,
                    }) => {
                        files.set(next_files);
                        selected_file.set(next_selected_file);
                        content.set(next_content);
                        error_message.set(None);
                    }
                    Err(error) => error_message.set(Some(format!("Failed to fetch logs: {error}"))),
                }
            });
        })
    };

    {
        let load_logs = load_logs.clone();

        use_effect_with((), move |_| {
            load_logs.emit(None);
            || ()
        });
    }

    let on_change = {
        let load_logs = load_logs.clone();

        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = input.value();
            load_logs.emit(Some(value));
        })
    };

    html! {
        <div class="page-shell">
            <section class="panel-card">
                <h1 class="page-title">{"Backend logs"}</h1>
                <p class="page-copy">{"Inspect daily request failures and login events without leaving the app."}</p>

                if let Some(error_message) = &*error_message {
                    <div class="error-message">{error_message.clone()}</div>
                }

                if !files.is_empty() {
                    <div class="log-picker">
                        <label for="log-file" class="field-label">{"Log file"}</label>
                        <select id="log-file" value={selected_file.as_deref().map(str::to_owned).unwrap_or_default()} onchange={on_change}>
                            {for files.iter().map(|file| {
                                html! {
                                    <option value={file.clone()} selected={Some(file.clone()) == *selected_file}>{file.clone()}</option>
                                }
                            })}
                        </select>
                    </div>
                }

                if !files.is_empty() {
                    <pre>{(*content).clone()}</pre>
                } else {
                    <div class="muted-note">{"No log files found."}</div>
                }

                <div class="actions-row">
                    <Link<Route> to={Route::Login} classes="secondary-action">{"Home"}</Link<Route>>
                </div>
            </section>
        </div>
    }
}