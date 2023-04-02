use jovial_engine::prelude::{*, egui::Ui};

use super::FileCreator;

pub struct FileUi;

pub const FILES: &str = "Files";
pub const NEW_FILE: &str = "New File";

#[derive(Default)]
pub struct Files {
    files: Vec<FileCreator>,
    current_file: Option<usize>,
}

impl Plugin for Files {
    fn update(&mut self) {}
}

impl FileUi {
    fn create_buttons(ui: &mut Ui, files: &mut Files) -> bool {
        if ui.button("New Entity").clicked() {
            files.current_file = Some(files.files.len());
            files.files.push(FileCreator::Entity(super::EntityData::default()));
            return true;
        }
        if ui.button("New Components").clicked() {
            files.current_file = Some(files.files.len());
            files.files.push(FileCreator::Component(super::ComponentData::default()));
            return true;
        }

        return false;
    }
}

impl Entity for FileUi {
    fn update(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
        let files = game_state.plugins.get_mut::<Files>(FILES).unwrap();
        egui::Window::new("File Creator").show(&game_state.ui.ctx(), |ui| {
            if Self::create_buttons(ui, files) {
                game_state.events.add(NEW_FILE, files.current_file.unwrap())
            }
        });
    }
}

