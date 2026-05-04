use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::render_resource::Face,
};
use bevy_kira_audio::prelude::*;

use crate::{config::get_stats, data::AutomatonVariant};

pub struct AutomatonsPlugin;

impl Plugin for AutomatonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, (update_based_on_owned, movement, update_automatons));
    }
}

#[derive(Clone, Copy)]
pub struct AutomatonDefinition {
    pub variant: AutomatonVariant,
    pub model_path: &'static str,
    pub entity_name: &'static str,
    pub ring_name: &'static str,
}

// Unlock order follows this table, while numeric balance stays in config.rs.
pub const AUTOMATON_DEFINITIONS: [AutomatonDefinition; 8] = [
    AutomatonDefinition {
        variant: AutomatonVariant::Hellmite,
        model_path: "models/hellmite.glb",
        entity_name: "Hellmite",
        ring_name: "hellmite_ring",
    },
    AutomatonDefinition {
        variant: AutomatonVariant::Abyssopod,
        model_path: "models/abyssopod.glb",
        entity_name: "Abyssopod",
        ring_name: "Abyssopod_ring",
    },
    AutomatonDefinition {
        variant: AutomatonVariant::GapingDubine,
        model_path: "models/gaping_dubine.glb",
        entity_name: "GapingDubine",
        ring_name: "GapingDubine_ring",
    },
    AutomatonDefinition {
        variant: AutomatonVariant::GazingHoku,
        model_path: "models/gazing_hoku.glb",
        entity_name: "GazingHoku",
        ring_name: "GazingHoku_ring",
    },
    AutomatonDefinition {
        variant: AutomatonVariant::Lorgner,
        model_path: "models/lorgner.glb",
        entity_name: "Lorgner",
        ring_name: "Lorgner_ring",
    },
    AutomatonDefinition {
        variant: AutomatonVariant::PelteLacerte,
        model_path: "models/pelte_lacerte.glb",
        entity_name: "PelteLacerte",
        ring_name: "PelteLacerte_ring",
    },
    AutomatonDefinition {
        variant: AutomatonVariant::Struthios,
        model_path: "models/struthios.glb",
        entity_name: "Struthios",
        ring_name: "Struthios_ring",
    },
    AutomatonDefinition {
        variant: AutomatonVariant::WoolyChionoescent,
        model_path: "models/wooly_chionoescent.glb",
        entity_name: "WoolyChionoescent",
        ring_name: "WoolyChionoescent_ring",
    },
];

pub fn is_automaton_variant(variant: AutomatonVariant) -> bool {
    AUTOMATON_DEFINITIONS
        .iter()
        .any(|definition| definition.variant == variant)
}

pub fn previous_automaton_variant(variant: AutomatonVariant) -> Option<AutomatonVariant> {
    AUTOMATON_DEFINITIONS
        .iter()
        .position(|definition| definition.variant == variant)
        .and_then(|index| index.checked_sub(1))
        .map(|previous_index| AUTOMATON_DEFINITIONS[previous_index].variant)
}

#[derive(Component)]
pub struct PurchaseRing;

#[derive(Component)]
pub struct Automaton {
    source: AutomatonVariant,
    currency_per_tick: u64,
    cooldown: f32,
    time_left: f32,
}

