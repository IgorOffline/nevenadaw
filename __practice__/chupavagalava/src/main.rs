use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub const DARK_COLOR: Color = Color::srgb_u8(33, 33, 33);
pub const GREEN_COLOR: Color = Color::srgb_u8(139, 195, 74);

#[derive(Component)]
struct MovingObject;

#[derive(Component)]
struct Speed(f32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "chupavagalava 0.1.0".into(),
                    resolution: (1280, 720).into(),
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin::default(),
        ))
        .insert_resource(ClearColor(DARK_COLOR))
        .add_systems(Startup, setup)
        .add_systems(Update, chu_update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(GREEN_COLOR)),
        Transform::default().with_scale(Vec3::splat(128.)),
        MovingObject,
        Speed(100.0),
    ));
}

fn chu_update(
    time: Res<Time<Real>>,
    mut query: Query<(&mut Transform, &Speed), With<MovingObject>>,
) {
    let delta = 12.0 * time.delta_secs();

    for (mut transform, speed) in &mut query {
        let distance_to_move = speed.0 * delta;

        transform.translation.x += distance_to_move;

        if transform.translation.x > 920.0 {
            transform.translation.x = -920.0;
        }
    }
}
