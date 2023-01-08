extern crate jovial_engine;
use jovial_engine::prelude::*;
use project_manager::ProjectManager;
pub mod writer;
pub mod project_manager;


fn main() {
    let window = create_window(
        "Jovial Editor", 
        "/sprites/Jove.png", 
        1920.,
        1080., 
    );
    let mut app = App::new(&window);
    app.spawn_entity(Fullscreener::new());
    app.spawn_ui(ProjectManager::new());
    app.run(window);
}
