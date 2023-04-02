use jovial_engine::prelude::*;

pub struct Input {
    pub buttons_pressed: Vec<Button>
}

impl Plugin for Input {
    fn update(&mut self) {} 
}

pub struct InputRecorder;


impl Entity for InputRecorder {
    fn update(&mut self, _game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
    } 
}
