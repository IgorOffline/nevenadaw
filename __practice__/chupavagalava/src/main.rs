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
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    let texture: Handle<Image> = asset_server.load("chess.png");
    let layout = TextureAtlasLayout::from_grid(UVec2::splat(480), 6, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);

    let sprite_raw = Sprite::from_atlas_image(
        texture,
        TextureAtlas {
            layout: texture_atlas_layout,
            index: 4,
        },
    );
    let sprite = Sprite {
        custom_size: Some(Vec2::new(240.0, 240.0)),
        ..sprite_raw
    };

    commands.spawn((sprite, MovingObject, Speed(100.0)));
}

fn chu_update(
    time: Res<Time<Real>>,
    mut query: Query<(&mut Transform, &Speed), With<MovingObject>>,
) {
    let delta = 12.0 * time.delta_secs();

    for (mut transform, speed) in &mut query {
        if transform.translation.y > 9999.9 {
            let distance_to_move = speed.0 * delta;

            transform.translation.x += distance_to_move;

            if transform.translation.x > 920.0 {
                transform.translation.x = -920.0;
            }
        }
    }
}
