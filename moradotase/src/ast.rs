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
