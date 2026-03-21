use anyhow::{ensure, Result};
use vocabu_larry_webui::{exam_viewmodel::ExamViewModel, login_viewmodel::LoginViewModel};

use crate::support::world::AcceptanceWorld;

pub async fn learner_dictionary_exists(world: &mut AcceptanceWorld) -> Result<()> {
    world
        .seed_dictionary("anna", "animals", "dog: Hund\ncat: Katze\n")
        .await
}

pub async fn backend_log_exists(world: &mut AcceptanceWorld) -> Result<()> {
    world
        .seed_log_file("2026-03-20.log", "LOGIN user=anna\nrequest failed\n")
        .await
}

pub async fn application_is_running(world: &mut AcceptanceWorld) -> Result<()> {
    world.start_application().await
}

pub async fn open_login_page(world: &mut AcceptanceWorld) -> Result<()> {
    let view_model = load_login_view_model(world).await?;
    ensure!(
        view_model.error_message().is_none(),
        "expected login view model to load without an error"
    );
    ensure!(
        !view_model.is_loading(),
        "expected login view model not to be loading anymore"
    );
    world.set_login_view_model(view_model);
    Ok(())
}

pub async fn choose_learner(world: &mut AcceptanceWorld) -> Result<()> {
    let base_url = world.base_url()?;
    let mut view_model = world.login_view_model()?.clone();

    ensure!(
        view_model.users().iter().any(|entry| entry.name == "anna"),
        "expected anna in login view model users"
    );

    if let Some(user) = view_model.select_user(Some("anna".to_owned())) {
        view_model.log_selected_user(user, &base_url).await;
    }

    world.set_selected_user("anna");
    world.set_login_view_model(view_model);
    Ok(())
}

pub async fn choose_dictionary(world: &mut AcceptanceWorld) -> Result<()> {
    let mut login_view_model = world.login_view_model()?.clone();

    ensure!(
        login_view_model
            .dictionaries()
            .iter()
            .any(|entry| entry == "animals"),
        "expected animals in login view model dictionaries"
    );

    login_view_model.select_dictionary(Some("animals".to_owned()));
    world.set_login_view_model(login_view_model);
    world.set_selected_dictionary("animals");
    world.set_exam_view_model(load_exam_view_model(world).await?);

    Ok(())
}

async fn load_login_view_model(world: &AcceptanceWorld) -> Result<LoginViewModel> {
    Ok(LoginViewModel::load(&world.base_url()?).await)
}

async fn load_exam_view_model(world: &AcceptanceWorld) -> Result<ExamViewModel> {
    let base_url = world.base_url()?;
    let user = world.selected_user().unwrap_or("anna");
    let dictionary = world.selected_dictionary().unwrap_or("animals");

    Ok(ExamViewModel::load(user, dictionary, &base_url, 0.0).await)
}
