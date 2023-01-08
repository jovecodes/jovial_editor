use jovial_engine::prelude::*;
use crate::writer::Writer;
use crate::writer::virtual_struct::VirtualStruct;
use ggez_egui;
use ggez_egui::EguiContext;

pub struct ProjectManager {
    name: String,
    project: Option<Writer>,
    struct_manager: StructManager
}

impl ProjectManager {
    pub fn new() -> Self {
        Self { 
            name: String::from("Project Name"), 
            project: None,
            struct_manager: StructManager::new(),
        }
    }

    fn edit_structs(&mut self, ctx: &mut GameState, gui: &EguiContext) {
        if self.project.is_some() {
            self.struct_manager.display(ctx, gui, self.project.as_mut().unwrap());
            if ctx.event_manager.has_event("add argument") {
                println!("add a thing to a thing");
                ctx.event_manager.use_event("add argument");
            }
        }
    }

    fn show_project_creator(&mut self, gui: &EguiContext) {
        let window = ggez_egui::egui::Window::new("Project Manager");
        window.show(&gui, |ui| {
            ui.horizontal( |ui| {
                ui.add(ggez_egui::egui::TextEdit::singleline(&mut self.name));
                if self.name != String::new() {
                    if ui.button("Create Project").clicked() {
                        self.project = Some(Writer::new(self.name.clone()));
                    }
                }
            });
        });
    }

    fn show_project_buttons(&mut self, gui: &EguiContext) {
        let window = ggez_egui::egui::Window::new("Project Manager").title_bar(false);
        window.show(&gui, |ui| {
            ui.horizontal( |ui| {
                if ui.button("save").clicked() {
                    self.project.as_ref().unwrap().save();
                }
                if ui.button("run").clicked() {
                    self.project.as_ref().unwrap().run();
                }
                if ui.button("delete project").clicked() {
                    self.project.as_ref().unwrap().delete();
                    self.project = None;
                }
            });
        });
    }

}

impl UIElement for ProjectManager {
    fn update(&mut self, ctx: &mut GameState, gui: &EguiContext) {
        if self.project.is_none() {
            self.show_project_creator(gui);
        } else {
            self.show_project_buttons(gui);
            self.edit_structs(ctx, gui);
            if ctx.is_raw_button_just_pressed(KeyCode::S) && ctx.is_raw_button_pressed(KeyCode::Shift){
                self.project.as_ref().unwrap().save();
            }
            if ctx.is_raw_button_just_pressed(KeyCode::R) && ctx.is_raw_button_pressed(KeyCode::Shift){
                self.project.as_ref().unwrap().run();
            }
            if ctx.is_raw_button_just_pressed(KeyCode::D) && ctx.is_raw_button_pressed(KeyCode::Shift){
                self.project.as_ref().unwrap().delete();
                self.project = None;
            }
        }
    }
}


pub struct StructManager {
    name: String,
    arg: String
}

impl StructManager {
    pub fn new() -> Self {
        Self { name: String::new(), arg: String::new() }
    }

    pub fn display(&mut self, ctx: &mut GameState, gui: &EguiContext, project: &mut Writer) {
        let window = ggez_egui::egui::Window::new("structures").min_width(250.);
        window.show(&gui, |ui| {
            ui.horizontal(|sub_ui| {
                sub_ui.add(ggez_egui::egui::TextEdit::singleline(&mut self.name));
                if self.name != String::new() {
                    if sub_ui.button("Create struct").clicked() {
                        project.add_struct(self.name.clone());
                        self.name = String::new();
                    }
                }
            });
            ui.separator();
            ggez_egui::egui::CollapsingHeader::new("Structs").show(ui, |ui| {
                let names = get_names_of_project(&project);
                for i in &names {
                    ggez_egui::egui::CollapsingHeader::new(i).show(ui, |ui| {
                        self.create_args(ui, project, ctx, i.to_owned());
                        for arg in &project.structs.get(i).unwrap().args {
                            ui.label(&arg.name);
                        }
                    });
                }
            });
        });
    }

    fn create_args(&mut self, ui: &mut ggez_egui::egui::Ui, project: &Writer, ctx: &mut GameState, target: String) {
        let mut arg = String::from("not changed");
        ggez_egui::egui::ComboBox::from_label("Add argument").width(200.)
            .selected_text(&arg)
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut arg, "test".to_owned(), "test".to_owned());
                for (name, _virtual_struct) in &project.structs {
                    ui.selectable_value(&mut arg, name.clone(), name.clone());
                }
            });
        if arg == String::from("not changed") {
            return
        }
        println!("{}", arg);
    }
}

fn get_names_of_project(project: &Writer) -> Vec<String> {
    let mut names = Vec::new();
    for i in project.structs.keys() {
        names.push(i.clone());
    }
    names
}
