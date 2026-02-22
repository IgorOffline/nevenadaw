#[derive(Debug, PartialEq)]
pub enum BosonogaCommand {
    Set(String, i32),
    Tali,
}

#[derive(Debug, PartialEq)]
pub enum BosonogaType {
    Bul,
    Inat,
}

#[derive(Debug, PartialEq)]
pub enum BosonogaElement {
    Command(BosonogaCommand),
    Type(BosonogaType),
}

#[derive(Debug, PartialEq)]
pub struct BosonogaVariable {
    pub name: String,
    pub bosonoga_type: BosonogaType,
    pub value: i32,
}

pub fn new_variable_i32(name: String, value: i32) -> BosonogaVariable {
    let bosonoga_type = BosonogaType::Inat;
    BosonogaVariable {
        name,
        bosonoga_type,
        value,
    }
}
