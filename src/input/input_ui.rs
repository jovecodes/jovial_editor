use jovial_engine::prelude::{*, egui::Ui};
use super::{Input, INPUT};

pub struct InputUi;

impl InputUi {
    fn command_area(ui: &mut Ui, game_state: &mut GameState) {
        let mut text = String::new();
        let input = game_state.plugins.get::<Input>(INPUT).unwrap();
        for i in &input.buttons_pressed {
            text += &i.to_string_fancy();
        }
        ui.label(&text);
    }
}

impl Entity for InputUi {
    fn update(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
        egui::Area::new("Command Area")
            .fixed_pos((5.0, 560.0))
            .show(&game_state.ui.ctx(), |ui| { Self::command_area(ui, game_state) });
    } 
}
