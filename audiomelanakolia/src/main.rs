use bevy::prelude::*;

#[derive(Resource)]
struct FloatValue(f32);

#[derive(Component)]
enum Action {
    Increase,
    Decrease,
}

#[derive(Component)]
struct ValueText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: (640, 360).into(),
                title: "audiomelanakolia".to_string(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(FloatValue(1.0))
        .add_systems(Startup, setup)
        .add_systems(Update, (button_system, update_ui_system))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font = asset_server.load("Iosevka-Italic.ttf");
    commands.spawn(Camera2d);

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text::new("Value: 1.00"),
                TextFont {
                    font: font.clone(),
                    font_size: 40.0,
                    ..default()
                },
                ValueText,
            ));

            parent
                .spawn(Node {
                    column_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|inner_parent| {
                    spawn_button(inner_parent, "Decrease", Action::Decrease, font.clone());
                    spawn_button(inner_parent, "Increase", Action::Increase, font.clone());
                });
        });
}

fn spawn_button(
    parent: &mut ChildSpawnerCommands,
    label: &str,
    action: Action,
    font: Handle<Font>,
) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(150.0),
                height: Val::Px(65.0),
                border: UiRect::all(Val::Px(5.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BorderColor::all(Color::BLACK),
            BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(label),
                TextFont {
                    font,
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &Action, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut float_value: ResMut<FloatValue>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                match action {
                    Action::Increase => float_value.0 = (float_value.0 + 0.1).min(2.0),
                    Action::Decrease => float_value.0 = (float_value.0 - 0.1).max(0.0),
                }
                color.0 = Color::srgb(0.35, 0.75, 0.35);
            }
            Interaction::Hovered => {
                color.0 = Color::srgb(0.25, 0.25, 0.25);
            }
            Interaction::None => {
                color.0 = Color::srgb(0.15, 0.15, 0.15);
            }
        }
    }
}

fn update_ui_system(float_value: Res<FloatValue>, mut query: Query<&mut Text, With<ValueText>>) {
    if float_value.is_changed() {
        for mut text in &mut query {
            text.0 = format!("Value: {:.2}", float_value.0);
        }
    }
}
