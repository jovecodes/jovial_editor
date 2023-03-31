use jovial_engine::prelude::*;
use project_creator::OPEN_SETTINGS;
use project_creator::ProjectCreator;
use project_creator::OpenSettings;

pub mod project_creator;
pub mod project_runner;
pub mod project_writer;
pub mod project_settings;

pub const ADD_CHILD: &str = "Add child to root";

fn main() {
    jovial_compile().unwrap();
    jovial!(Root, "Root")
        .set_title("Jovial Editor")
        .set_resolution(1280, 720)
        .add_plugin(OpenSettings::default(), OPEN_SETTINGS)
        .run()
}

struct Root;
impl Entity for Root {
    fn start(&mut self, game_state: &mut GameState, entity_commands: &mut EntityCommands) {
        entity_commands.add_child(ProjectCreator::new(), "ProjectCreator", game_state)
    }

    fn update(&mut self, game_state: &mut GameState, entity_commands: &mut EntityCommands) {
        if let Some(child) = game_state.events.take::<Box<dyn Entity>>(ADD_CHILD) {
            let boxed = child.downcast::<Box<dyn Entity>>().unwrap();
            entity_commands.add_boxed_child(
                *boxed,
                "ProjectReader",
                game_state,
            )
        }
    }
}
