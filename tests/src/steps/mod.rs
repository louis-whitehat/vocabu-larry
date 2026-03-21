mod common;
mod exam;
mod logs;

use anyhow::{bail, Result};

use crate::support::world::AcceptanceWorld;

pub async fn dispatch(world: &mut AcceptanceWorld, step: &str) -> Result<()> {
    if let Some((user, dictionary)) =
        parse_two_quoted_values(step, "learner ", " with ", " dictionary exists")
    {
        return common::learner_dictionary_exists(world, &user, &dictionary).await;
    }

    if let Some(user) = parse_single_quoted_value(step, "I choose learner ") {
        return common::choose_learner(world, &user).await;
    }

    if let Some(dictionary) = parse_single_quoted_value(step, "I choose dictionary ") {
        return common::choose_dictionary(world, &dictionary).await;
    }

    if let Some(word) = parse_single_quoted_value(step, "I should see the question for ") {
        return exam::should_see_question(world, &word).await;
    }

    if let Some(answer) = parse_single_quoted_value(step, "I answer ") {
        return exam::answer(world, &answer).await;
    }

    if let Some((dictionary, remainder)) =
        parse_quoted_value_with_remainder(step, "the score table should show ")
    {
        if remainder == " with 1 correct out of 1" {
            return exam::score_table_should_show_result(world, &dictionary).await;
        }
    }

    match step {
        "a backend log file exists" => common::backend_log_exists(world).await,
        "the application is running" => common::application_is_running(world).await,
        "I open the login page" => common::open_login_page(world).await,
        "I should see the exam page" => exam::should_see_exam_page(world).await,
        "I should see that the answer was correct" => exam::answer_correct(world).await,
        "I finish the exam" => exam::finish_exam(world).await,
        "I should see the score page" => exam::should_see_score_page(world).await,
        "I open the logs page" => logs::open_logs_page(world).await,
        "I choose the seeded log file" => logs::choose_seeded_log_file(world).await,
        "I should see the log content" => logs::should_see_log_content(world).await,
        _ => bail!("unhandled acceptance step: {step}"),
    }
}

fn parse_single_quoted_value(step: &str, prefix: &str) -> Option<String> {
    let suffix = step.strip_prefix(prefix)?;
    let value = suffix.strip_prefix('\'')?.strip_suffix('\'')?;
    Some(value.to_owned())
}

fn parse_quoted_value_with_remainder<'a>(step: &'a str, prefix: &str) -> Option<(String, &'a str)> {
    let remainder = step.strip_prefix(prefix)?;
    split_quoted_value(remainder)
}

fn parse_two_quoted_values(
    step: &str,
    prefix: &str,
    infix: &str,
    suffix: &str,
) -> Option<(String, String)> {
    let remainder = step.strip_prefix(prefix)?;
    let (left, remainder) = split_quoted_value(remainder)?;
    let remainder = remainder.strip_prefix(infix)?;
    let (right, remainder) = split_quoted_value(remainder)?;
    if remainder == suffix {
        Some((left, right))
    } else {
        None
    }
}

fn split_quoted_value(value: &str) -> Option<(String, &str)> {
    let value = value.strip_prefix('\'')?;
    let closing_quote = value.find('\'')?;
    let quoted = value[..closing_quote].to_owned();
    let remainder = &value[closing_quote + 1..];
    Some((quoted, remainder))
}
