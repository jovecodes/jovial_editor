use jovial_engine::prelude::Button;

use super::editor_button::EditorButton;


pub fn vec_contains_vec<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
    for window in haystack.windows(needle.len()) {
        if window == needle {
            return true;
        }
    }
    false
}

pub fn string_to_buttons(string: &str) -> Vec<EditorButton> {
    let mut buttons = Vec::new();
    for character in string.chars() {
        let shift = character.is_uppercase();
        let mut new_char = character;
        if character.is_alphabetic() {
            new_char = character.to_ascii_lowercase();
        }

        match new_char {
            'a' => buttons.push(EditorButton::new(Button::A, shift)),
            'b' => buttons.push(EditorButton::new(Button::B, shift)),
            'c' => buttons.push(EditorButton::new(Button::C, shift)),
            'd' => buttons.push(EditorButton::new(Button::D, shift)),
            'e' => buttons.push(EditorButton::new(Button::E, shift)),
            'f' => buttons.push(EditorButton::new(Button::F, shift)),
            'g' => buttons.push(EditorButton::new(Button::G, shift)),
            'h' => buttons.push(EditorButton::new(Button::H, shift)),
            'i' => buttons.push(EditorButton::new(Button::I, shift)),
            'j' => buttons.push(EditorButton::new(Button::J, shift)),
            'k' => buttons.push(EditorButton::new(Button::K, shift)),
            'l' => buttons.push(EditorButton::new(Button::L, shift)),
            'm' => buttons.push(EditorButton::new(Button::M, shift)),
            'n' => buttons.push(EditorButton::new(Button::N, shift)),
            'o' => buttons.push(EditorButton::new(Button::O, shift)),
            'p' => buttons.push(EditorButton::new(Button::P, shift)),
            'q' => buttons.push(EditorButton::new(Button::Q, shift)),
            'r' => buttons.push(EditorButton::new(Button::R, shift)),
            's' => buttons.push(EditorButton::new(Button::S, shift)),
            't' => buttons.push(EditorButton::new(Button::T, shift)),
            'u' => buttons.push(EditorButton::new(Button::U, shift)),
            'v' => buttons.push(EditorButton::new(Button::V, shift)),
            'w' => buttons.push(EditorButton::new(Button::W, shift)),
            'x' => buttons.push(EditorButton::new(Button::X, shift)),
            'y' => buttons.push(EditorButton::new(Button::Y, shift)),
            'z' => buttons.push(EditorButton::new(Button::Z, shift)),

            ' ' => buttons.push(EditorButton::new(Button::Space, shift)),
            ';' => buttons.push(EditorButton::new(Button::Semicolon, shift)),
            ':' => buttons.push(EditorButton::new(Button::Semicolon, true)),

            '1' => buttons.push(EditorButton::new(Button::Key1, shift)),
            '2' => buttons.push(EditorButton::new(Button::Key2, shift)),
            '3' => buttons.push(EditorButton::new(Button::Key3, shift)),
            '4' => buttons.push(EditorButton::new(Button::Key4, shift)),
            '5' => buttons.push(EditorButton::new(Button::Key5, shift)),
            '6' => buttons.push(EditorButton::new(Button::Key6, shift)),
            '7' => buttons.push(EditorButton::new(Button::Key7, shift)),
            '8' => buttons.push(EditorButton::new(Button::Key8, shift)),
            '9' => buttons.push(EditorButton::new(Button::Key9, shift)),
            '0' => buttons.push(EditorButton::new(Button::Key0, shift)),

            '~' => buttons.push(EditorButton::new(Button::Return, shift)),
            '!' => buttons.push(EditorButton::new(Button::RControl, shift)),

            _ => unreachable!("Unknown character {} ", character)
        }
    }
    dbg!(&buttons);
    return buttons;
}
