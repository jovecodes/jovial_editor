use jovial_engine::prelude::*;
use crate::project_creator::{OpenSettings, OPEN_SETTINGS};
use std::process::Command;
use std::thread::{self, JoinHandle};


pub const SAVE: &str = "Save";

pub struct ProjectRunner {
    run_handle: Option<JoinHandle<()>>,
    build_handle: Option<JoinHandle<()>>,
    run_queued: bool
}

impl ProjectRunner {
    pub fn new() -> Self {
        Self { 
            run_handle: None, 
            build_handle: None,
            run_queued: false
        }
    }

    fn cargo_build(open_settings: OpenSettings) {
        let output = Command::new("cargo")
            .current_dir(open_settings.path.clone())
            .arg("build")
            .output()
            .unwrap();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("BUILD ERROR: {}", stderr);
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if stdout.is_empty() {
                return;
            }
            println!("\n---Build---");
            println!("{}", stdout);
        }
    }

    fn cargo_run(open_settings: OpenSettings) {
        let output = Command::new("cargo")
            .current_dir(open_settings.path.clone())
            .arg("run")
            .output()
            .unwrap();

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("RUN ERROR: {}", stderr);
        } else {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("\n---Output---");
            println!("{}", stdout);
        }
    }

    fn run(&mut self, game_state: &mut GameState) {
        let open_settings = game_state.plugins.get_mut::<OpenSettings>(OPEN_SETTINGS).unwrap();
        let open_settings_copy = open_settings.clone();
        let run_handle = thread::spawn(move || Self::cargo_run(open_settings_copy));
        self.run_handle = Some(run_handle);
        self.run_queued = false;
    }

    fn build(&mut self, game_state: &mut GameState) {
        let open_settings = game_state.plugins.get_mut::<OpenSettings>(OPEN_SETTINGS).unwrap();
        let open_settings_copy = open_settings.clone();
        let build_handle = thread::spawn(move || Self::cargo_build(open_settings_copy));
        self.build_handle = Some(build_handle);
    }

    fn can_run(&self) -> bool {
        self.run_handle.is_none() && self.build_handle.is_none()
    }
}

impl Entity for ProjectRunner {
    fn start(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
        if let Some(open_settings) = game_state.plugins.get::<OpenSettings>(OPEN_SETTINGS) {
            if open_settings.first_edit {
                self.build(game_state);
                self.run_queued = true;
            }
        }
    }

    fn update(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
        egui::Window::new("Project").show(&game_state.ui.ctx(), |ui| {
            ui.horizontal(|button_ui| {
                if button_ui.button("Run").clicked() && self.can_run() {
                    self.build(game_state);
                    self.run_queued = true;
                }
                if button_ui.button("Build").clicked() && self.can_run() {
                    self.build(game_state);
                }
                if button_ui.button("Save").clicked() {
                    game_state.events.add(SAVE, true)
                }
            });


            if let Some(build_handle) = &self.build_handle {
                ui.spinner();
                if build_handle.is_finished() == false {
                    return;
                }
                self.build_handle = None;
                if self.run_queued {
                    self.run(game_state);
                }
            }
            if let Some(handle) = &self.run_handle {
                if handle.is_finished() {
                    self.run_handle = None;
                    game_state.plugins
                        .get_mut::<OpenSettings>(OPEN_SETTINGS)
                        .unwrap()
                        .has_run = true;
                }
            }
        });
    }
}

