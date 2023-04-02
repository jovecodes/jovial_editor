use std::time::{Instant, Duration};

use jovial_engine::prelude::*;

pub const INPUT: &str = "Input";
pub const USE_INPUT: &str = "Use Input";
#[derive(Debug)]
pub struct Input {
    pub buttons_pressed: Vec<Button>,
    pub last_updated: Instant,
    pub shift: bool,
    reset_duration: Duration,
    mode: Modes,
}

impl Plugin for Input {
    fn update(&mut self) {
        if Instant::now().duration_since(self.last_updated) > self.reset_duration && self.mode != Modes::Command {
            self.buttons_pressed.clear();
        }
    }
}

impl Input {
    fn append(&mut self, buttons: Vec<Button>) {
        if buttons.is_empty() {
            return;
        }
        for i in buttons {
            if i == Button::Escape {
                self.mode = Modes::Normal;
            }
            if i == Button::I {
                self.mode = Modes::Insert;
            }
            if i == Button::V {
                self.mode = Modes::Visual;
            }
            if i == Button::Semicolon && self.shift {
                self.mode = Modes::Command;
            }

            if i == Button::LShift || i == Button::RShift {
                self.shift = true;
            }
            self.buttons_pressed.push(i);
        }
        self.last_updated = Instant::now();
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Modes {
    Normal,
    Insert,
    Command,
    Visual,
}

impl Default for Input {
    fn default() -> Self {
        Self { 
            buttons_pressed: Default::default(), 
            last_updated: Instant::now(),
            reset_duration: Duration::new(0, 500_000_000),
            mode: Modes::Normal,
            shift: false
        }
    }
}

pub struct InputRecorder;


impl Entity for InputRecorder {
    fn update(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) { 
        let input = game_state.plugins.get_mut::<Input>(INPUT).unwrap();
        input.append(game_state.input.everything_just_pressed());

        if game_state.input.is_raw_button_just_released(Button::LShift) {
            input.shift = false
        }
        if game_state.input.is_raw_button_just_released(Button::LShift) {
            input.shift = false
        }

        egui::Window::new("Input").show(&game_state.ui.ctx(), |ui| {
            ui.label(&format!("{:#?}", input))  
        });
    } 
}
