mod common;
mod exam;
mod logs;

use anyhow::{bail, Result};

use crate::support::world::AcceptanceWorld;

pub async fn dispatch(world: &mut AcceptanceWorld, step: &str) -> Result<()> {
	match step {
		"learner anna with animals dictionary exists" => common::learner_dictionary_exists(world).await,
		"a backend log file exists" => common::backend_log_exists(world).await,
		"the application is running" => common::application_is_running(world).await,
		"I open the login page" => common::open_login_page(world).await,
		"I choose learner anna" => common::choose_learner(world).await,
		"I choose dictionary animals" => common::choose_dictionary(world).await,
		"I should see the exam page" => exam::should_see_exam_page(world).await,
		"I should see the question for dog" => exam::should_see_dog_question(world).await,
		"I answer Hund" => exam::answer_hund(world).await,
		"I should see that the answer was correct" => exam::answer_correct(world).await,
		"I finish the exam" => exam::finish_exam(world).await,
		"I should see the score page" => exam::should_see_score_page(world).await,
		"the score table should show animals with 1 correct out of 1" => {
			exam::score_table_should_show_result(world).await
		}
		"I open the logs page" => logs::open_logs_page(world).await,
		"I choose the seeded log file" => logs::choose_seeded_log_file(world).await,
		"I should see the log content" => logs::should_see_log_content(world).await,
		_ => bail!("unhandled acceptance step: {step}"),
	}
}