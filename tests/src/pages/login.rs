use anyhow::{anyhow, Result};
use fantoccini::{Client, Locator};
use serde_json::json;
use tokio::time::{sleep, Duration};

pub async fn open(client: &Client, base_url: &str) -> Result<()> {
    let response = reqwest::get(base_url).await?;
    if !response.status().is_success() {
        return Err(anyhow!(
            "login page request returned unexpected status {} for {base_url}",
            response.status()
        ));
    }

    client.goto(base_url).await?;
    if client
        .wait()
        .for_element(Locator::Css("#login-page"))
        .await
        .is_err()
    {
        let current_url = client
            .current_url()
            .await
            .map(|value| value.to_string())
            .unwrap_or_else(|_| "<unavailable>".to_owned());
        let page_source = client
            .source()
            .await
            .map(|source| source.chars().take(1200).collect::<String>())
            .unwrap_or_else(|_| "<unavailable>".to_owned());
        return Err(anyhow!(
            "login page did not render at {current_url}; page source starts with: {page_source}"
        ));
    }
    Ok(())
}

pub async fn select_user(client: &Client, user: &str) -> Result<()> {
    wait_for_select_option(client, "user-select", user).await?;
    set_select_by_visible_text(client, "user-select", user).await
}

pub async fn select_dictionary(client: &Client, dictionary: &str) -> Result<()> {
    client
        .wait()
        .for_element(Locator::Css("#dictionary-select"))
        .await?;
    wait_for_select_option(client, "dictionary-select", dictionary).await?;
    set_select_by_visible_text(client, "dictionary-select", dictionary).await
}

async fn set_select_by_visible_text(client: &Client, select_id: &str, text: &str) -> Result<()> {
    client
        .execute(
            r#"
            const select = document.getElementById(arguments[0]);
            const option = Array.from(select.options).find((entry) => entry.text === arguments[1]);
            if (!option) {
                throw new Error(`missing option '${arguments[1]}' for select '${arguments[0]}'`);
            }

            select.value = option.value;
            select.dispatchEvent(new Event('change', { bubbles: true }));
            "#,
            vec![json!(select_id), json!(text)],
        )
        .await?;

    Ok(())
}

async fn wait_for_select_option(client: &Client, select_id: &str, text: &str) -> Result<()> {
    for _ in 0..50 {
        let option_exists = client
            .execute(
                r#"
                const select = document.getElementById(arguments[0]);
                if (!select) {
                    return false;
                }

                return Array.from(select.options).some((entry) => entry.text === arguments[1]);
                "#,
                vec![json!(select_id), json!(text)],
            )
            .await?;

        if option_exists.as_bool() == Some(true) {
            return Ok(());
        }

        sleep(Duration::from_millis(200)).await;
    }

    let available_options = client
        .execute(
            r#"
            const select = document.getElementById(arguments[0]);
            if (!select) {
                return [];
            }

            return Array.from(select.options).map((entry) => entry.text);
            "#,
            vec![json!(select_id)],
        )
        .await?;
    let api_users_response = if select_id == "user-select" {
        let current_url = client.current_url().await?.to_string();
        let api_url = format!("{}/api/users", current_url.trim_end_matches('/'));
        Some(reqwest::get(&api_url).await?.text().await?)
    } else {
        None
    };

    Err(anyhow!(
        "option '{text}' did not appear in select '{select_id}' in time; available options: {}; api/users: {}",
        available_options
            .as_array()
            .map(|values| values
                .iter()
                .filter_map(|value| value.as_str())
                .collect::<Vec<_>>()
                .join(", "))
            .unwrap_or_else(|| "<unavailable>".to_owned()),
        api_users_response.unwrap_or_else(|| "<not checked>".to_owned())
    ))
}
