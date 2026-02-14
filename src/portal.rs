use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};

pub struct PortalPlugin;

#[derive(Component)]
struct PortalState {
    hovered: bool,
    scale: f32,
}

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add the point light
    commands
        .spawn((
            PortalState {
                hovered: false,
                scale: 1.0,
            },
            Mesh3d(meshes.add(Sphere::default())),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: Color::srgb(16., 0., 0.).into(),
                base_color: Srgba::rgb_u8(255, 0, 0).into(),
                reflectance: 1.0,
                perceptual_roughness: 0.0,
                metallic: 1.0,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(2.0)),
            children![(
                PointLight {
                    shadows_enabled: true,
                    range: 10.0,
                    color: Color::srgb(255., 0., 0.),
                    intensity: 25.0,
                    ..default()
                },
                Transform::from_xyz(0., 0.1, 0.)
            )],
            NotShadowCaster,
            NotShadowReceiver,
        ))
        .observe(click_on_portal);
}

fn hover_portal(on: On<Pointer<Over>>) {
    let entity = on.entity;
}

fn click_on_portal(_on: On<Pointer<Click>>, mut game_data: ResMut<crate::data::GameData>) {
    game_data.currency += 1;
}
