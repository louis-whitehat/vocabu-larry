use anyhow::{ensure, Result};

use crate::{pages::logs, support::world::AcceptanceWorld};

pub async fn open_logs_page(world: &mut AcceptanceWorld) -> Result<()> {
    logs::open(world.browser()?).await
}

pub async fn choose_seeded_log_file(world: &mut AcceptanceWorld) -> Result<()> {
    logs::select_log_file(world.browser()?, "2026-03-20.log").await
}

pub async fn should_see_log_content(world: &mut AcceptanceWorld) -> Result<()> {
    let content = logs::content_text(world.browser()?).await?;
    ensure!(content.contains("LOGIN user=anna"), "expected the seeded log content to be visible");
    Ok(())
}