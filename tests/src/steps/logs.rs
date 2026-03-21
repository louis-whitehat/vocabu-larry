use anyhow::{ensure, Result};
use vocabu_larry_webui::logs_viewmodel::{LogResponse, LogsViewModel};

use crate::support::world::AcceptanceWorld;

pub async fn open_logs_page(world: &mut AcceptanceWorld) -> Result<()> {
    world.set_logs_view_model(load_logs_view_model(world, None).await?);
    Ok(())
}

pub async fn choose_seeded_log_file(world: &mut AcceptanceWorld) -> Result<()> {
    world.set_logs_view_model(load_logs_view_model(world, Some("2026-03-20.log")).await?);
    Ok(())
}

pub async fn should_see_log_content(world: &mut AcceptanceWorld) -> Result<()> {
    let view_model = world.logs_view_model()?;
    ensure!(
        view_model.content().contains("LOGIN user=anna"),
        "expected the seeded log content to be visible"
    );
    Ok(())
}

async fn load_logs_view_model(
    world: &AcceptanceWorld,
    file: Option<&str>,
) -> Result<LogsViewModel> {
    let base_url = world.base_url()?;
    let requested_file = file.map(str::to_owned);

    Ok(LogsViewModel::load_with(|| async move {
        let mut url = reqwest::Url::parse(&format!("{base_url}/api/logs"))
            .map_err(|error| error.to_string())?;

        if let Some(file) = requested_file.as_deref() {
            url.query_pairs_mut().append_pair("file", file);
        }

        reqwest::get(url)
            .await
            .map_err(|error| error.to_string())?
            .json::<LogResponse>()
            .await
            .map_err(|error| error.to_string())
    })
    .await)
}
