use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::hash::{Hash, Hasher};

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum BosonogaValue {
    Bul(bool),
    Inat(i32),
}

#[derive(Debug, PartialEq, Clone, Eq, Hash, PartialOrd, Ord)]
pub enum BosonogaCommand {
    Set(BosonogaType, String, BosonogaValue),
    Tali,
    Game(i32, i32, String, String),
    SpawnRectangle,
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
    pub value: BosonogaValue,
}

impl BosonogaVariable {
    pub fn new_i32(name: impl Into<String>, value: i32) -> Self {
        Self {
            name: name.into(),
            bosonoga_type: BosonogaType::Inat,
            value: BosonogaValue::Inat(value),
        }
    }

    pub fn new_bool(name: impl Into<String>, value: bool) -> Self {
        Self {
            name: name.into(),
            bosonoga_type: BosonogaType::Bul,
            value: BosonogaValue::Bul(value),
        }
    }

    pub fn from_set(t: BosonogaType, n: String, v: BosonogaValue) -> Self {
        match (t, v) {
            (BosonogaType::Bul, BosonogaValue::Bul(b)) => BosonogaVariable::new_bool(n, b),
            (BosonogaType::Bul, BosonogaValue::Inat(i)) => BosonogaVariable::new_bool(n, i != 0),
            (BosonogaType::Inat, BosonogaValue::Bul(b)) => {
                BosonogaVariable::new_i32(n, if b { 1 } else { 0 })
            }
            (BosonogaType::Inat, BosonogaValue::Inat(i)) => BosonogaVariable::new_i32(n, i),
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

pub fn extract_variables(elements: Vec<BosonogaElement>) -> BTreeSet<BosonogaVariable> {
    let mut variables = BTreeSet::new();
    for element in elements {
        if let BosonogaElement::Command(BosonogaCommand::Set(t, n, v)) = element {
            variables.replace(BosonogaVariable::from_set(t, n, v));
        }
    }
    variables
}
