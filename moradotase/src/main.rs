use bevy::app::PluginGroup;
use bevy::color::Srgba;
use bevy::math::Vec2;
use bevy::prelude::{
    default, App, Camera2d, ClearColor, Commands, Sprite, Transform, Window, WindowPlugin,
};
use bevy::window::WindowResolution;
use bevy::DefaultPlugins;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(pub bosonoga);

mod ast;

#[cfg(test)]
mod tests;

fn main() {
    App::new()
        .insert_resource(ClearColor(Srgba::hex("#607D8B").unwrap().into()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(640, 360),
                title: "Bosonoga".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(bevy::prelude::Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn((
        Sprite {
            color: Srgba::hex("#512DA8").unwrap().into(),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            ..default()
        },
        Transform::default(),
    ));
}
