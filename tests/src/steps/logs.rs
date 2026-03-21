use anyhow::{ensure, Result};
use vocabu_larry_webui::logs_viewmodel::LogsViewModel;

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

pub async fn should_see_log_content_containing(
    world: &mut AcceptanceWorld,
    fragment: &str,
) -> Result<()> {
    ensure!(
        world.logs_view_model()?.content().contains(fragment),
        "expected log content to contain {fragment}"
    );
    Ok(())
}

pub async fn should_see_single_selected_log_file(world: &mut AcceptanceWorld) -> Result<()> {
    let view_model = world.logs_view_model()?;
    ensure!(
        view_model.files().len() == 1,
        "expected exactly one available log file"
    );
    ensure!(
        view_model.selected_file() == Some(view_model.files()[0].as_str()),
        "expected the only available log file to be selected"
    );
    Ok(())
}

pub async fn should_see_available_log_file_count(
    world: &mut AcceptanceWorld,
    expected_count: usize,
) -> Result<()> {
    ensure!(
        world.logs_view_model()?.files().len() == expected_count,
        "expected {expected_count} available log files"
    );
    Ok(())
}

async fn load_logs_view_model(
    world: &AcceptanceWorld,
    file: Option<&str>,
) -> Result<LogsViewModel> {
    let base_url = world.base_url()?;

    Ok(LogsViewModel::load(file, &base_url).await)
}
