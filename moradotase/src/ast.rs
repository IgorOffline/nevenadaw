use bevy::color::Srgba;
use bevy::prelude::{
    default, App, ButtonInput, Camera2d, ClearColor, Commands, Entity, KeyCode, PluginGroup, Query,
    Res, Sprite, Startup, Transform, Update, Vec2, Window, WindowPlugin, With,
};
use bevy::window::{PrimaryWindow, WindowResolution};
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
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
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(window_width, window_height),
                title: window_title,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .add_systems(Startup, game_launch_setup)
        .add_systems(Update, spawn_rectangle_system)
        .add_systems(EguiPrimaryContextPass, ui_main_layout_system)
        .run();
}

fn game_launch_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn spawn_rectangle_system(
    mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Query<&Window>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
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

fn ui_main_layout_system(
    mut contexts: EguiContexts,
    window_query: Query<(Entity, &Window), With<PrimaryWindow>>,
) {
    let Ok((window_entity, _window)) = window_query.single() else {
        return;
    };

    let ctx = match contexts.ctx_for_entity_mut(window_entity) {
        Ok(ctx) => ctx,
        Err(_) => return,
    };

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("Lorem UI Ipsum");
    });
}
