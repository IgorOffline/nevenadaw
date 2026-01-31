use bevy::prelude::*;
use bevy::window::WindowResolution;

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
        .insert_resource(ClearColor(Color::Srgba(Srgba::hex("#212121").unwrap())))
        .add_systems(Startup, setup)
        .add_systems(Update, move_circle)
        .run();
}

#[derive(Component)]

struct MovingCircle {
    speed: f32,
    amplitude: f32,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Circle::new(20.0))),
        MeshMaterial2d(materials.add(ColorMaterial::from_color(Srgba::hex("#1976D2").unwrap()))),
        MovingCircle {
            speed: 2.0,
            amplitude: 100.0,
        },
        Transform {
            translation: Vec3::new(0.0, 0.0, 0.0),
            ..Default::default()
        },
    ));
}

fn move_circle(time: Res<Time>, mut query: Query<(&mut Transform, &MovingCircle)>) {
    for (mut transform, bouncer) in &mut query {
        transform.translation.y = (time.elapsed_secs() * bouncer.speed).sin() * bouncer.amplitude;
    }
}
