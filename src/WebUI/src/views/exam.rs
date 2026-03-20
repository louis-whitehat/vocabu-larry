use std::collections::HashMap;

use js_sys::Math;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::api;
use crate::models::DictionaryEntry;
use crate::Route;

#[derive(Properties, PartialEq)]
pub struct ExamViewProps {
    pub user: String,
    pub dictionary: String,
}

fn get_entry_key(entry: &DictionaryEntry) -> String {
    format!("{}\n{}", entry.word, entry.translation)
}

fn get_entry_weight(entry: &DictionaryEntry, failure_counts: &HashMap<String, u32>) -> u32 {
    1 + failure_counts.get(&get_entry_key(entry)).copied().unwrap_or(0)
}

fn select_weighted_entry(entries: &[DictionaryEntry], failure_counts: &HashMap<String, u32>) -> Option<DictionaryEntry> {
    if entries.is_empty() {
        return None;
    }

    let total_weight: u32 = entries.iter().map(|entry| get_entry_weight(entry, failure_counts)).sum();
    let mut remaining_weight = Math::random() * f64::from(total_weight);

    for entry in entries {
        remaining_weight -= f64::from(get_entry_weight(entry, failure_counts));
        if remaining_weight < 0.0 {
            return Some(entry.clone());
        }
    }

    entries.last().cloned()
}

fn normalize_answer(value: &str) -> String {
    value.split_whitespace().collect::<Vec<_>>().join(" ").to_lowercase()
}

fn strip_leading_token(value: &str, token: &str) -> String {
    if value.starts_with(&format!("{token} ")) {
        value[token.len() + 1..].to_owned()
    } else {
        value.to_owned()
    }
}

fn get_dictionary_language(dictionary_name: &str) -> Option<&str> {
    let language = dictionary_name.split('.').next()?.trim().to_lowercase();

    match language.as_str() {
        "english" => Some("english"),
        "french" => Some("french"),
        _ => None,
    }
}

fn answers_match(expected_answer: &str, actual_answer: &str, dictionary_name: &str) -> bool {
    let normalized_expected = normalize_answer(expected_answer);
    let normalized_actual = normalize_answer(actual_answer);

    if normalized_expected == normalized_actual {
        return true;
    }

    if get_dictionary_language(dictionary_name) != Some("english") {
        return false;
    }

    strip_leading_token(&normalized_expected, "to") == strip_leading_token(&normalized_actual, "to")
        || strip_leading_token(&normalized_expected, "the") == strip_leading_token(&normalized_actual, "the")
}

#[function_component(ExamView)]
pub fn exam_view(props: &ExamViewProps) -> Html {
    let entries = use_state(Vec::<DictionaryEntry>::new);
    let current_entry = use_state(|| None::<DictionaryEntry>);
    let input = use_state(String::new);
    let your_answer = use_state(|| None::<String>);
    let previous_correct = use_state(|| None::<String>);
    let answer_correct = use_state(|| None::<bool>);
    let total_count = use_state(|| 0_u32);
    let correct_count = use_state(|| 0_u32);
    let failure_counts = use_state(HashMap::<String, u32>::new);
    let load_error = use_state(|| None::<String>);

    {
        let entries = entries.clone();
        let current_entry = current_entry.clone();
        let failure_counts = failure_counts.clone();
        let load_error = load_error.clone();
        let user = props.user.clone();
        let dictionary = props.dictionary.clone();

        use_effect_with((props.user.clone(), props.dictionary.clone()), move |_| {
            spawn_local(async move {
                match api::fetch_dictionary(&user, &dictionary).await {
                    Ok(dictionary_entries) => {
                        let next_failure_counts = HashMap::new();
                        let next_entry = select_weighted_entry(&dictionary_entries, &next_failure_counts);
                        entries.set(dictionary_entries);
                        current_entry.set(next_entry);
                        failure_counts.set(next_failure_counts);
                        load_error.set(None);
                    }
                    Err(error) => load_error.set(Some(format!("Failed to fetch dictionary: {error}"))),
                }
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
        let entries = entries.clone();
        let current_entry = current_entry.clone();
        let input = input.clone();
        let your_answer = your_answer.clone();
        let previous_correct = previous_correct.clone();
        let answer_correct = answer_correct.clone();
        let total_count = total_count.clone();
        let correct_count = correct_count.clone();
        let failure_counts = failure_counts.clone();
        let user = props.user.clone();
        let dictionary = props.dictionary.clone();

        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let Some(entry) = (*current_entry).clone() else {
                return;
            };

            let given_answer = (*input).clone();
            let is_correct = answers_match(&entry.translation, &given_answer, &dictionary);
            previous_correct.set(Some(entry.translation.clone()));
            your_answer.set(Some(given_answer.clone()));
            answer_correct.set(Some(is_correct));
            total_count.set(*total_count + 1);
            if is_correct {
                correct_count.set(*correct_count + 1);
            }

            let mut next_failure_counts = (*failure_counts).clone();
            if !is_correct {
                let entry_key = get_entry_key(&entry);
                let failures = next_failure_counts.get(&entry_key).copied().unwrap_or(0);
                next_failure_counts.insert(entry_key, failures + 1);
            }

            let next_entry = select_weighted_entry(entries.as_ref(), &next_failure_counts);
            failure_counts.set(next_failure_counts);
            current_entry.set(next_entry);
            input.set(String::new());

            let user = user.clone();
            let dictionary = dictionary.clone();
            spawn_local(async move {
                let _ = api::post_score(user, dictionary, is_correct).await;
            });
        })
    };

    let status_class = match *answer_correct {
        Some(true) => "feedback-panel correct",
        Some(false) => "feedback-panel wrong",
        None => "feedback-panel",
    };

    let num_words = current_entry
        .as_ref()
        .as_ref()
        .map(|entry| entry.translation.split_whitespace().count())
        .unwrap_or(0);

    html! {
        <div class="page-shell exam-page">
            <section class="panel-card exam-card">
                <h1 class="page-title">{"Exam"}</h1>
                <p class="page-copy">{"Answer the translation and keep the streak moving."}</p>

                if let Some(error) = &*load_error {
                    <p class="error-message load-error">{error.clone()}</p>
                }

                <div class="exam-question">
                    <span class="muted-note">{"What is the translation of"}</span>
                    <span class="word">{current_entry.as_ref().as_ref().map(|entry| entry.word.clone()).unwrap_or_default()}</span>
                    <span class="hint-pill">{format!("Hint: {num_words} word(s)")}</span>
                </div>

                <form onsubmit={on_submit} class="exam-form">
                    <input type="text" value={(*input).clone()} oninput={on_input} class="answer-input" />
                    <button type="submit">{"Submit"}</button>
                    <span class="score-chip">{format!("{} / {}", *correct_count, *total_count)}</span>
                </form>

                <div class={status_class}>
                    if *answer_correct == Some(true) {
                        <div>{"Correct 👍😉"}</div>
                    }
                    if *answer_correct == Some(false) {
                        <div>
                            {"Sorry 🙁 Your answer "}
                            <span class="word">{your_answer.as_deref().map(str::to_owned).unwrap_or_default()}</span>
                            {" is not correct, correct answer would have been "}
                            <span class="word">{previous_correct.as_deref().map(str::to_owned).unwrap_or_default()}</span>
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
