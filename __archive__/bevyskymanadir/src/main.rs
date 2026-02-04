use bevy::prelude::*;
use bevy_obj::ObjPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, ObjPlugin))
        .add_systems(Startup, (spawn_cube, point_light))
        .add_systems(Update, spin)
        .run();
}

fn spawn_cube(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let texture_handle = asset_server.load("twocubeone/TwoMaterialColor.png");

    commands.spawn((
        Mesh3d(asset_server.load("twocubeone/twocubeone.gltf#Mesh0/Primitive0")),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(texture_handle),
            ..default()
        })),
        Transform::from_xyz(1.7, 0.0, -0.5),
        Spin,
    ));
}

fn point_light(mut commands: Commands) {
    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 4.0, 3.0)));
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(1.7, 2.7, 4.4).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

#[derive(Component)]
struct Spin;

fn spin(time: Res<Time>, mut query: Query<&mut Transform, With<Spin>>) {
    for mut transform in query.iter_mut() {
        transform.rotate_local_y(1.5 * time.delta_secs());
    }
}
