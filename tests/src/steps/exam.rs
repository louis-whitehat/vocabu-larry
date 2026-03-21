use anyhow::{ensure, Result};
use vocabu_larry_webui::score_viewmodel::ScoreViewModel;

use crate::support::world::AcceptanceWorld;

pub async fn should_see_exam_page(world: &mut AcceptanceWorld) -> Result<()> {
    let view_model = world.exam_view_model()?;
    ensure!(
        view_model.error_message().is_none(),
        "expected exam view model to load without an error"
    );
    ensure!(
        !view_model.is_loading(),
        "expected exam view model not to be loading anymore"
    );
    ensure!(
        view_model.current_word().is_some(),
        "expected exam view model to contain a question"
    );
    Ok(())
}

pub async fn should_see_question(world: &mut AcceptanceWorld, word: &str) -> Result<()> {
    ensure!(
        world.exam_view_model()?.current_word() == Some(word),
        "expected the current word to be {word}"
    );
    Ok(())
}

pub async fn answer(world: &mut AcceptanceWorld, answer: &str) -> Result<()> {
    let base_url = world.base_url()?;
    let user = world.selected_user().unwrap_or("anna").to_owned();
    let dictionary = world.selected_dictionary().unwrap_or("animals").to_owned();
    let mut view_model = world.exam_view_model()?.clone();

    view_model
        .submit_answer(answer.to_owned(), user, dictionary, &base_url, 0.0)
        .await;

    world.set_exam_view_model(view_model);
    Ok(())
}

pub async fn answer_correct(world: &mut AcceptanceWorld) -> Result<()> {
    ensure!(
        world.exam_view_model()?.answer_correct() == Some(true),
        "expected a correct answer message"
    );
    Ok(())
}

pub async fn finish_exam(world: &mut AcceptanceWorld) -> Result<()> {
    let view_model = load_score_view_model(world).await?;
    world.set_score_view_model(view_model);
    Ok(())
}

pub async fn should_see_score_page(world: &mut AcceptanceWorld) -> Result<()> {
    let view_model = world.score_view_model()?;
    ensure!(
        view_model.error_message().is_none(),
        "expected score view model to load without an error"
    );
    ensure!(
        !view_model.is_loading(),
        "expected score view model not to be loading anymore"
    );
    Ok(())
}

pub async fn score_table_should_show_result(
    world: &mut AcceptanceWorld,
    dictionary_name: &str,
    correct: u64,
    total: u64,
) -> Result<()> {
    let rows = world.score_view_model()?.rows();

    ensure!(
        rows.iter().any(|row| row.dictionary == dictionary_name),
        "expected {dictionary_name} in score view model rows"
    );
    ensure!(
        rows.iter().any(|row| row.dictionary == dictionary_name
            && row.correct == correct
            && row.total == total),
        "expected {dictionary_name} with {correct} correct out of {total} in score view model rows"
    );
    Ok(())
}

async fn load_score_view_model(world: &AcceptanceWorld) -> Result<ScoreViewModel> {
    let base_url = world.base_url()?;
    let user = world.selected_user().unwrap_or("anna").to_owned();

    Ok(ScoreViewModel::load(&user, &base_url).await)
}
