use std::collections::HashMap;

use project_builder::{delete_folder, create_project, write_to_program, run_program};
use virtual_struct::VirtualStruct;

use self::project_builder::Project;

pub mod virtual_struct;
pub mod project_builder;

pub struct MainFn {
    text: Vec<String>
}

impl MainFn {
    pub fn new() -> Self {
        Self { text: vec!["fn main() {\n".to_string()] }
    }

    pub fn add(&mut self, text: String) {
        self.text.push(text);
    }

    pub fn print(&self) -> String {
        let mut main = String::new();
        for i in &self.text {
            main += i;
        }
        main + "}\n"
    }
}

pub struct Writer {
    pub functions: Vec<VirtualFunction>,
    pub structs: HashMap<String, VirtualStruct>,
    project: Project,
    main: MainFn,
}

impl Writer {
    pub fn new(project_name: String) -> Self {
        Self {
            functions: Vec::new(),
            structs: HashMap::new(),
            project: create_project(project_name),
            main: MainFn::new()
        }
    }

    pub fn delete(&self) {
        delete_folder(&self.project.path)
    }

    pub fn run(&self) {
        run_program(&self.project);
    }

    pub fn save(&self) {
        write_to_program(self.get_file_contents(), &self.project.path)
    }

    pub fn add_struct(&mut self, name: String) {
        self.structs.insert(name.clone(), VirtualStruct::new(name));
    }

    fn get_file_contents(&self) -> String {
        let mut contents = String::new();
        contents += &self.get_structs();
        contents += &self.main.print();
        contents += &self.get_functions();
        contents
    }

    fn get_structs(&self) -> String {
        let mut structs = String::new();
        for i in self.structs.values() {
            structs += &i.print()
        }
        structs
    }

    fn get_functions(&self) -> String {
        let mut functions = String::new();
        for i in &self.functions {
            functions += &i.print();
        }
        functions
    }
}

pub struct VirtualFunction(pub String);

impl VirtualFunction {
    pub fn print(&self) -> String {
        self.0.clone()
    }
}
