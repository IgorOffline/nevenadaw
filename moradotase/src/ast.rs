use bevy::color::Srgba;
use bevy::prelude::{
    default, App, Camera2d, ClearColor, Commands, PluginGroup, Query, Res, ResMut, Resource,
    Sprite, Startup, Transform, Update, Vec2, Window, WindowPlugin,
};
use bevy::time::{Time, Timer, TimerMode};
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use rand::RngExt;
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
    Game(i32, i32, String, String),
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

const SPAWN_INTERVAL_SECONDS: f32 = 2.0;
const RECTANGLE_COLOR: &str = "#512DA8";
const RECTANGLE_SIZE: Vec2 = Vec2::new(50.0, 50.0);

pub fn game_launch(
    window_width: u32,
    window_height: u32,
    window_title: String,
    window_hex_color: String,
) {
    App::new()
        .insert_resource(ClearColor(Srgba::hex(window_hex_color).unwrap().into()))
        .insert_resource(SpawnTimer(Timer::from_seconds(
            SPAWN_INTERVAL_SECONDS,
            TimerMode::Repeating,
        )))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(window_width, window_height),
                title: window_title,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, game_launch_setup)
        .add_systems(Update, spawn_rectangle_system)
        .run();
}

#[derive(Resource)]
struct SpawnTimer(Timer);

fn spawn_rectangle_system(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<SpawnTimer>,
    window: Query<&Window>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        if let Ok(window) = window.single() {
            let width = window.width();
            let height = window.height();

            let mut rng = rand::rng();
            let x = rng.random_range(-(width / 2.0)..(width / 2.0));
            let y = rng.random_range(-(height / 2.0)..(height / 2.0));

            commands.spawn((
                Sprite {
                    color: Srgba::hex(RECTANGLE_COLOR).unwrap().into(),
                    custom_size: Some(RECTANGLE_SIZE),
                    ..default()
                },
                Transform::from_xyz(x, y, 0.0),
            ));
        }
    }
}

fn game_launch_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
