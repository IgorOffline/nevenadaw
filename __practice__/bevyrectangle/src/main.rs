use bevy::prelude::*;
use bevy::window::WindowResolution;
use rand::random;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(800, 600),
                title: "Bevyrectangle".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_rectangle, despawn_and_respawn_rectangle))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());

    spawn_rectangle(&mut commands, &asset_server);
}

fn spawn_rectangle(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    commands.spawn((
        Sprite::from_image(asset_server.load("rectangle_03A9F4_100w100h.png")),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Movable {
            direction: random::<bool>(),
            speed: 125.0,
        },
    ));
}

#[derive(Component)]
struct Movable {
    direction: bool,
    speed: f32,
}

fn move_rectangle(time: Res<Time>, mut query: Query<(&mut Transform, &Movable)>) {
    for (mut transform, mover) in query.iter_mut() {
        let mut add_to_x = mover.speed * time.delta().as_secs_f32();
        if !mover.direction {
            add_to_x *= -1.0;
        }
        transform.translation.x += add_to_x
    }
}
fn despawn_and_respawn_rectangle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    query: Query<(Entity, &Transform), With<Movable>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.x < -125.0 || transform.translation.x > 125.0 {
            commands.entity(entity).despawn();

            spawn_rectangle(&mut commands, &asset_server);
        }
    }
}
