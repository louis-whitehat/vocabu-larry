use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{get_json, post_json};
use crate::views::login_viewmodel::{login_path, login_request, users_path, LoginViewModel};
use crate::Route;

#[function_component(LoginView)]
pub fn login_view() -> Html {
    let view_model = use_state(LoginViewModel::loading);
    let navigator = use_navigator();
    let user_select_ref = use_node_ref();
    let dictionary_select_ref = use_node_ref();
    let dictionaries = view_model.dictionaries();

    {
        let view_model = view_model.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let next_view_model = LoginViewModel::load_with(|| async {
                    get_json(users_path())
                        .await
                        .map_err(|error| format!("Failed to fetch users: {error}"))
                })
                .await;
                view_model.set(next_view_model);
            });

            || ()
        });
    }

    {
        let user_select_ref = user_select_ref.clone();
        let selected_user = view_model.selected_user().map(str::to_owned);
        let user_count = view_model.users().len();

        use_effect_with((selected_user, user_count), move |(selected_user, _)| {
            if selected_user.is_none() {
                if let Some(select) = user_select_ref.cast::<web_sys::HtmlSelectElement>() {
                    select.set_value("");
                    select.set_selected_index(0);
                }
            }

            || ()
        });
    }

    {
        let dictionary_select_ref = dictionary_select_ref.clone();
        let selected_dictionary = view_model.selected_dictionary().map(str::to_owned);
        let dictionary_count = dictionaries.len();

        use_effect_with(
            (selected_dictionary, dictionary_count),
            move |(selected_dictionary, _)| {
                if selected_dictionary.is_none() {
                    if let Some(select) = dictionary_select_ref.cast::<web_sys::HtmlSelectElement>()
                    {
                        select.set_value("");
                        select.set_selected_index(0);
                    }
                }

                || ()
            },
        );
    }

    let on_user_change = {
        let view_model = view_model.clone();

        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = input.value();
            let next_user = if value.is_empty() { None } else { Some(value) };

            let mut next_view_model = (*view_model).clone();
            let user_to_log = next_view_model.select_user(next_user);
            view_model.set(next_view_model.clone());

            let Some(user) = user_to_log else {
                return;
            };

            let view_model = view_model.clone();
            spawn_local(async move {
                let mut next_view_model = next_view_model;
                match post_json(login_path(), &login_request(user.clone())).await {
                    Ok(()) => next_view_model.mark_user_logged(user),
                    Err(error) => next_view_model
                        .set_error_message(format!("Failed to log login event: {error}")),
                }
                view_model.set(next_view_model);
            });
        })
    };

    let on_dictionary_change = {
        let view_model = view_model.clone();
        let navigator = navigator.clone();

        Callback::from(move |event: Event| {
            let input = event.target_unchecked_into::<web_sys::HtmlSelectElement>();
            let value = input.value();
            let next_dictionary = if value.is_empty() { None } else { Some(value) };

            let mut next_view_model = (*view_model).clone();
            next_view_model.select_dictionary(next_dictionary.clone());
            let selected_user = next_view_model.selected_user().map(str::to_owned);
            view_model.set(next_view_model);

            if let (Some(user), Some(dictionary), Some(navigator)) =
                (selected_user, next_dictionary, navigator.clone())
            {
                navigator.push(&Route::Exam { user, dictionary });
            }
        })
    };

    html! {
        <div class="login-page" id="login-page">
            <section class="panel-card" id="login-card">
                <h1 class="page-title">{"Welcome back"}</h1>
                <p class="hero-copy">{"Pick a learner, choose a dictionary, and jump straight into the next round."}</p>

                if let Some(error_message) = view_model.error_message() {
                    <p class="error-message">{error_message.to_owned()}</p>
                }

                if view_model.is_loading() {
                    <p class="muted-note">{"Loading..."}</p>
                }

                <div class="form-grid">
                    <label class="field-label" for="user-select">{"Who are you?"}</label>
                    <select
                        id="user-select"
                        ref={user_select_ref}
                        value={view_model.selected_user().unwrap_or_default().to_owned()}
                        onchange={on_user_change}
                    >
                        <option value="">{"Select a learner"}</option>
                        {for view_model.users().iter().map(|item| {
                            html! {
                                <option value={item.name.clone()}>
                                    {item.name.clone()}
                                </option>
                            }
                        })}
                    </select>

                    if view_model.selected_user().is_some() {
                        <>
                            <label class="field-label" for="dictionary-select">{"Choose a dictionary"}</label>
                            <select
                                id="dictionary-select"
                                ref={dictionary_select_ref}
                                value={view_model.selected_dictionary().unwrap_or_default().to_owned()}
                                onchange={on_dictionary_change}
                            >
                                <option value="">{"Select a dictionary"}</option>
                                {for dictionaries.into_iter().map(|item| {
                                    html! {
                                        <option value={item.clone()}>{item}</option>
                                    }
                                })}
                            </select>
                        </>
                    }
                </div>

                if let Some(user) = view_model.selected_user() {
                    <div class="actions-row">
                        <Link<Route> to={Route::Score { user: user.to_owned() }} classes="secondary-action">
                            {"Show score"}
                        </Link<Route>>
                    </div>
                }
            </section>
        </div>
    }
}
