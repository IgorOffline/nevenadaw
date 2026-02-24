use crate::bosonoga::BosonogaParser;
use bevy::color::Srgba;
use bevy::prelude::{
    default, App, ButtonInput, Camera2d, ClearColor, Commands, KeyCode, PluginGroup, Query, Res,
    ResMut, Resource, Sprite, Startup, Transform, Update, Vec2, Window, WindowPlugin,
};
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use rand::RngExt;
use std::collections::BTreeSet;
use std::fs;

use super::types::{
    BosonogaCommand, BosonogaElement, BosonogaType, BosonogaValue, BosonogaVariable,
};

const RECTANGLE_COLOR: &str = "#512DA8";
const RECTANGLE_SIZE: Vec2 = Vec2::new(50.0, 50.0);

#[derive(Resource)]
pub struct BosonogaVariables(pub BTreeSet<BosonogaVariable>);

#[derive(Resource)]
pub struct PendingRectSpawns(pub u32);

pub fn game_launch(
    window_width: u32,
    window_height: u32,
    window_title: String,
    window_hex_color: String,
    variables: BTreeSet<BosonogaVariable>,
    pending_rect_spawns: u32,
) {
    App::new()
        .insert_resource(BosonogaVariables(variables))
        .insert_resource(PendingRectSpawns(pending_rect_spawns))
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
        .add_systems(Update, runtime_input_system)
        .add_systems(EguiPrimaryContextPass, ui_main_layout_system)
        .run();
}

fn game_launch_setup(
    mut commands: Commands,
    mut variables: ResMut<BosonogaVariables>,
    pending: Res<PendingRectSpawns>,
    window: Query<&Window>,
) {
    commands.spawn(Camera2d::default());

    if pending.0 == 0 {
        return;
    }

    if let Ok(window) = window.single() {
        let width = window.width();
        let height = window.height();

        for _ in 0..pending.0 {
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

            let rectangle_count_inner = BosonogaVariable::new_i32("rectangle_count", 0);
            let current_count = match variables.0.get(&rectangle_count_inner) {
                Some(var) => match var.value {
                    BosonogaValue::Inat(i) => i,
                    _ => 0,
                },
                None => 0,
            };

            variables.0.replace(BosonogaVariable::new_i32(
                "rectangle_count",
                current_count + 1,
            ));
        }
    }
}

fn spawn_rectangle_system(
    mut commands: Commands,
    mut variables: ResMut<BosonogaVariables>,
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

            let rectangle_count_inner = BosonogaVariable::new_i32("rectangle_count", 0);
            let current_count = match variables.0.get(&rectangle_count_inner) {
                Some(var) => match var.value {
                    BosonogaValue::Inat(i) => i,
                    _ => 0,
                },
                None => 0,
            };

            variables.0.replace(BosonogaVariable::new_i32(
                "rectangle_count",
                current_count + 1,
            ));
        }
    }
}

fn ui_main_layout_system(mut contexts: EguiContexts, variables: Res<BosonogaVariables>) {
    let ctx = contexts.ctx_mut().unwrap();
    let mut style = (*ctx.style()).clone();
    style.visuals.window_fill = style.visuals.window_fill.linear_multiply(0.6);
    egui::Window::new("Bosonoga Variables")
        .frame(egui::Frame::window(&style))
        .show(ctx, |ui| {
            for var in &variables.0 {
                ui.horizontal(|ui| {
                    ui.label(format!("{}:", var.name));
                    match &var.value {
                        BosonogaValue::Bul(b) => ui.label(format!("{}", b)),
                        BosonogaValue::Inat(i) => ui.label(format!("{}", i)),
                    };
                });
            }
        });
}

fn runtime_input_system(
    mut commands: Commands,
    mut variables: ResMut<BosonogaVariables>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    window: Query<&Window>,
) {
    if !keyboard_input.just_pressed(KeyCode::KeyT) {
        return;
    }

    let Ok(input) = fs::read_to_string("runtime-input.bosonoga") else {
        eprintln!("Failed to read runtime-input.bosonoga");
        return;
    };

    let parser = BosonogaParser::new();
    let Ok(commands_vec) = parser.parse(&input) else {
        eprintln!("Failed to parse runtime-input.bosonoga");
        return;
    };

    for element in commands_vec {
        match element {
            BosonogaElement::Command(BosonogaCommand::Set(
                BosonogaType::Inat,
                name,
                BosonogaValue::Inat(v),
            )) => {
                variables.0.replace(BosonogaVariable::new_i32(name, v));
            }
            BosonogaElement::Command(BosonogaCommand::Set(
                BosonogaType::Bul,
                name,
                BosonogaValue::Bul(v),
            )) => {
                variables.0.replace(BosonogaVariable::new_bool(name, v));
            }
            BosonogaElement::Command(BosonogaCommand::Tali) => {
                println!("TALI[{:#?}]", variables.0);
            }
            BosonogaElement::Command(BosonogaCommand::SpawnRectangle) => {
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

                    let rectangle_count_inner = BosonogaVariable::new_i32("rectangle_count", 0);
                    let current_count = match variables.0.get(&rectangle_count_inner) {
                        Some(var) => match var.value {
                            BosonogaValue::Inat(i) => i,
                            _ => 0,
                        },
                        None => 0,
                    };

                    variables.0.replace(BosonogaVariable::new_i32(
                        "rectangle_count",
                        current_count + 1,
                    ));
                }
            }
            _ => {}
        }
    }
}
