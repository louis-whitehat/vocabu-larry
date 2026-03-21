use anyhow::{ensure, Result};
use vocabu_larry_webui::score_viewmodel::{ScoreStore, ScoreViewModel};

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

pub async fn should_see_dog_question(world: &mut AcceptanceWorld) -> Result<()> {
    ensure!(
        world.exam_view_model()?.current_word() == Some("dog"),
        "expected the current word to be dog"
    );
    Ok(())
}

pub async fn answer_hund(world: &mut AcceptanceWorld) -> Result<()> {
    let base_url = world.base_url()?;
    let user = world.selected_user().unwrap_or("anna").to_owned();
    let dictionary = world.selected_dictionary().unwrap_or("animals").to_owned();
    let mut view_model = world.exam_view_model()?.clone();

    view_model
        .submit_answer_with(
            "Hund".to_owned(),
            user,
            dictionary,
            0.0,
            move |user, dictionary, is_correct| {
                let base_url = base_url.clone();
                async move {
                    reqwest::Client::new()
                        .post(format!("{base_url}/api/score"))
                        .json(&serde_json::json!({
                            "user": user,
                            "dictionary": dictionary,
                            "isCorrect": is_correct,
                        }))
                        .send()
                        .await
                        .map_err(|error| error.to_string())?
                        .error_for_status()
                        .map(|_| ())
                        .map_err(|error| error.to_string())
                }
            },
        )
        .await
        .map_err(anyhow::Error::msg)?;

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

pub async fn score_table_should_show_result(world: &mut AcceptanceWorld) -> Result<()> {
    let rows = world.score_view_model()?.rows();

    ensure!(
        rows.iter().any(|row| row.dictionary == "animals"),
        "expected animals in score view model rows"
    );
    ensure!(
        rows.iter()
            .any(|row| row.dictionary == "animals" && row.correct == 1 && row.total == 1),
        "expected animals with 1 correct out of 1 in score view model rows"
    );
    Ok(())
}

async fn load_score_view_model(world: &AcceptanceWorld) -> Result<ScoreViewModel> {
    let base_url = world.base_url()?;
    let user = world.selected_user().unwrap_or("anna").to_owned();

    Ok(ScoreViewModel::load_with(|| async move {
        let mut url = reqwest::Url::parse(&format!("{base_url}/api/score"))
            .map_err(|error| error.to_string())?;
        url.query_pairs_mut().append_pair("user", &user);

        reqwest::get(url)
            .await
            .map_err(|error| error.to_string())?
            .json::<ScoreStore>()
            .await
            .map_err(|error| error.to_string())
    })
    .await)
}
