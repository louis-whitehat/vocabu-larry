use js_sys::Math;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api::{encode_query_value, get_json};
use crate::views::exam_viewmodel::{DictionaryEntry, ExamViewModel};
use crate::views::score::post_score;
use crate::Route;

pub async fn fetch_dictionary(
    user: &str,
    dictionary: &str,
) -> Result<Vec<DictionaryEntry>, String> {
    get_json(&format!(
        "/api/dictionary?user={}&dictionary={}",
        encode_query_value(user),
        encode_query_value(dictionary)
    ))
    .await
}

#[derive(Properties, PartialEq)]
pub struct ExamViewProps {
    pub user: String,
    pub dictionary: String,
}

#[function_component(ExamView)]
pub fn exam_view(props: &ExamViewProps) -> Html {
    let view_model = use_state(ExamViewModel::loading);
    let input = use_state(String::new);

    {
        let view_model = view_model.clone();
        let user = props.user.clone();
        let dictionary = props.dictionary.clone();

        use_effect_with((props.user.clone(), props.dictionary.clone()), move |_| {
            spawn_local(async move {
                let next_view_model = ExamViewModel::load_with(
                    || async {
                        fetch_dictionary(&user, &dictionary)
                            .await
                            .map_err(|error| format!("Failed to fetch dictionary: {error}"))
                    },
                    Math::random(),
                )
                .await;
                view_model.set(next_view_model);
            });

            || ()
        });
    }

    let on_input = {
        let input = input.clone();
        Callback::from(move |event: InputEvent| {
            let target = event.target_unchecked_into::<web_sys::HtmlInputElement>();
            input.set(target.value());
        })
    };

    let on_submit = {
        let view_model = view_model.clone();
        let input = input.clone();
        let user = props.user.clone();
        let dictionary = props.dictionary.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let given_answer = (*input).clone();
            input.set(String::new());

            let view_model = view_model.clone();
            let user = user.clone();
            let dictionary = dictionary.clone();
            spawn_local(async move {
                let mut next_view_model = (*view_model).clone();
                let _ = next_view_model
                    .submit_answer_with(
                        given_answer,
                        user,
                        dictionary.clone(),
                        Math::random(),
                        |user, dictionary, is_correct| async move {
                            post_score(user, dictionary, is_correct).await
                        },
                    )
                    .await;
                view_model.set(next_view_model);
            });
        })
    };

    let status_class = match view_model.answer_correct() {
        Some(true) => "feedback-panel correct",
        Some(false) => "feedback-panel wrong",
        None => "feedback-panel",
    };

    let num_words = view_model.current_translation_word_count();

    html! {
        <div class="page-shell exam-page" id="exam-page">
            <section class="panel-card exam-card" id="exam-card">
                <h1 class="page-title">{"Exam"}</h1>
                <p class="page-copy">{"Answer the translation and keep the streak moving."}</p>

                if let Some(error) = view_model.error_message() {
                    <p class="error-message load-error">{error.to_owned()}</p>
                }

                <div class="exam-question" id="exam-question">
                    <span class="muted-note">{"What is the translation of"}</span>
                    <span class="word" id="exam-word">{view_model.current_word().unwrap_or_default().to_owned()}</span>
                    <span class="hint-pill" id="exam-hint">{format!("Hint: {num_words} word(s)")}</span>
                </div>

                if view_model.is_loading() {
                    <p class="muted-note">{"Loading..."}</p>
                }

                <form onsubmit={on_submit} class="exam-form" id="exam-form">
                    <input
                        type="text"
                        value={(*input).clone()}
                        oninput={on_input}
                        class="answer-input"
                        id="answer-input"
                    />
                    <button type="submit" id="submit-answer-button">{"Submit"}</button>
                    <span class="score-chip" id="exam-score-chip">{format!("{} / {}", view_model.correct_count(), view_model.total_count())}</span>
                </form>

                <div class={status_class} id="answer-feedback">
                    if view_model.answer_correct() == Some(true) {
                        <div id="answer-correct-message">{"Correct 👍😉"}</div>
                    }
                    if view_model.answer_correct() == Some(false) {
                        <div id="answer-wrong-message">
                            {"Sorry 🙁 Your answer "}
                            <span class="word" id="your-answer">{view_model.your_answer().unwrap_or_default().to_owned()}</span>
                            {" is not correct, correct answer would have been "}
                            <span class="word" id="correct-answer">{view_model.previous_correct().unwrap_or_default().to_owned()}</span>
                        </div>
                    }
                </div>

                <div class="actions-row">
                    <Link<Route> to={Route::Score { user: props.user.clone() }} classes="secondary-action">
                        {"Finished"}
                    </Link<Route>>
                </div>
            </section>
        </div>
    }
}
