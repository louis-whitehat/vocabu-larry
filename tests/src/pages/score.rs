use anyhow::Result;
use fantoccini::{Client, Locator};

pub async fn wait_for_score_page(client: &Client) -> Result<()> {
    client.wait().for_element(Locator::Css("#score-page")).await?;
    Ok(())
}

pub async fn table_text(client: &Client) -> Result<String> {
    Ok(client.find(Locator::Css("#score-table")).await?.text().await?)
}