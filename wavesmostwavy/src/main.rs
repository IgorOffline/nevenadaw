use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use uuid::Uuid;

#[derive(Resource)]
struct Regina {
    uuid_one: Uuid,
    uuid_two: Uuid,
    translation_y: f32,
}

#[derive(Component)]
struct Movable {
    speed: f32,
    amplitude: f32,
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
            translation_y: 0.0,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, movable)
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
        Movable {
            speed: 2.0,
            amplitude: 100.0,
        },
        Transform {
            translation: Vec3::new(-120.0, regina.translation_y, 0.0),
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
        let direction = match regina.translation_y {
            y if y > 0.0 => "+",
            y if y < 0.0 => "-",
            _ => "0",
        };
        ui.label(format!("({}) {} {}", direction, hello_string, world_string));
    });
}

fn movable(
    mut regina: ResMut<Regina>,
    time: Res<Time>,
    mut query: Query<(&mut Transform, &Movable)>,
) {
    for (mut transform, movable) in &mut query {
        regina.translation_y = ops::sin(time.elapsed_secs() * movable.speed) * movable.amplitude;
        transform.translation.y = regina.translation_y;
    }
}
