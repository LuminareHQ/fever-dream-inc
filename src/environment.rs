use bevy::{light::FogVolume, pbr::ExtendedMaterial, prelude::*};

use crate::materials::{self, TarDeformMaterial};

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::BLACK));
        // app.insert_resource(GlobalAmbientLight::NONE);
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        FogVolume::default(),
        Transform::from_scale(Vec3::splat(100.0)),
    ));

    commands.spawn((
        Name::new("Ground Plane"),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2 { x: 500., y: 500. }))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("444444").unwrap().into(),
            unlit: false,
            cull_mode: None,
            ..default()
        })),
        Transform::from_xyz(0., 0., 0.),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(2.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("888888").unwrap().into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(1_000_000.0)),
    ));
}
