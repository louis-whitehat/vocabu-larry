use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{encode_query_value, get_json, post_json};
use crate::Route;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct ScoreRequest {
    user: String,
    dictionary: String,
    is_correct: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize)]
pub struct ScoreEntry {
    pub total: u64,
    pub correct: u64,
}

pub type ScoreStore = BTreeMap<String, BTreeMap<String, ScoreEntry>>;

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
    let scores = use_state(|| None::<ScoreStore>);
    let error_message = use_state(|| None::<String>);

    {
        let scores = scores.clone();
        let error_message = error_message.clone();
        let user = props.user.clone();

        use_effect_with(props.user.clone(), move |_| {
            spawn_local(async move {
                match fetch_scores(&user).await {
                    Ok(response) => {
                        scores.set(Some(response));
                        error_message.set(None);
                    }
                    Err(error) => error_message.set(Some(format!("Failed to fetch scores: {error}"))),
                }
            });

            || ()
        });
    }

    let sorted_dates = scores
        .as_ref()
        .as_ref()
        .map(|score_store| {
            let mut dates = score_store.keys().cloned().collect::<Vec<_>>();
            dates.sort_by(|left, right| right.cmp(left));
            dates
        })
        .unwrap_or_default();

    html! {
        <div class="page-shell" id="score-page">
            <section class="panel-card" id="score-card">
                <h1 class="page-title">{"Score"}</h1>
                <p class="page-copy">{"Daily totals for the selected learner across all dictionaries."}</p>

                if let Some(error_message) = &*error_message {
                    <p class="error-message">{error_message.clone()}</p>
                }

                if let Some(score_store) = scores.as_ref().as_ref() {
                    if score_store.is_empty() {
                        <p class="muted-note">{"No score entries yet."}</p>
                    } else {
                        <table class="score-table" id="score-table">
                            <tr>
                                <th class="label">{"Date"}</th>
                                <th class="label">{"Dictionary"}</th>
                                <th class="value">{"Correct"}</th>
                                <th class="value">{"Total"}</th>
                                <th class="value">{"Pass Rate"}</th>
                            </tr>
                            {for sorted_dates.iter().flat_map(|date| {
                                score_store[date].iter().map(move |(dictionary, stats)| {
                                    let pass_rate = if stats.total == 0 {
                                        "0.00".to_owned()
                                    } else {
                                        format!("{:.2}", stats.correct as f64 / stats.total as f64 * 100.0)
                                    };

                                    html! {
                                        <tr key={format!("{date}:{dictionary}")}>
                                            <td class="label">{date.clone()}</td>
                                            <td class="label">{dictionary.clone()}</td>
                                            <td class="value">{stats.correct}</td>
                                            <td class="value">{stats.total}</td>
                                            <td class="value">{pass_rate}</td>
                                        </tr>
                                    }
                                }).collect::<Vec<_>>()
                            })}
                        </table>
                    }
                } else if error_message.is_none() {
                    <p class="muted-note">{"Loading..."}</p>
                }

                <div class="actions-row">
                    <Link<Route> to={Route::Login} classes="secondary-action">{"Home"}</Link<Route>>
                </div>
            </section>
        </div>
    }
}