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
    pub fn new() -> Self {
        Self { 
            title: String::from("Jovial Game"), 
            size: Vector2i::new(720, 1280), 
            resolution: Vector2i::new(360, 640), 
            debug: true, 
            root: (String::from("Root"), String::from("Root")), 
            compile: true,
        }
    }

    pub fn build(&self) -> String {
        let mut main = "use jovial_engine::prelude::*;\n\n".to_owned();
        main += "fn main() {\n";
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
            "\tjovial_compile().unwrap();"
        } else {
            ""
        }
    }

    fn jovial(&self) -> String {
        format!("\n\tjovial!({}, {:?})", self.root.0, self.root.1)
    }

    fn title(&self) -> String {
        format!("\n\t\t.set_title({:?})", self.title)
    }

    fn size(&self) -> String {
        format!("\n\t\t.set_size({}, {})", self.size.x, self.size.y)
    }

    fn resolution(&self) -> String {
        format!("\n\t\t.set_resolution({}, {})", self.resolution.x, self.resolution.y)
    }

    fn debug(&self) -> &str {
        if self.debug {
            "\n\t\t.set_debug_mode(true)"
        } else {
            "\n\t\t.set_debug_mode(false)"
        }
    }
}


