#[derive(Debug, PartialEq)]
pub enum BosonogaCommand {
    Funak(String),
    Tali(String),
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
