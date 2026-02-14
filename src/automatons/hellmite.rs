use bevy::prelude::*;

pub struct HellmitePlugin;

impl Plugin for HellmitePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/hellmite.glb"));

    commands.spawn((
        Name::new("Hellmite"),
        Mesh3d(SceneRoot(model)),
        Transform::from_xyz(10., 0., 0.).look_at(Vec3::ZERO, Vec3::Y),
    ));
}
