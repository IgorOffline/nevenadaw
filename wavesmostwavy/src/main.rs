use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use uuid::Uuid;

#[derive(Resource)]
struct Regina {
    uuid_one: Uuid,
    uuid_two: Uuid,
    movable_vertical_translation_y: f32,
    movable_horizontal_translation_x: f32,
}

#[derive(Component)]
struct MovableVertical {
    speed: f32,
    amplitude: f32,
}

#[derive(Component)]
struct MovableHorizontal {
    speed: f32,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(640, 360),
                title: "Wavesmostwavy".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin::default())
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("#212121").unwrap())))
        .insert_resource(Regina {
            uuid_one: Uuid::new_v4(),
            uuid_two: Uuid::new_v4(),
            movable_vertical_translation_y: 0.0,
            movable_horizontal_translation_x: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (movable_vertical, movable_horizontal))
        .add_systems(EguiPrimaryContextPass, ui_update)
        .run();
}

fn setup(
    regina: Res<Regina>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Srgba::hex("#1976D2").unwrap()))),
        MovableVertical {
            speed: 2.0,
            amplitude: 100.0,
        },
        Transform {
            translation: Vec3::new(-130.0, regina.movable_vertical_translation_y, 0.0),
            ..Default::default()
        },
    ));

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Srgba::hex("#E64A19").unwrap()))),
        MovableHorizontal { speed: 15.0 },
        Transform {
            translation: Vec3::new(-90.0, regina.movable_vertical_translation_y, 0.0),
            ..Default::default()
        },
    ));
}

fn ui_update(regina: Res<Regina>, mut contexts: EguiContexts) {
    let Ok(ctx) = contexts.ctx_mut() else {
        return;
    };
    let hello_string = format!("Hello {}", regina.uuid_one);
    let world_string = format!("World {}", regina.uuid_two);
    egui::Window::new("Hello World").show(ctx, |ui| {
        let direction = match regina.movable_vertical_translation_y {
            y if y > 0.0 => "+",
            y if y < 0.0 => "-",
            _ => "0",
        };
        ui.label(format!("({}) {} {}", direction, hello_string, world_string));
    });
}

fn movable_vertical(
    mut regina: ResMut<Regina>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MovableVertical)>,
) {
    let direction_old = match regina.movable_vertical_translation_y {
        y if y > 0.0 => true,
        y if y < 0.0 => false,
        _ => false,
    };
    for (mut transform, movable) in &mut query {
        regina.movable_vertical_translation_y =
            ops::sin(movable.speed * time.elapsed_secs()) * movable.amplitude;
        transform.translation.y = regina.movable_vertical_translation_y;
    }
    let direction_new = match regina.movable_vertical_translation_y {
        y if y > 0.0 => true,
        y if y < 0.0 => false,
        _ => false,
    };
    if direction_old != direction_new {
        println!("{} --> {}", direction_old, direction_new);
    }
}

fn movable_horizontal(
    mut regina: ResMut<Regina>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &MovableHorizontal)>,
) {
    for (mut transform, movable) in &mut query {
        regina.movable_horizontal_translation_x = movable.speed * time.elapsed_secs();
        transform.translation.x = regina.movable_horizontal_translation_x;
    }
}
