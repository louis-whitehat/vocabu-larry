mod steps;
mod support;

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use tokio::fs;

use support::world::AcceptanceWorld;

#[derive(Debug)]
struct Scenario {
    feature_name: String,
    scenario_name: String,
    steps: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    for scenario in load_scenarios(&Path::new(env!("CARGO_MANIFEST_DIR")).join("features")).await? {
        println!(
            "Running feature '{}' scenario '{}'",
            scenario.feature_name, scenario.scenario_name
        );

        let mut world = AcceptanceWorld::default();
        for step in scenario.steps {
            steps::dispatch(&mut world, &step).await.with_context(|| {
                format!(
                    "step '{step}' failed in scenario '{}'",
                    scenario.scenario_name
                )
            })?;
        }
    }

    Ok(())
}

async fn load_scenarios(features_dir: &Path) -> Result<Vec<Scenario>> {
    let mut entries = fs::read_dir(features_dir).await?;
    let mut feature_paths = Vec::<PathBuf>::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) == Some("feature") {
            feature_paths.push(path);
        }
    }

    feature_paths.sort();

    let mut scenarios = Vec::new();
    for feature_path in feature_paths {
        scenarios.extend(parse_feature_file(&feature_path).await?);
    }

    Ok(scenarios)
}

async fn parse_feature_file(path: &Path) -> Result<Vec<Scenario>> {
    let content = fs::read_to_string(path).await?;
    let mut feature_name = path
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("unknown")
        .to_owned();
    let mut scenarios = Vec::<Scenario>::new();
    let mut current_scenario = None::<Scenario>;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some(name) = line.strip_prefix("Feature:") {
            feature_name = name.trim().to_owned();
            continue;
        }

        if let Some(name) = line.strip_prefix("Scenario:") {
            if let Some(scenario) = current_scenario.take() {
                scenarios.push(scenario);
            }

            current_scenario = Some(Scenario {
                feature_name: feature_name.clone(),
                scenario_name: name.trim().to_owned(),
                steps: Vec::new(),
            });
            continue;
        }

        for keyword in ["Given", "When", "Then", "And"] {
            if let Some(step) = line.strip_prefix(keyword) {
                if let Some(scenario) = &mut current_scenario {
                    scenario.steps.push(step.trim().to_owned());
                }
                break;
            }
        }
    }

    if let Some(scenario) = current_scenario {
        scenarios.push(scenario);
    }

    Ok(scenarios)
}
