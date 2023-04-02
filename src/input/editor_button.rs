use jovial_engine::prelude::Button;

use super::button_to_string::{button_to_string, button_to_string_fancy};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditorButton {
    pub button: Button,
    pub shift: bool,
}

impl EditorButton {
    pub fn new(button: Button, shift: bool) -> Self {
        let shift = match button {
            Button::RControl => false,
            Button::RShift => false,
            Button::Return => false,
            Button::Space => false,
            Button::Back => false,
            _ => shift 
        };
        Self { button, shift }
    }
    
    pub fn to_string_fancy(&self) -> String {
        button_to_string_fancy(&self.button, self.shift) 
    }
}

impl ToString for EditorButton {
    fn to_string(&self) -> String {
        button_to_string(&self.button, self.shift) 
    }
}
