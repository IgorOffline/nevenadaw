use bevy::prelude::*;
use bevy::window::WindowResolution;
use material_icons::Icon;

fn main() {
    App::new()
        .insert_resource(ClearColor(Srgba::hex("#455A64").unwrap().into()))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(640, 360),
                title: "Bevy Material".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
    spawn_rectangle(&mut commands);
    setup_ui(&mut commands, &asset_server);
}

fn spawn_rectangle(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(Srgba::hex("#FF5722").unwrap(), Vec2 { x: 40.0, y: 25.0 }),
        Transform::from_xyz(0.0, 0.0, 0.0),
    ));
}

fn setup_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    setup_material_ui(commands, asset_server);
    setup_regular_ui(commands, asset_server);
}

fn setup_material_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let material_font: Handle<Font> = asset_server.load("fonts/MaterialIcons-Regular.ttf");
    let icon_home = Icon::Home.to_string();
    let text_font = TextFont {
        font: material_font.clone(),
        font_size: 24.0,
        ..default()
    };
    commands.spawn((
        Text2d::new(&format!("{}", icon_home)),
        text_font,
        TextColor::WHITE,
        Transform::from_xyz(0.0, 0.0, 1.0),
    ));
}

fn setup_regular_ui(commands: &mut Commands, asset_server: &Res<AssetServer>) {
    let iosevka_font: Handle<Font> = asset_server.load("fonts/IosevkaTerm-Regular.ttf");
    let text_font = TextFont {
        font: iosevka_font.clone(),
        font_size: 24.0,
        ..default()
    };
    commands.spawn((
        Text2d::new("Lorem Ipsum"),
        text_font,
        TextColor::WHITE,
        Transform::from_xyz(105.0, 0.0, 1.0),
    ));
}
