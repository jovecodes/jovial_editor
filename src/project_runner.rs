use jovial_engine::prelude::*;
use crate::project_creator::{OpenSettings, OPEN_SETTINGS};
use std::process::Command;

pub struct ProjectRunner;

impl ProjectRunner {
    fn run(open_setting: &OpenSettings) {
        let output = Command::new("cargo")
            .current_dir(open_setting.path.clone())
            .arg("run")
            .output()
            .unwrap();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error running project: {}", stderr);
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("\n---Output---\n");
            println!("{}", stdout);
        }
    }

}

impl Entity for ProjectRunner {
    fn update(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
        egui::Window::new("Project").show(&game_state.ui.ctx(), |ui| {
            if ui.button("Run").clicked() {
                Self::run(game_state.plugins.get(OPEN_SETTINGS).unwrap());
            }
        });
    }
}

