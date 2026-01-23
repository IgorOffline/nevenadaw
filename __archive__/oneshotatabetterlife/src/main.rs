use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (640, 360).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(
            Startup,
            (
                setup_hello,
                setup_world.after(setup_hello),
                (setup_chain_one, setup_chain_two, setup_chain_three).chain(),
            ),
        )
        .run();
}

fn setup_hello(_commands: Commands) {
    println!("setup_hello");
}

fn setup_world(_commands: Commands) {
    println!("setup_world");
}

fn setup_chain_one(_commands: Commands) {
    println!("setup_chain_one");
}

fn setup_chain_two(_commands: Commands) {
    println!("setup_chain_two");
}

fn setup_chain_three(_commands: Commands) {
    println!("setup_chain_three");
}
