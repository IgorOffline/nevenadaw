use bevy::color::Srgba;
use bevy::prelude::{
    default, App, Camera2d, ClearColor, Commands, PluginGroup, Sprite, Startup, Transform, Vec2,
    Window, WindowPlugin,
};
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use std::cmp::Ordering;
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
    Game(i32, i32),
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

pub fn game_launch(window_width: u32, window_height: u32) {
    App::new()
        .insert_resource(ClearColor(Srgba::hex("#607D8B").unwrap().into()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(window_width, window_height),
                title: "Bosonoga".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            color: Srgba::hex("#512DA8").unwrap().into(),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::default(),
    ));
}
