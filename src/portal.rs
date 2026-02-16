use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
};
use bevy_kira_audio::prelude::*;

use rand;

pub struct PortalPlugin;

#[derive(Component)]
struct PortalState {
    hovered: bool,
    scale: f32,
}

#[derive(Component)]
struct PortalRing {
    grow: bool,
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
                alpha_mode: AlphaMode::Opaque,
                ..default()
            })),
            Transform::from_xyz(0., 0., 0.),
            children![(
                PointLight {
                    shadows_enabled: true,
                    range: 40.0,
                    color: Color::srgb(255., 0., 0.),
                    intensity: 40.0,
                    ..default()
                },
                Transform::from_xyz(0., 0.4, 0.)
            )],
            NotShadowCaster,
            NotShadowReceiver,
        ))
        .observe(update_interface_state::<Pointer<Out>>(None))
        .observe(update_interface_state::<Pointer<Over>>(Some(
            crate::data::AutomatonVariant::Portal,
        )))
        .observe(hover_portal)
        .observe(unhover_portal)
        .observe(click_on_portal);

    for _ in 0..10 {
        commands
            .spawn((
                PortalRing { grow: false },
                Mesh3d(meshes.add(Torus {
                    minor_radius: 0.02,
                    major_radius: 0.5,
                })),
                MeshMaterial3d(materials.add(StandardMaterial {
                    emissive: Color::srgb(4., 2., 0.).into(),
                    base_color: Srgba::rgb_u8(255, 127, 0).into(),
                    #[cfg(target_arch = "wasm32")]
                    unlit: true, // Doesn't look right, but prevent the screen from getting blown out on web... :/
                    reflectance: 0.0,
                    perceptual_roughness: 0.0,
                    metallic: 0.0,
                    alpha_mode: AlphaMode::Opaque,
                    ..default()
                })),
                Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(0.5)),
                NotShadowCaster,
                NotShadowReceiver,
                Visibility::Hidden,
            ))
            .insert(Pickable::IGNORE);
    }
}

fn update_interface_state<E: EntityEvent>(
    new_hovered: Option<crate::data::AutomatonVariant>,
) -> impl Fn(On<E>, ResMut<crate::interface::InterfaceState>) {
    move |_event, mut interface_state| {
        interface_state.hovered_automaton = new_hovered;
    }
}

fn hover_portal(_on: On<Pointer<Over>>, mut query: Query<&mut PortalState>) {
    for mut state in query.iter_mut() {
        state.hovered = true;
    }
}

fn unhover_portal(_on: On<Pointer<Out>>, mut query: Query<&mut PortalState>) {
    for mut state in query.iter_mut() {
        state.hovered = false;
    }
}

fn click_on_portal(
    on: On<Pointer<Click>>,
    mut game_data: ResMut<crate::data::GameData>,
    mut rings: Query<(&mut Transform, &mut PortalRing)>,
    audio_state: Res<crate::audio::AudioState>,
    interaction: Res<AudioChannel<crate::audio::InteractionChannel>>,
    asset_server: Res<AssetServer>,
) {
    if on.button != PointerButton::Primary {
        return;
    }

    // Play Pickup Sound if enabled
    if audio_state.play_pickup {
        interaction
            .play(asset_server.load(crate::audio::PICKUP_AUDIO))
            .with_playback_rate(0.75);
    }

    game_data.add_income(crate::data::AutomatonVariant::Portal, 1);
    for (mut ring_transform, mut ring_state) in rings.iter_mut() {
        if ring_transform.scale.x < 0.75 {
            ring_transform.scale = Vec3::splat(1.0);
            ring_state.grow = true;
            ring_transform.rotate_x(rand::random_range(0.0..1.0));
            ring_transform.rotate_y(rand::random_range(0.0..1.0));
            ring_transform.rotate_z(rand::random_range(0.0..1.0));
            break;
        }
    }
}

fn update(
    time: Res<Time>,
    mut portals: Query<(&mut Transform, &mut PortalState), Without<PortalRing>>,
    mut rings: Query<(&mut Transform, &mut PortalRing, &mut Visibility), Without<PortalState>>,
) {
    for (mut transform, mut state) in portals.iter_mut() {
        let target_scale = if state.hovered { 1.10 } else { 1.0 };
        state.scale += (target_scale - state.scale) * time.delta_secs() * 10.0;
        transform.scale = Vec3::splat(state.scale);
    }
    for (mut transform, mut ring_state, mut vis) in rings.iter_mut() {
        if ring_state.grow && transform.scale.x >= 0.75 {
            transform.scale += Vec3::splat(0.2);
            *vis = Visibility::Visible;
            if transform.scale.x > 4.0 {
                transform.scale = Vec3::splat(0.5);
                ring_state.grow = false;
                *vis = Visibility::Hidden;
            }
        }
    }
}
