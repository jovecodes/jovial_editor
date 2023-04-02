
pub mod StructCreator {
     
}


fn entity_struct(name: &str) -> String {
    format!(r#"

pub struct {} {{}}

impl Entity for {} {{
    fn start(&mut self, _game_state: &mut GameState, _entity_commands: &mut EntityCommands) {{
        println!("Start");
    }}

    fn update(&mut self, _game_state: &mut GameState, _entity_commands: &mut EntityCommands) {{
        // println!("Update");
    }}

    fn death(&mut self, _game_state: &mut GameState) {{
        // println!("Death");
    }}
}}
    "#, name, name)
}

fn components_enum(struct_name: &str, components: Vec<(String, String)>) -> String {
    let mut components_enum = format!("enum {}Components {{", struct_name);
    for (name, component_type) in components {
        components_enum += &format!("\t{}({})", name, component_type)
    }
    components_enum += "}";
    return components_enum
}
