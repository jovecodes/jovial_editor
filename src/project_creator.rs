use std::process::Command;

use jovial_engine::prelude::*;

use crate::{ADD_CHILD, project_runner::ProjectRunner};

pub const OPEN_SETTINGS: &str = "OpenSettings";

pub struct ProjectCreator {
    project_name: String,
    project_destination: String,
}

#[derive(Debug, Default)]
pub struct OpenSettings {
    pub path: String, 
    pub first_edit: bool,
}

impl Plugin for OpenSettings {
    fn update(&mut self) {}
}

impl ProjectCreator {
    pub fn new() -> Self {
        Self {
            project_name: String::from("project_name"),
            project_destination: String::from(
                r#"C:\Users\Jove\Documents\Projects\Code\Rust\Games"#,
            ),
        }
    }

    pub fn create(&mut self, open_settings: &mut OpenSettings) {
        let output = Command::new("cargo")
            .current_dir(self.project_destination.clone())
            .arg("new")
            .arg(&self.project_name)
            .output()
            .expect("could not create project");

        open_settings.path = self.project_destination.clone() + "/" + &self.project_name;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("Error creating project: {}", stderr);

            open_settings.first_edit = false;
        } else {
            open_settings.first_edit = true;
        }
    }

    fn show_ui(&mut self, ui: &mut egui::Ui, entity_commands: &mut EntityCommands, game_state: &mut GameState) {
        ui.text_edit_singleline(&mut self.project_name);
        ui.text_edit_singleline(&mut self.project_destination);
        if ui.button("Create Project").clicked() {
            self.create(game_state.plugins.get_mut(OPEN_SETTINGS).unwrap());
            entity_commands.die();
            game_state.events.add::<Box<dyn Entity>>(
                ADD_CHILD,
                Box::new(ProjectRunner),
            )
        }
    }
}

impl Entity for ProjectCreator {
    fn update(&mut self, game_state: &mut GameState, entity_commands: &mut EntityCommands) {
        egui::Window::new("New Project").show(&game_state.ui.ctx(), |ui| {
            self.show_ui(ui, entity_commands, game_state);
        });
    }
}
