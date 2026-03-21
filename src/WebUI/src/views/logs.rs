use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::views::logs_viewmodel::LogsViewModel;
use crate::Route;

#[function_component(LogsView)]
pub fn logs_view() -> Html {
    let view_model = use_state(LogsViewModel::loading);

    let load_logs = {
        let view_model = view_model.clone();

        Callback::from(move |file: Option<String>| {
            let view_model = view_model.clone();

            spawn_local(async move {
                let next_view_model = LogsViewModel::load(file.as_deref(), None).await;
                view_model.set(next_view_model);
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
        <div class="page-shell" id="logs-page">
            <section class="panel-card" id="logs-card">
                <h1 class="page-title">{"Backend logs"}</h1>
                <p class="page-copy">{"Inspect daily request failures and login events without leaving the app."}</p>

                if let Some(error_message) = view_model.error_message() {
                    <div class="error-message">{error_message.to_owned()}</div>
                }

                if !view_model.is_empty() {
                    <div class="log-picker" id="log-picker">
                        <label for="log-file" class="field-label">{"Log file"}</label>
                        <select id="log-file" value={view_model.selected_file().unwrap_or_default().to_owned()} onchange={on_change}>
                            {for view_model.files().iter().map(|file| {
                                html! {
                                    <option value={file.clone()} selected={Some(file.clone()) == view_model.selected_file().map(str::to_owned)}>{file.clone()}</option>
                                }
                            })}
                        </select>
                    </div>
                }

                if !view_model.is_empty() {
                    <pre id="log-content">{view_model.content().to_owned()}</pre>
                } else if view_model.is_loading() {
                    <div class="muted-note" id="no-log-files-message">{"Loading..."}</div>
                } else {
                    <div class="muted-note" id="no-log-files-message">{"No log files found."}</div>
                }

                <div class="actions-row">
                    <Link<Route> to={Route::Login} classes="secondary-action">{"Home"}</Link<Route>>
                </div>
            </section>
        </div>
    }
}
