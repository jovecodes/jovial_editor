use jovial_engine::prelude::Vector2i;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ProjectSettings {
    pub title: String,
    pub size: Vector2i,
    pub resolution: Vector2i,
    pub debug: bool,
    pub root: (String, String),
    pub compile: bool,
}

impl ProjectSettings {
    pub fn build(&self) -> String {
        let mut main = "fn main() {\n".to_owned();
        main += self.compiled();
        main += &self.jovial();
        main += &self.title();
        main += &self.size();
        main += &self.resolution();
        main += self.debug();

        main += ";\n}";

        return main
    }

    fn compiled(&self) -> &str {
        if self.compile {
            "\tjovial_compile().unwrap();\n"
        } else {
            ""
        }
    }

    fn jovial(&self) -> String {
        format!("\tjovial!({}, {})", self.root.0, self.root.1)
    }

    fn title(&self) -> String {
        format!("\t\t.set_title({})", self.title)
    }

    fn size(&self) -> String {
        format!("\t\t.set_size({})", self.size)
    }

    fn resolution(&self) -> String {
        format!("\t\t.set_resolution({})", self.resolution)
    }

    fn debug(&self) -> &str {
        if self.debug {
            "\t\t.set_debug_mode(true)"
        } else {
            "\t\t.set_debug_mode(false)"
        }
    }
}


