use crate::bosonoga::BosonogaParser;
use bevy::color::Srgba;
use bevy::prelude::{
    default, App, ButtonInput, Camera2d, ClearColor, Click, Commands, Component, Entity,
    EntityEvent, KeyCode, On, Pickable, PluginGroup, Pointer, Query, Res, ResMut, Resource, Sprite,
    Startup, Transform, Update, Vec2, Window, WindowPlugin, With,
};
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use std::collections::BTreeSet;
use std::fs;

use super::types::{
    BosonogaCommand, BosonogaElement, BosonogaType, BosonogaValue, BosonogaVariable,
};

const RECTANGLE_COLOR: &str = "#512DA8";
const RECTANGLE_SIZE: Vec2 = Vec2::new(50.0, 50.0);

#[derive(Resource)]
pub struct BosonogaVariables(pub BTreeSet<BosonogaVariable>);

#[derive(Resource, Default)]
struct RectangleCounter(i32);

#[derive(Component)]
struct RectangleId(i32);

pub fn game_launch(
    window_width: u32,
    window_height: u32,
    window_title: String,
    window_hex_color: String,
    variables: BTreeSet<BosonogaVariable>,
) {
    App::new()
        .insert_resource(BosonogaVariables(variables))
        .insert_resource(RectangleCounter::default())
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
        .add_systems(Update, runtime_input_system)
        .add_systems(EguiPrimaryContextPass, ui_main_layout_system)
        .run();
}

fn game_launch_setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
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
    mut rectangle_counter: ResMut<RectangleCounter>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    _window: Query<&Window>,
    rectangles: Query<Entity, With<RectangleId>>,
) {
    let mut current_count = rectangles.iter().count() as i32;

    if keyboard_input.just_pressed(KeyCode::Digit0) {
        for entity in &rectangles {
            commands.entity(entity).despawn();
        }
        current_count = 0;
        variables
            .0
            .replace(BosonogaVariable::new_i32("rectangle_count", 0));
    }

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
                if name == "rectangle_count" {
                    current_count = v;
                }
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
            BosonogaElement::Command(BosonogaCommand::SpawnRectangle(x_i, y_i)) => {
                rectangle_counter.0 += 1;
                let id = rectangle_counter.0;
                current_count += 1;

                let x = x_i as f32;
                let y = y_i as f32;

                commands
                    .spawn((
                        Sprite {
                            color: Srgba::hex(RECTANGLE_COLOR).unwrap().into(),
                            custom_size: Some(RECTANGLE_SIZE),
                            ..default()
                        },
                        Transform::from_xyz(x, y, 0.0),
                        Pickable::default(),
                        RectangleId(id),
                    ))
                    .observe(|ev: On<Pointer<Click>>, query: Query<&RectangleId>| {
                        if let Ok(rectangle_id) = query.get(ev.event_target()) {
                            println!("Rectangle picked at: {}", rectangle_id.0);
                        }
                    });

                variables
                    .0
                    .replace(BosonogaVariable::new_i32("rectangle_count", current_count));
            }
            BosonogaElement::Command(BosonogaCommand::SpawnRectangles(
                count_x,
                count_y,
                x_start,
                y_start,
            )) => {
                for i in 0..count_x {
                    for j in 0..count_y {
                        rectangle_counter.0 += 1;
                        let id = rectangle_counter.0;
                        current_count += 1;

                        let x = (x_start + i * 100) as f32;
                        let y = (y_start + j * 100) as f32;

                        commands
                            .spawn((
                                Sprite {
                                    color: Srgba::hex(RECTANGLE_COLOR).unwrap().into(),
                                    custom_size: Some(RECTANGLE_SIZE),
                                    ..default()
                                },
                                Transform::from_xyz(x, y, 0.0),
                                Pickable::default(),
                                RectangleId(id),
                            ))
                            .observe(|ev: On<Pointer<Click>>, query: Query<&RectangleId>| {
                                if let Ok(rectangle_id) = query.get(ev.event_target()) {
                                    println!("Rectangle picked at: {}", rectangle_id.0);
                                }
                            });
                    }
                }

                variables
                    .0
                    .replace(BosonogaVariable::new_i32("rectangle_count", current_count));
            }
            _ => {}
        }
    }
}
