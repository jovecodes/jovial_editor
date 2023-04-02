use std::{fs::File, path::Path};
use jovial_engine::prelude::GameState;
use crate::project_creator::{OpenSettings, OPEN_SETTINGS};

pub mod file_ui;

pub enum FileCreator {
    Component(ComponentData),
    Entity(EntityData),
}

impl FileCreator {
    pub fn new_entity(name: &str) -> Self {
        let entity_data = EntityData {
            name: name.to_owned(),
            ..Default::default()
        };

        Self::Entity(entity_data)
    }

    pub fn new_component(name: &str) -> Self {
        let component_data = ComponentData {
            name: name.to_owned(),
            ..Default::default()
        };
        
        Self::Component(component_data)
    }

    pub fn create(&self, game_state: &mut GameState) {
        let open_settings = game_state.plugins.get::<OpenSettings>(OPEN_SETTINGS).unwrap();
        match self {
            FileCreator::Component(c) => Self::create_component(c, open_settings),
            FileCreator::Entity(e) => Self::create_entity(e, open_settings),
        } 
    }

    fn create_component(component_data: &ComponentData, open_settings: &OpenSettings) {
        let component_path = Path::new(&open_settings.path)
            .join("src")
            .join("components");
        
        if component_path.exists() == false {
            std::fs::create_dir(component_path.clone()).unwrap();
        }

        let file = File::create(component_path.join(&component_data.name));
    }

    fn create_entity(entity_data: &EntityData, open_settings: &OpenSettings) {
        let entity_path = Path::new(&open_settings.path)
            .join("src")
            .join("entities");

        if entity_path.exists() == false {
            std::fs::create_dir(entity_path.clone()).unwrap()
        }

        let file = File::create(entity_path.join(&entity_data.name));
    }
}

#[derive(Default)]
pub struct ComponentData {
    name: String
}

#[derive(Default)]
pub struct EntityData {
    name: String,
    components: Vec<(String, String)>
}
