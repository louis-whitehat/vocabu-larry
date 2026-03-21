use anyhow::{ensure, Result};
use vocabu_larry_webui::{
    exam_viewmodel::{dictionary_path, DictionaryEntry, ExamViewModel},
    login_viewmodel::{login_path, login_request, users_path, LoginViewModel, UserEntry},
};

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
        reqwest::Client::new()
            .post(format!("{base_url}{}", login_path()))
            .json(&login_request(user))
            .send()
            .await?
            .error_for_status()?;
        view_model.mark_user_logged("anna".to_owned());
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
    let url = format!("{}{}", world.base_url()?, users_path());

    Ok(LoginViewModel::load_with(|| async move {
        reqwest::get(url)
            .await
            .map_err(|error| error.to_string())?
            .json::<Vec<UserEntry>>()
            .await
            .map_err(|error| error.to_string())
    })
    .await)
}

async fn load_exam_view_model(world: &AcceptanceWorld) -> Result<ExamViewModel> {
    let base_url = world.base_url()?;
    let user = world.selected_user().unwrap_or("anna");
    let dictionary = world.selected_dictionary().unwrap_or("animals");

    Ok(ExamViewModel::load_with(
        || async move {
            let url = format!("{base_url}{}", dictionary_path(user, dictionary));

            reqwest::get(url)
                .await
                .map_err(|error| error.to_string())?
                .json::<Vec<DictionaryEntry>>()
                .await
                .map_err(|error| error.to_string())
        },
        0.0,
    )
    .await)
}
