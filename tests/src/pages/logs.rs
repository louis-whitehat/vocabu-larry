use anyhow::Result;
use fantoccini::{Client, Locator};
use serde_json::json;

pub async fn open(client: &Client) -> Result<()> {
    client.find(Locator::Css("a[href='/logs']")).await?.click().await?;
    client.wait().for_element(Locator::Css("#logs-page")).await?;
    Ok(())
}

pub async fn select_log_file(client: &Client, file_name: &str) -> Result<()> {
    client.wait().for_element(Locator::Css("#log-file")).await?;
    client
        .execute(
            r#"
            const select = document.getElementById('log-file');
            const option = Array.from(select.options).find((entry) => entry.text === arguments[0]);
            if (!option) {
                throw new Error(`missing log file '${arguments[0]}'`);
            }

            select.value = option.value;
            select.dispatchEvent(new Event('change', { bubbles: true }));
            "#,
            vec![json!(file_name)],
        )
        .await?;
    Ok(())
}

pub async fn content_text(client: &Client) -> Result<String> {
    Ok(client.find(Locator::Css("#log-content")).await?.text().await?)
}