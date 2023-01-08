
use convert_case::{Case, Casing};

pub trait VirtualType {
    fn get_type_name(&self) -> String;
    fn get_value(&self) -> String;
    fn virtual_clone(&self) -> Box<dyn VirtualType>;
}


#[derive(Clone)]
pub struct VirtualStruct {
    pub name: String,
    pub args: Vec<VirtualValue>,
}

impl VirtualStruct {
    pub fn new(name: String) -> Self {
        Self { name, args: Vec::new() }
    }

    pub fn print(&self) -> String {
        self.print_struct() + &self.print_impl()
    }

    pub fn to_virtual_value(&self) -> VirtualValue {
        VirtualValue { name: self.name.clone(), value: Box::new(Self { name: self.name.clone(), args: Vec::new() }) }
    }

    fn print_struct(&self) -> String {
        let mut string = format!("\npub struct {} {{", &self.name.to_case(Case::Pascal));
        for i in &self.args {
            string += &format!("\n\t{}: {},",
                &i.name.to_case(Case::Snake), 
                &i.value.get_type_name()
            );
        }
        string += "\n}\n";
        println!("struct: \n {}", &string);
        string
    }

    fn print_impl(&self) -> String {
        let mut string = format!("\nimpl {} {{", &self.name.to_case(Case::Pascal));

        string += "\n\tfn new() -> Self {\n\t\tSelf {";
        for i in &self.args {
            string += &format!("\n\t\t\t{}: {},",
                &i.name.to_case(Case::Snake), 
                &i.value.get_value()
            );
        }
        string += "\n\t\t}\n\t}";

        string += "\n}\n";
        println!("impl: \n {}", &string);
        string
    }
}

pub struct VirtualValue {
    pub name: String,
    pub value: Box<dyn VirtualType>
}


impl VirtualType for VirtualStruct {
    fn get_value(&self) -> String {
        self.print_struct()
    }
    fn get_type_name(&self) -> String {
        self.name.clone()
    }
    fn virtual_clone(&self) -> Box<dyn VirtualType> {
        Box::new(Self { name: self.name.clone(), args: Vec::new()} )
    }
}

impl Clone for VirtualValue {
    fn clone(&self) -> Self {
        Self { name: self.name.clone(), value: self.value.virtual_clone() }
    }
}

impl VirtualType for i32 {
    fn get_type_name(&self) -> String {
        "i32".to_string()
    }

    fn get_value(&self) -> String {
        self.to_string()
    }

    fn virtual_clone(&self) -> Box<dyn VirtualType> {
        Box::new(self.clone()) 
    }
}
