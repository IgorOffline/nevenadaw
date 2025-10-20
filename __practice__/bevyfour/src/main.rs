use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1280, 720),
                title: "Bevyrectangle".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        //.add_systems(Update, (move_rectangle, despawn_and_respawn_rectangle))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    spawn_rectangles(&mut commands, &asset_server);
}

fn spawn_rectangles(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    for i in 0..12 {
        let mut transform: Transform = Transform::from_xyz(-225.0 + 65.0 * i as f32, 0.0, 0.0);
        transform = transform.with_scale(Vec3::new(0.5, 0.5, 1.0));
        commands.spawn((
            Sprite::from_image(asset_server.load("rectangle_03A9F4_100w100h.png")),
            Transform::from(transform),
            Movable {
                direction: random::<bool>(),
                speed: 0.0,
            },
        ));
    }
}

#[derive(Component)]
struct Movable {
    direction: bool,
    speed: f32,
}

fn _move_rectangle(time: Res<Time>, mut query: Query<(&mut Transform, &Movable)>) {
    for (mut transform, mover) in query.iter_mut() {
        let mut add_to_x = mover.speed * time.delta().as_secs_f32();
        if !mover.direction {
            add_to_x *= -1.0;
        }
        transform.translation.x += add_to_x
    }
}
fn _despawn_and_respawn_rectangle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Transform), With<Movable>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x < -125.0 || transform.translation.x > 125.0 {
            commands.entity(entity).despawn();

            spawn_rectangles(&mut commands, &asset_server);
        }
    }
}
