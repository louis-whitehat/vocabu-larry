use anyhow::{ensure, Result};
use tokio::time::{sleep, Duration};

use crate::{
    pages::{exam, score},
    support::world::AcceptanceWorld,
};

pub async fn should_see_exam_page(world: &mut AcceptanceWorld) -> Result<()> {
    exam::wait_for_exam_page(world.browser()?).await
}

pub async fn should_see_dog_question(world: &mut AcceptanceWorld) -> Result<()> {
    ensure!(
        exam::current_word(world.browser()?).await? == "dog",
        "expected the current word to be dog"
    );
    Ok(())
}

pub async fn answer_hund(world: &mut AcceptanceWorld) -> Result<()> {
    exam::submit_answer(world.browser()?, "Hund").await
}

pub async fn answer_correct(world: &mut AcceptanceWorld) -> Result<()> {
    for _ in 0..50 {
        if exam::feedback_text(world.browser()?)
            .await?
            .contains("Correct")
        {
            return Ok(());
        }

        sleep(Duration::from_millis(200)).await;
    }

    ensure!(
        exam::feedback_text(world.browser()?)
            .await?
            .contains("Correct"),
        "expected a correct answer message"
    );
    Ok(())
}

pub async fn finish_exam(world: &mut AcceptanceWorld) -> Result<()> {
    exam::finish(world.browser()?).await
}

pub async fn should_see_score_page(world: &mut AcceptanceWorld) -> Result<()> {
    score::wait_for_score_page(world.browser()?).await
}

pub async fn score_table_should_show_result(world: &mut AcceptanceWorld) -> Result<()> {
    let score_text = score::table_text(world.browser()?).await?;
    ensure!(
        score_text.contains("animals"),
        "expected animals in score table"
    );
    ensure!(
        score_text.contains("1"),
        "expected score values to include 1"
    );
    Ok(())
}
