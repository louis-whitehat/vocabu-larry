use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::models::UserEntry;
use crate::Route;

#[function_component(LoginView)]
pub fn login_view() -> Html {
    let users = use_state(Vec::<UserEntry>::new);
    let selected_user = use_state(|| None::<String>);
    let selected_dictionary = use_state(|| None::<String>);
    let last_logged_user = use_state(|| None::<String>);
    let error_message = use_state(|| None::<String>);
    let navigator = use_navigator();

    {
        let users = users.clone();
        let error_message = error_message.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                match api::fetch_users().await {
                    Ok(response) => users.set(response),
                    Err(error) => {
                        error_message.set(Some(format!("Failed to fetch users: {error}")))
                    }
                }
            });

            || ()
        });
    }

    let dictionaries = {
        let current_user = (*selected_user).clone();
        users
            .iter()
            .find(|entry| Some(entry.name.clone()) == current_user)
            .map(|entry| entry.dictionaries.clone())
            .unwrap_or_default()
    };

    let on_user_change = {
        let selected_user = selected_user.clone();
        let selected_dictionary = selected_dictionary.clone();
        let last_logged_user = last_logged_user.clone();
        let error_message = error_message.clone();

        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = input.value();
            let next_user = if value.is_empty() { None } else { Some(value) };

            selected_dictionary.set(None);
            selected_user.set(next_user.clone());

            let should_log = next_user.is_some() && next_user != *last_logged_user;
            if !should_log {
                return;
            }

            let last_logged_user = last_logged_user.clone();
            let error_message = error_message.clone();
            spawn_local(async move {
                let user = next_user.expect("user should exist when logging");
                match api::post_login(user.clone()).await {
                    Ok(()) => last_logged_user.set(Some(user)),
                    Err(error) => {
                        error_message.set(Some(format!("Failed to log login event: {error}")))
                    }
                }
            });
        })
    };

    let on_dictionary_change = {
        let selected_dictionary = selected_dictionary.clone();
        let selected_user = selected_user.clone();
        let navigator = navigator.clone();

        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = input.value();
            let next_dictionary = if value.is_empty() { None } else { Some(value) };

            selected_dictionary.set(next_dictionary.clone());

            if let (Some(user), Some(dictionary), Some(navigator)) =
                ((*selected_user).clone(), next_dictionary, navigator.clone())
            {
                navigator.push(&Route::Exam { user, dictionary });
            }
        })
    };

    html! {
        <div class="login-page">
            <section class="panel-card">
                <h1 class="page-title">{"Welcome back"}</h1>
                <p class="hero-copy">{"Pick a learner, choose a dictionary, and jump straight into the next round."}</p>

                if let Some(error_message) = &*error_message {
                    <p class="error-message">{error_message.clone()}</p>
                }

                <div class="form-grid">
                    <label class="field-label" for="user-select">{"Who are you?"}</label>
                    <select id="user-select" onchange={on_user_change}>
                        <option value="">{"Select a learner"}</option>
                        {for users.iter().map(|item| {
                            html! {
                                <option value={item.name.clone()} selected={Some(item.name.clone()) == *selected_user}>
                                    {item.name.clone()}
                                </option>
                            }
                        })}
                    </select>

                    if selected_user.is_some() {
                        <>
                            <label class="field-label" for="dictionary-select">{"Choose a dictionary"}</label>
                            <select id="dictionary-select" onchange={on_dictionary_change}>
                                <option value="">{"Select a dictionary"}</option>
                                {for dictionaries.into_iter().map(|item| {
                                    let is_selected = Some(item.clone()) == *selected_dictionary;
                                    html! {
                                        <option value={item.clone()} selected={is_selected}>{item}</option>
                                    }
                                })}
                            </select>
                        </>
                    }
                </div>

                if let Some(user) = &*selected_user {
                    <div class="actions-row">
                        <Link<Route> to={Route::Score { user: user.clone() }} classes="secondary-action">
                            {"Show score"}
                        </Link<Route>>
                    </div>
                }
            </section>
        </div>
    }
}
