use bevy::{light::FogVolume, prelude::*};

use crate::automatons::AutomatonOrb;

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

    commands
        .spawn((
            Name::new("Ground Plane"),
            Mesh3d(meshes.add(Plane3d::new(Vec3::Y, Vec2 { x: 500., y: 500. }))),
            MeshMaterial3d(materials.add(StandardMaterial {
                base_color: Color::srgba_u8(44, 44, 44, 128),
                unlit: false,
                cull_mode: None,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0., -0.01, 0.),
        ))
        .insert(Pickable::IGNORE);

    // Orbs to show currency source
    for i in 0..50 {
        commands.spawn((
            AutomatonOrb::default(),
            Name::new(format!("Orb {}", i)),
            Mesh3d(meshes.add(Sphere {
                radius: 0.05,
                ..default()
            })),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: Color::srgb(8., 0., 0.).into(),
                base_color: Srgba::rgb_u8(255, 0, 0).into(),
                reflectance: 1.0,
                perceptual_roughness: 0.0,
                metallic: 1.0,
                alpha_mode: AlphaMode::Opaque,
                ..default()
            })),
            Transform::from_xyz(0., 0., 0.),
            Visibility::Hidden,
        ));
    }
}
