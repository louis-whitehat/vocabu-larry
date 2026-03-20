use anyhow::Result;
use fantoccini::{Client, Locator};

pub async fn wait_for_exam_page(client: &Client) -> Result<()> {
    client.wait().for_element(Locator::Css("#exam-page")).await?;
    Ok(())
}

pub async fn current_word(client: &Client) -> Result<String> {
    Ok(client.find(Locator::Css("#exam-word")).await?.text().await?)
}

pub async fn submit_answer(client: &Client, answer: &str) -> Result<()> {
    client.find(Locator::Css("#answer-input")).await?.send_keys(answer).await?;
    client.find(Locator::Css("#submit-answer-button")).await?.click().await?;
    Ok(())
}

pub async fn feedback_text(client: &Client) -> Result<String> {
    Ok(client.find(Locator::Css("#answer-feedback")).await?.text().await?)
}

pub async fn finish(client: &Client) -> Result<()> {
    client.find(Locator::LinkText("Finished")).await?.click().await?;
    Ok(())
}