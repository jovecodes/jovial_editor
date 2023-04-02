use jovial_engine::prelude::Button;



pub fn button_to_string_fancy(button: &Button, shift: bool) -> String {
    if shift {
        match button {
            Button::Semicolon => return ":".to_owned(),
            Button::RBracket => return "}".to_owned(),
            Button::LBracket => return "{".to_owned(),
            Button::Minus => return "_".to_owned(),
            Button::Equals => return "+".to_owned(),
            _ => (), 
        }
    }

    match button {
        Button::RControl => String::from("<Ctrl>"),
        Button::LControl => String::from("<Ctrl>"),
        Button::Space => String::from(" "),
        Button::Semicolon => String::from(";"),
        Button::RBracket => "]".to_owned(),
        Button::LBracket => "[".to_owned(),
        Button::LShift => "".to_owned(),
        Button::RShift => "".to_owned(),
        Button::Return  => "".to_owned(),
        Button::Key1 => "1".to_owned(),
        Button::Key2 => "2".to_owned(),
        Button::Key3 => "3".to_owned(),
        Button::Key4 => "4".to_owned(),
        Button::Key5 => "5".to_owned(),
        Button::Key6 => "6".to_owned(),
        Button::Key7 => "7".to_owned(),
        Button::Key8 => "8".to_owned(),
        Button::Key9 => "9".to_owned(),
        Button::Key0 => "0".to_owned(),
        Button::Minus => "-".to_owned(),
        Button::Equals => "=".to_owned(),
        Button::Back => "".to_owned(),
        _ => {
            let raw = format!("{button:?}");
            if !shift {
                raw.to_lowercase()
            } else {
                raw
            }
        }
    }
}

pub fn button_to_string(button: &Button, shift: bool) -> String {
    match button {
        Button::RControl => String::from("<Ctrl>"),
        Button::LControl => String::from("<Ctrl>"),
        Button::Space => String::from("<Space>"),
        _ => format!("{button:?}") 
    }
}
