use jovial_engine::prelude::Button;


pub fn vec_contains_vec<T: PartialEq>(haystack: &[T], needle: &[T]) -> bool {
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

            ';' => buttons.push(Button::Semicolon),
            ':' => {
                buttons.push(Button::RShift);
                buttons.push(Button::Semicolon);
            },

            '1' => buttons.push(Button::Key1),
            '2' => buttons.push(Button::Key2),
            '3' => buttons.push(Button::Key3),
            '4' => buttons.push(Button::Key4),
            '5' => buttons.push(Button::Key5),
            '6' => buttons.push(Button::Key6),
            '7' => buttons.push(Button::Key7),
            '8' => buttons.push(Button::Key8),
            '9' => buttons.push(Button::Key9),
            '0' => buttons.push(Button::Key0),

            '~' => buttons.push(Button::Return),
            '!' => buttons.push(Button::RControl),

            _ => unreachable!("Unknown character {} ", character)
        }
    }
    dbg!(&buttons);
    return buttons;
}
