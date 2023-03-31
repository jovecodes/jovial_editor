use jovial_engine::prelude::*;
use std::fs::OpenOptions;

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

pub struct ProjectWriter {

}


impl ProjectWriter {
    // fn add_jovial_engine_dependency(&self) {
    //     if self.first_edit {
    //         append_to_file(
    //             r#"jovial_engine = { git = "https://github.com/jovecodes/JovialEngine" }"#,
    //             self.path.join("Cargo.toml"),
    //         )
    //         .unwrap();
    //     }
    // }
    //
    // fn create_main(&self) {
    //     if self.first_edit {
    //         replace_file_text(UNCOMPILED_PROJECT, self.path.join("src").join("main.rs")).unwrap();
    //     }
    // }
    //
    // fn update_main(&self) {
    //     if self.first_edit {
    //     replace_file_text(DEFUALT_PROJECT, self.path.join("src").join("main.rs")).unwrap();
    //     }
    // }
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

// fn append_to_file(text: &str, path: PathBuf) -> std::io::Result<()> {
//     let mut file = OpenOptions::new().create(true).append(true).open(path)?;
//     file.write_all(text.as_bytes())?;
//     Ok(())
// }
//
// fn replace_file_text(text: &str, path: PathBuf) -> std::io::Result<()> {
//     let mut file = OpenOptions::new().write(true).truncate(true).open(path)?;
//     file.write_all(text.as_bytes())?;
//     Ok(())
// }
