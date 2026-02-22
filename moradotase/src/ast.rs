use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum BosonogaCommand {
    Set(BosonogaType, String, i32),
    Tali,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum BosonogaType {
    Bul,
    Inat,
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum BosonogaElement {
    Command(BosonogaCommand),
    Type(BosonogaType),
}

#[derive(Debug, Clone)]
pub struct BosonogaVariable {
    pub name: String,
    pub bosonoga_type: BosonogaType,
    pub value: i32,
}

impl BosonogaVariable {
    pub fn new_i32(name: impl Into<String>, value: i32) -> Self {
        Self {
            name: name.into(),
            bosonoga_type: BosonogaType::Inat,
            value,
        }
    }

    pub fn new_bool(name: impl Into<String>, value: bool) -> Self {
        Self {
            name: name.into(),
            bosonoga_type: BosonogaType::Bul,
            value: if value { 1 } else { 0 },
        }
    }
}

impl PartialEq for BosonogaVariable {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for BosonogaVariable {}

impl Hash for BosonogaVariable {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl PartialOrd for BosonogaVariable {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for BosonogaVariable {
    fn cmp(&self, other: &Self) -> Ordering {
        self.name.cmp(&other.name)
    }
}
