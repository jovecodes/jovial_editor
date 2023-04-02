use jovial_engine::prelude::*;
use jovial_engine::prelude::egui::Ui;
use crate::input::{string_to_buttons, INPUT, Input, Modes};
use crate::project_creator::{OpenSettings, OPEN_SETTINGS};
use std::process::Command;
use std::thread::{self, JoinHandle};


pub const SAVE: &str = "Save";

pub struct ProjectRunner {
    run_handle: Option<JoinHandle<()>>,
    build_handle: Option<JoinHandle<()>>,
    run_queued: bool,
    run_shortcut: Vec<Button>,
    build_shortcut: Vec<Button>,
}

impl ProjectRunner {
    pub fn new() -> Self {
        Self { 
            run_handle: None, 
            build_handle: None,
            run_queued: false,
            run_shortcut: string_to_buttons(" r"),
            build_shortcut: string_to_buttons(" b"),
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

    fn run(&mut self, game_state: &mut GameState) -> bool {
        if self.can_run() == false {
            return false;
        }
        let input = game_state.plugins.get_mut::<Input>(INPUT).unwrap();
        if input.is_pressed(&self.build_shortcut, Modes::Normal) == false {
            return false;
        }

        let input = game_state.plugins.get_mut::<Input>(INPUT).unwrap();
        let open_settings = game_state.plugins.get_mut::<OpenSettings>(OPEN_SETTINGS).unwrap();
        let open_settings_copy = open_settings.clone();
        let run_handle = thread::spawn(move || Self::cargo_run(open_settings_copy));
        self.run_handle = Some(run_handle);
        self.run_queued = false;

        return true;
    }

    fn build(&mut self, game_state: &mut GameState) -> bool {
        if self.can_run() == false {
            return false;
        }

        println!("build");
        
        let open_settings = game_state.plugins.get_mut::<OpenSettings>(OPEN_SETTINGS).unwrap();
        let open_settings_copy = open_settings.clone();
        let build_handle = thread::spawn(move || Self::cargo_build(open_settings_copy));
        self.build_handle = Some(build_handle);

        return true;
    }

    fn can_run(&self) -> bool {
        self.run_handle.is_none() && self.build_handle.is_none()
    }

    fn check_shortcuts(&mut self, game_state: &mut GameState) {
        self.check_run_shortcut(game_state);
        self.check_build_shortcut(game_state);
    }

    fn check_build_shortcut(&mut self, game_state: &mut GameState) {
        let input = game_state.plugins.get_mut::<Input>(INPUT).unwrap();
        if input.is_pressed(&self.build_shortcut, Modes::Normal) == false {
            return;
        }
        self.build(game_state);
    }

    fn check_run_shortcut(&mut self, game_state: &mut GameState) {
        let input = game_state.plugins.get_mut::<Input>(INPUT).unwrap();
        if input.is_pressed(&self.run_shortcut, Modes::Normal) == false {
            return;
        }
        if self.build(game_state) {
            self.run_queued = true;
        }
    }

    fn check_build(&mut self, ui: &mut Ui, game_state: &mut GameState) {
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
    }

    fn check_run(&mut self, ui: &mut Ui, game_state: &mut GameState) {
        if let Some(handle) = &self.run_handle {
            if handle.is_finished() {
                self.run_handle = None;
                game_state.plugins
                    .get_mut::<OpenSettings>(OPEN_SETTINGS)
                    .unwrap()
                    .has_run = true;
            }
        }
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
                if button_ui.button("Run").clicked() {
                    if self.build(game_state) {
                        self.run_queued = true;
                    }
                } else if button_ui.button("Build").clicked() {
                    self.build(game_state);
                } else if button_ui.button("Save").clicked() {
                    game_state.events.add(SAVE, true)
                }
                self.check_shortcuts(game_state);
            });
            self.check_build(ui, game_state);
            self.check_run(ui, game_state);
        });
    }
}

