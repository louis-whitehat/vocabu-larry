use anyhow::Result;

use crate::{pages::login, support::world::AcceptanceWorld};

pub async fn learner_dictionary_exists(world: &mut AcceptanceWorld) -> Result<()> {
    world.seed_dictionary("anna", "animals", "dog: Hund\ncat: Katze\n").await
}

pub async fn backend_log_exists(world: &mut AcceptanceWorld) -> Result<()> {
    world.seed_log_file("2026-03-20.log", "LOGIN user=anna\nrequest failed\n").await
}

pub async fn application_is_running(world: &mut AcceptanceWorld) -> Result<()> {
    world.start_application().await
}

pub async fn open_login_page(world: &mut AcceptanceWorld) -> Result<()> {
    let base_url = world.base_url()?;
    login::open(world.browser()?, &base_url).await
}

pub async fn choose_learner(world: &mut AcceptanceWorld) -> Result<()> {
    login::select_user(world.browser()?, "anna").await
}

pub async fn choose_dictionary(world: &mut AcceptanceWorld) -> Result<()> {
    login::select_dictionary(world.browser()?, "animals").await
}