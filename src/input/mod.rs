use std::time::{Instant, Duration};
use jovial_engine::prelude::*;
use self::{string_to_buttons::{vec_contains_vec, string_to_buttons}, editor_button::EditorButton};

pub mod string_to_buttons;
pub mod input_ui;
pub mod button_to_string;
pub mod editor_button;

pub const INPUT: &str = "Input";
pub const USE_INPUT: &str = "Use Input";

#[derive(Debug)]
pub struct Input {
    pub buttons_pressed: Vec<EditorButton>,
    pub last_updated: Instant,
    pub shift: bool,
    reset_duration: Duration,
    needs_to_reset: bool,
    mode: Modes,
}

impl Plugin for Input {
    fn update(&mut self) {
        if Instant::now().duration_since(self.last_updated) > self.reset_duration && self.mode != Modes::Command {
            self.buttons_pressed.clear();
        }

        if self.needs_to_reset {
            self.buttons_pressed.clear();
            self.needs_to_reset = false;
        }

        if self.mode == Modes::Command && self.buttons_pressed.contains(&EditorButton::new(Button::Return, false)) {
            self.needs_to_reset = true;
        }
    }
}

impl Input {
    pub fn is_pressed(&mut self, shortcut: &Shortcut) -> bool {
        if self.mode != shortcut.mode {
            return false;
        }
        if vec_contains_vec(&self.buttons_pressed, &shortcut.buttons) {
            self.buttons_pressed.clear();
            if self.mode == Modes::Command {
                self.mode = Modes::Normal;
            }
            true
        } else {
            false
        }
    }

    fn append(&mut self, buttons: Vec<Button>) {
        if buttons.is_empty() {
            return;
        }
        for i in buttons {
            self.update_mode(&i);
            if self.deal_with_backspace(&i) {
                return;
            }

            if i == Button::LShift || i == Button::RShift {
                self.shift = true;
            } else if i == Button::LControl || i == Button::RControl {
                self.buttons_pressed.push(EditorButton::new(Button::RControl, false));
            } else {
                self.buttons_pressed.push(EditorButton::new(i, self.shift));
            }
        }
        self.last_updated = Instant::now();
    }

    fn deal_with_backspace(&mut self, button: &Button) -> bool {
        if button == &Button::Back && self.mode == Modes::Command {
            self.buttons_pressed.pop();
            if self.buttons_pressed.is_empty() {
                self.mode = Modes::Normal;
            }
            return true
        }

        return false;
    }

    fn update_mode(&mut self, button: &Button) -> bool {
        if button == &Button::Escape {
            self.mode = Modes::Normal;
            self.buttons_pressed.clear();
            return true;
        }
        if button == &Button::I && self.mode == Modes::Normal {
            self.mode = Modes::Insert;
            self.buttons_pressed.clear();
            return true;
        }
        if button == &Button::V && self.mode == Modes::Normal {
            self.mode = Modes::Visual;
            self.buttons_pressed.clear();
            return true;
        }
        if button == &Button::Semicolon && self.shift && self.mode != Modes::Command && self.mode != Modes::Insert {
            self.mode = Modes::Command;
            self.buttons_pressed.clear();
            return true;
        }
        return false;
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
            shift: false,
            needs_to_reset: false,
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

pub struct Shortcut {
    mode: Modes,
    buttons: Vec<EditorButton>,
}

impl Shortcut {
    pub fn new(string: &str, mode: Modes) -> Self {
        let mut string = String::from(string);
        if mode == Modes::Command {
            string += "~";
        }
        let buttons = string_to_buttons(&string);

        Self { mode, buttons } 
    }

    pub fn command(string: &str) -> Self {
        Self::new(string, Modes::Command)
    }

    pub fn normal(string: &str) -> Self {
        Self::new(string, Modes::Normal)
    }

    pub fn insert(string: &str) -> Self {
        Self::new(string, Modes::Insert)
    }

    pub fn visual(string: &str) -> Self {
        Self::new(string, Modes::Visual)
    }
}

