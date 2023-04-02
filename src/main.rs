use file_creation::file_ui::FILES;
use file_creation::file_ui::Files;
use jovial_engine::prelude::*;
use project_creator::OPEN_SETTINGS;
use project_creator::ProjectCreator;
use project_creator::OpenSettings;

pub mod project_creator;
pub mod project_runner;
pub mod project_writer;
pub mod project_settings;
pub mod component_creation;
pub mod file_creation;
pub mod input;

pub const ADD_CHILD: &str = "Add child to root";

fn main() {
    jovial_compile().unwrap();
    jovial!(Root, "Root")
        .set_title("Jovial Editor")
        .set_resolution(1280, 720)
        .add_plugin(OpenSettings::default(), OPEN_SETTINGS)
        .add_plugin(Files::default(), FILES)
        .run()
}

struct Root;
impl Entity for Root {
    fn start(&mut self, game_state: &mut GameState, entity_commands: &mut EntityCommands) {
        entity_commands.add_child(ProjectCreator::new(), "ProjectCreator", game_state)
    }

    fn update(&mut self, game_state: &mut GameState, entity_commands: &mut EntityCommands) {
        if let Some(child) = game_state.events.take(ADD_CHILD) {
            let children = child.downcast::<Vec<Box<dyn Entity>>>().unwrap();
            for child in *children {
                entity_commands.add_boxed_child(
                    child,
                    "ProjectReader",
                    game_state,
                )
            }
        }
    }
}

pub fn add_child_to_root<T: Entity + 'static>(game_state: &mut GameState, entity: T) {
    if let Some(event) = game_state.events.take_new(ADD_CHILD) {
        let mut children_vec = event.downcast::<Vec<Box<dyn Entity>>>().unwrap();
        children_vec.push(Box::new(entity));
        game_state.events.add::<Vec<Box<dyn Entity>>>(ADD_CHILD, *children_vec)
    } else {
        game_state.events.add::<Vec<Box<dyn Entity>>>(
            ADD_CHILD,
            vec![Box::new(entity)],
        );
    }
}
