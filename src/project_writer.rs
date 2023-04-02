use jovial_engine::prelude::*;
use std::{fs::{OpenOptions, File}, io::Write, path::{PathBuf, Path}};

use crate::{project_creator::{OpenSettings, OPEN_SETTINGS}, project_settings::ProjectSettings, project_runner::SAVE};

const UNCOMPILED_PROJECT: &str = r#"
use jovial_engine::prelude::*;

fn main() {
    jovial_compile().unwrap();
}
"#;

const DEFUALT_PROJECT: &str = r#"
use jovial_engine::prelude::*;

fn main() {
    jovial_compile().unwrap();
    jovial!(Root, "Root").run()
}

struct Root;
impl Entity for Root {}
"#;

#[derive(Default)]
pub struct ProjectWriter {
    settings: Option<ProjectSettings>
}

impl ProjectWriter {
    pub fn new() -> Self {
        Self::default()
    }
}


impl ProjectWriter {
    fn add_jovial_engine_dependency(&self, open_settings: &OpenSettings) {
        if open_settings.first_edit {
            append_to_file(
                r#"jovial_engine = { git = "https://github.com/jovecodes/JovialEngine" }"#,
                Path::new(&open_settings.path).join("Cargo.toml"),
            )
            .unwrap();
        }
    }

    fn create_main(&self, open_settings: &OpenSettings) {
        if open_settings.first_edit {
            replace_file_text(
                UNCOMPILED_PROJECT, 
                Path::new(&open_settings.path).join("src").join("main.rs")
            )
            .unwrap();
        }
    }

    fn update_main(&self, open_settings: &OpenSettings) {
        replace_file_text(
            &self.settings.as_ref().unwrap().build(), 
            Path::new(&open_settings.path).join("src").join("main.rs")
        )
        .unwrap();
    }

    fn save_settings(&self, open_settings: &OpenSettings) {
        let settings = serde_json::to_string(&self.settings.as_ref().unwrap()).unwrap();
        let mut file = File::create(self.settings_path(open_settings)).unwrap();
        file.write_all(settings.as_bytes());
        self.update_main(open_settings);
    }

    fn settings_path(&self, open_settings: &OpenSettings) -> PathBuf {
        Path::new(&open_settings.path)
            .join("target")
            .join("output")
            .join("project_settings.json")
    }

    fn update_settings(&mut self, open_settings: &OpenSettings) {
        if let Ok(settings_json) = std::fs::read_to_string(self.settings_path(open_settings)) {
            let settings = serde_json::from_str::<ProjectSettings>(&settings_json).unwrap();
            self.settings = Some(settings);
        } else {
            self.settings = Some(ProjectSettings::new());
        }
    }
}

impl Entity for ProjectWriter {
    fn start(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
        if let Some(open_settings) = game_state.plugins.get::<OpenSettings>(OPEN_SETTINGS) {
            if open_settings.first_edit {
                println!("did the stuff");
                self.add_jovial_engine_dependency(open_settings);
                self.create_main(open_settings);
            }

            self.update_settings(open_settings)
        }
    }

    fn update(&mut self, game_state: &mut GameState, _entity_commands: &mut EntityCommands) {
        if let Some(open_settings) = game_state.plugins.get::<OpenSettings>(OPEN_SETTINGS) {
            if open_settings.has_run && open_settings.first_edit {
                self.update_main(open_settings);
            }
            if game_state.events.get::<bool>(SAVE).is_some() {
                self.save_settings(open_settings)
            }
        }
    }
}

pub fn find_and_replace_line(haystack: &str, needle: &str, replace_text: &str) -> String {
    let mut result = String::new();
    for line in haystack.lines() {
        if line.contains(needle) {
            result.push_str(replace_text);
            result.push('\n');
        } else {
            result.push_str(line);
            result.push('\n');
        }
    }
    result
}

pub fn find_and_insert_text(haystack: &str, needle: &str, insert_text: &str) -> String {
    let mut result = String::from(haystack);
    if let Some(position) = result.find(needle) {
        result.insert_str(position, insert_text);
    }
    result
}

fn append_to_file(text: &str, path: PathBuf) -> std::io::Result<()> {
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

fn replace_file_text(text: &str, path: PathBuf) -> std::io::Result<()> {
    let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}
