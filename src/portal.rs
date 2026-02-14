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
        app.add_systems(Update, update);
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
            Mesh3d(meshes.add(Sphere {
                radius: 1.,
                ..default()
            })),
            MeshMaterial3d(materials.add(StandardMaterial {
                emissive: Color::srgb(8., 0., 0.).into(),
                base_color: Srgba::rgb_u8(255, 0, 0).into(),
                reflectance: 1.0,
                perceptual_roughness: 0.0,
                metallic: 1.0,
                alpha_mode: AlphaMode::Blend,
                ..default()
            })),
            Transform::from_xyz(0., 0., 0.),
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
        .observe(hover_portal)
        .observe(unhover_portal)
        .observe(click_on_portal);
}

fn hover_portal(on: On<Pointer<Over>>, mut query: Query<&mut PortalState>) {
    for mut state in query.iter_mut() {
        state.hovered = true;
    }
}

fn unhover_portal(on: On<Pointer<Out>>, mut query: Query<&mut PortalState>) {
    for mut state in query.iter_mut() {
        state.hovered = false;
    }
}

fn click_on_portal(
    _on: On<Pointer<Click>>,
    mut game_data: ResMut<crate::data::GameData>,
    mut portals: Query<(&mut Transform, &mut PortalState)>,
) {
    game_data.currency += 1;
    for (mut transform, state) in portals.iter_mut() {
        transform.scale = Vec3::splat(1.5);
    }
}

fn update(time: Res<Time>, mut portals: Query<(&mut Transform, &mut PortalState)>) {
    for (mut transform, mut state) in portals.iter_mut() {
        let target_scale = if state.hovered { 1.10 } else { 1.0 };
        state.scale += (target_scale - state.scale) * time.delta_secs() * 10.0;
        transform.scale = Vec3::splat(state.scale);
    }
}