impl Automaton {
    fn new(source: AutomatonVariant) -> Self {
        let stats = get_stats(source);
        Self {
            source,
            currency_per_tick: stats.currency_per_tick,
            cooldown: stats.cooldown,
            time_left: random_time_left(stats.cooldown),
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let transparent_mat = materials.add(StandardMaterial {
        base_color: Srgba::rgba_u8(255, 255, 255, 0).into(),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let hover_mat = materials.add(StandardMaterial {
        base_color: Srgba::rgba_u8(255, 255, 255, 12).into(),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        cull_mode: Some(Face::Front),
        ..default()
    });

    for definition in AUTOMATON_DEFINITIONS {
        let stats = get_stats(definition.variant);
        let _: Handle<Scene> =
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(definition.model_path));

        commands
            .spawn((
                PurchaseRing,
                Name::new(definition.ring_name),
                Mesh3d(meshes.add(Torus {
                    minor_radius: stats.scale,
                    major_radius: stats.distance_from_origin,
                })),
                MeshMaterial3d(transparent_mat.clone()),
                NotShadowCaster,
                NotShadowReceiver,
            ))
            .observe(update_material_on::<Pointer<Out>>(transparent_mat.clone()))
            .observe(update_material_on::<Pointer<Over>>(hover_mat.clone()))
            .observe(update_interface_state::<Pointer<Out>>(None))
            .observe(update_interface_state::<Pointer<Over>>(Some(
                definition.variant,
            )))
            .observe(purchase_automaton(definition.variant));
    }
}

fn update_material_on<E: EntityEvent>(
    new_material: Handle<StandardMaterial>,
) -> impl Fn(On<E>, Query<&mut MeshMaterial3d<StandardMaterial>>) {
    move |event, mut query| {
        if let Ok(mut material) = query.get_mut(event.event_target()) {
            material.0 = new_material.clone();
        }
    }
}

fn update_interface_state<E: EntityEvent>(
    new_hovered: Option<AutomatonVariant>,
) -> impl Fn(On<E>, ResMut<crate::interface::InterfaceState>) {
    move |_event, mut interface_state| {
        interface_state.hovered_automaton = new_hovered;
    }
}

fn purchase_automaton(
    variant: AutomatonVariant,
) -> impl Fn(On<Pointer<Click>>, ResMut<crate::data::GameData>) {
    move |_event, mut game_data| {
        game_data.purchase_source(variant);
    }
}

fn update_based_on_owned(
    mut commands: Commands,
    game_data: Res<crate::data::GameData>,
    asset_server: Res<AssetServer>,
    mut automatons: Query<(&Automaton, &mut Transform)>,
) {
    for definition in AUTOMATON_DEFINITIONS {
        let variant = definition.variant;
        let quantity_owned = game_data.get_quantity_owned_by_source(variant);
        let current_count = automatons
            .iter()
            .filter(|(automaton, _)| automaton.source == variant)
            .count() as u64;

        if quantity_owned <= current_count {
            continue;
        }

        let stats = get_stats(variant);
        for new_index in current_count..quantity_owned {
            let scene: Handle<Scene> =
                asset_server.load(GltfAssetLabel::Scene(0).from_asset(definition.model_path));

            commands.spawn((
                Name::new(definition.entity_name),
                SceneRoot(scene),
                Automaton::new(variant),
                circle_transform(
                    new_index,
                    quantity_owned,
                    stats.distance_from_origin,
                    stats.scale,
                ),
            ));
        }

        for (i, (_automaton, mut transform)) in automatons
            .iter_mut()
            .filter(|(automaton, _)| automaton.source == variant)
            .enumerate()
        {
            *transform = circle_transform(
                i as u64,
                quantity_owned,
                stats.distance_from_origin,
                stats.scale,
            );
        }
    }
}

pub fn update_automatons(
    mut automatons: Query<(&mut Automaton, &Transform), Without<AutomatonOrb>>,
    time: Res<Time>,
    mut data: ResMut<crate::data::GameData>,
    mut orbs: Query<(&mut Transform, &mut AutomatonOrb, &mut Visibility)>,
    interaction: Res<AudioChannel<crate::audio::InteractionChannel>>,
    asset_server: Res<AssetServer>,
    audio_state: Res<crate::audio::AudioState>,
) {
    for (mut automaton, entity_transform) in automatons.iter_mut() {
        if automaton.time_left >= 0.0 {
            automaton.time_left -= time.delta_secs();
        } else {
            data.add_income(automaton.source, automaton.currency_per_tick);

            // Play Pickup Sound if enabled
            if audio_state.play_pickup {
                interaction
                    .play(asset_server.load(crate::audio::PICKUP_AUDIO))
                    .with_playback_rate(0.75);
            }

            automaton.time_left = automaton.cooldown;
            for (mut orb_transform, mut orb, _) in orbs.iter_mut() {
                if orb_transform.translation.distance(Vec3::ZERO) <= 0.25 {
                    orb.start = entity_transform.translation;
                    orb_transform.translation = orb.start;
                    orb.progress = 0.0;
                    break;
                }
            }
        }
    }

    for (mut transform, mut orb, mut vis) in orbs.iter_mut() {
        if transform.translation.distance(Vec3::ZERO) > 0.25 {
            *vis = Visibility::Visible;
            orb.update(time.delta_secs());
            let t = orb.progress / 0.5;
            transform.translation = orb.start.lerp(Vec3::ZERO, t);

            // Catch for is something goes wrong, just reset to the center
            if transform.translation.distance(Vec3::ZERO) > 100. {
                transform.translation = Vec3::ZERO;
            }
        } else {
            *vis = Visibility::Hidden;
        }
    }
}

fn movement(mut query: Query<(&mut Transform, &Automaton)>, time: Res<Time>) {
    let nudge_amount = 0.1;
    let nudge_recovery_duration = 0.5;

    for (mut transform, automaton) in query.iter_mut() {
        let stats = get_stats(automaton.source);
        let angle = stats.rotation * time.delta_secs();
        let rot = Quat::from_rotation_y(angle);
        let new_translation = rot * transform.translation;

        let time_since_tick = automaton.cooldown - automaton.time_left;
        let current_distance = if time_since_tick < nudge_recovery_duration {
            let t = time_since_tick / nudge_recovery_duration;
            stats.distance_from_origin - nudge_amount * (1.0 - t)
        } else {
            stats.distance_from_origin
        };

        transform.translation = new_translation.normalize_or_zero() * current_distance;
        transform.rotation = Transform::from_translation(transform.translation)
            .looking_at(Vec3::ZERO, Vec3::Y)
            .rotation;
    }
}

fn random_time_left(cooldown: f32) -> f32 {
    rand::random_range(0.0..cooldown)
}

fn circle_transform(index: u64, total: u64, distance_from_origin: f32, scale: f32) -> Transform {
    let angle = 2.0 * std::f32::consts::PI * (index as f32) / (total as f32);
    let x = distance_from_origin * angle.cos();
    let z = distance_from_origin * angle.sin();

    Transform::from_xyz(x, 0.0, z)
        .looking_at(Vec3::ZERO, Vec3::Y)
        .with_scale(Vec3::splat(scale))
}

#[derive(Component)]
pub struct AutomatonOrb {
    start: Vec3,
    progress: f32,
}

impl Default for AutomatonOrb {
    fn default() -> Self {
        Self {
            start: Vec3::ZERO,
            progress: 1.0,
        }
    }
}

impl AutomatonOrb {
    pub fn update(&mut self, time: f32) {
        self.progress += time;
    }
}
