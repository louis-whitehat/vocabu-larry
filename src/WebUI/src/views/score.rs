use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{encode_query_value, get_json, post_json};
use crate::views::score_viewmodel::{ScoreStore, ScoreViewModel};
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ScoreRequest {
    user: String,
    dictionary: String,
    is_correct: bool,
}

pub async fn post_score(user: String, dictionary: String, is_correct: bool) -> Result<(), String> {
    post_json(
        "/api/score",
        &ScoreRequest {
            user,
            dictionary,
            is_correct,
        },
    )
    .await
}

pub async fn fetch_scores(user: &str) -> Result<ScoreStore, String> {
    get_json(&format!("/api/score?user={}", encode_query_value(user))).await
}

#[derive(Properties, PartialEq)]
pub struct ScoreViewProps {
    pub user: String,
}

#[function_component(ScoreView)]
pub fn score_view(props: &ScoreViewProps) -> Html {
    let view_model = use_state(ScoreViewModel::loading);

    {
        let view_model = view_model.clone();
        let user = props.user.clone();

        use_effect_with(props.user.clone(), move |_| {
            spawn_local(async move {
                let next_view_model = ScoreViewModel::load_with(|| async move {
                    fetch_scores(&user)
                        .await
                        .map_err(|error| format!("Failed to fetch scores: {error}"))
                })
                .await;
                view_model.set(next_view_model);
            });

            || ()
        });
    }
    let rows = view_model.rows();

    html! {
        <div class="page-shell" id="score-page">
            <section class="panel-card" id="score-card">
                <h1 class="page-title">{"Score"}</h1>
                <p class="page-copy">{"Daily totals for the selected learner across all dictionaries."}</p>

                if let Some(error_message) = view_model.error_message() {
                    <p class="error-message">{error_message.to_owned()}</p>
                }

                if view_model.is_empty() {
                    <p class="muted-note">{"No score entries yet."}</p>
                } else if !rows.is_empty() {
                    <table class="score-table" id="score-table">
                        <tr>
                            <th class="label">{"Date"}</th>
                            <th class="label">{"Dictionary"}</th>
                            <th class="value">{"Correct"}</th>
                            <th class="value">{"Total"}</th>
                            <th class="value">{"Pass Rate"}</th>
                        </tr>
                        {for rows.into_iter().map(|row| {
                            html! {
                                <tr key={format!("{}:{}", row.date, row.dictionary)}>
                                    <td class="label">{row.date}</td>
                                    <td class="label">{row.dictionary}</td>
                                    <td class="value">{row.correct}</td>
                                    <td class="value">{row.total}</td>
                                    <td class="value">{row.pass_rate}</td>
                                </tr>
                            }
                        })}
                    </table>
                } else if view_model.is_loading() {
                    <p class="muted-note">{"Loading..."}</p>
                }

                <div class="actions-row">
                    <Link<Route> to={Route::Login} classes="secondary-action">{"Home"}</Link<Route>>
                </div>
            </section>
        </div>
    }
}