#[derive(Debug, PartialEq)]
pub enum BosonogaCommand {
    Funak(String, i32),
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
