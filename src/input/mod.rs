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
    pub fn is_pressed(&mut self, buttons: &Vec<Button>, mode: Modes) -> bool {
        if self.mode != mode {
            return false;
        }
        if vec_contains_vec(&self.buttons_pressed, buttons) {
            self.buttons_pressed.clear();
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
                self.buttons_pressed.push(Button::RShift);
            } else if i == Button::LControl || i == Button::RControl {
                self.buttons_pressed.push(Button::RControl);
            } else {
                self.buttons_pressed.push(i);
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
        if button == &Button::I {
            self.mode = Modes::Insert;
            self.buttons_pressed.clear();
            return true;
        }
        if button == &Button::V {
            self.mode = Modes::Visual;
            self.buttons_pressed.clear();
            return true;
        }
        if button == &Button::Semicolon && self.shift {
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

fn vec_contains_vec<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    for window in haystack.windows(needle.len()) {
        if window == needle {
            return true;
        }
    }
    false
}

pub fn string_to_buttons(string: &str) -> Vec<Button> {
    let mut buttons = Vec::new();
    for character in string.chars() {
        match character {
            'a' => buttons.push(Button::A),
            'A' => {
                buttons.push(Button::RShift);
                buttons.push(Button::A);
            },
            'b' => buttons.push(Button::B),
            'B' => {
                buttons.push(Button::RShift);
                buttons.push(Button::B);
            },
            'c' => buttons.push(Button::C),
            'C' => {
                buttons.push(Button::RShift);
                buttons.push(Button::C);
            },
            'd' => buttons.push(Button::D),
            'D' => {
                buttons.push(Button::RShift);
                buttons.push(Button::D);
            },
            'e' => buttons.push(Button::E),
            'E' => {
                buttons.push(Button::RShift);
                buttons.push(Button::E);
            },
            'f' => buttons.push(Button::F),
            'F' => {
                buttons.push(Button::RShift);
                buttons.push(Button::F);
            },
            'g' => buttons.push(Button::G),
            'G' => {
                buttons.push(Button::RShift);
                buttons.push(Button::G);
            },
            'h' => buttons.push(Button::H),
            'H' => {
                buttons.push(Button::RShift);
                buttons.push(Button::H);
            },
            'i' => buttons.push(Button::I),
            'I' => {
                buttons.push(Button::RShift);
                buttons.push(Button::I);
            },
            'j' => buttons.push(Button::J),
            'J' => {
                buttons.push(Button::RShift);
                buttons.push(Button::J);
            },
            'k' => buttons.push(Button::K),
            'K' => {
                buttons.push(Button::RShift);
                buttons.push(Button::K);
            },
            'l' => buttons.push(Button::L),
            'L' => {
                buttons.push(Button::RShift);
                buttons.push(Button::L);
            },
            'm' => buttons.push(Button::M),
            'M' => {
                buttons.push(Button::RShift);
                buttons.push(Button::M);
            },
            'n' => buttons.push(Button::N),
            'N' => {
                buttons.push(Button::RShift);
                buttons.push(Button::N);
            },
            'o' => buttons.push(Button::O),
            'O' => {
                buttons.push(Button::RShift);
                buttons.push(Button::O);
            },
            'p' => buttons.push(Button::P),
            'P' => {
                buttons.push(Button::RShift);
                buttons.push(Button::P);
            },
            'q' => buttons.push(Button::Q),
            'Q' => {
                buttons.push(Button::RShift);
                buttons.push(Button::Q);
            },
            'r' => buttons.push(Button::R),
            'R' => {
                buttons.push(Button::RShift);
                buttons.push(Button::R);
            },
            's' => buttons.push(Button::S),
            'S' => {
                buttons.push(Button::RShift);
                buttons.push(Button::S);
            },
            't' => buttons.push(Button::T),
            'T' => {
                buttons.push(Button::RShift);
                buttons.push(Button::T);
            },
            'u' => buttons.push(Button::U),
            'U' => {
                buttons.push(Button::RShift);
                buttons.push(Button::U);
            },
            'v' => buttons.push(Button::V),
            'V' => {
                buttons.push(Button::RShift);
                buttons.push(Button::V);
            },
            'w' => buttons.push(Button::W),
            'W' => {
                buttons.push(Button::RShift);
                buttons.push(Button::W);
            },
            'x' => buttons.push(Button::X),
            'X' => {
                buttons.push(Button::RShift);
                buttons.push(Button::X);
            },
            'y' => buttons.push(Button::Y),
            'Y' => {
                buttons.push(Button::RShift);
                buttons.push(Button::Y);
            },
            'z' => buttons.push(Button::Z),
            'Z' => {
                buttons.push(Button::RShift);
                buttons.push(Button::Z);
            },
            ' ' => buttons.push(Button::Space),

            _ => unreachable!("Unknown character {} ", character)
        }
    }
    dbg!(&buttons);
    return buttons;
}
