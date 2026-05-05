use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::render_resource::Face,
};
use bevy_kira_audio::prelude::*;

use crate::{
    audio,
    data::{AutomatonVariant, automaton_definitions},
    interface::{InterfaceState, set_hovered_automaton},
    rand,
};

pub struct AutomatonsPlugin;

impl Plugin for AutomatonsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(
            Update,
            (
                update_based_on_owned,
                movement,
                update_automatons,
                make_automaton_meshes_unpickable,
            ),
        );
    }
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
        let stats = source.stats();
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

    for definition in automaton_definitions() {
        let stats = &definition.stats;
        let model_path = definition
            .model_path()
            .expect("automaton definition should have a model path");
        let ring_name = definition
            .ring_name()
            .expect("automaton definition should have a ring name");
        let _: Handle<Scene> = asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path));

        commands
            .spawn((
                PurchaseRing,
                Name::new(ring_name),
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
            .observe(set_hovered_automaton::<Pointer<Out>>(None))
            .observe(set_hovered_automaton::<Pointer<Over>>(Some(
                definition.variant,
            )))
            .observe(select_automaton(definition.variant));
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

fn select_automaton(
    variant: AutomatonVariant,
) -> impl Fn(On<Pointer<Click>>, ResMut<InterfaceState>) {
    move |event, mut interface_state| {
        if event.button != PointerButton::Primary {
            return;
        }
        interface_state.selected_automaton = Some(variant);
    }
}

fn update_based_on_owned(
    mut commands: Commands,
    game_data: Res<crate::data::GameData>,
    asset_server: Res<AssetServer>,
    mut automatons: Query<(Entity, &Automaton, &mut Transform)>,
) {
    for definition in automaton_definitions() {
        let variant = definition.variant;
        let quantity_owned = game_data.get_quantity_owned_by_source(variant);
        let current_count = automatons
            .iter()
            .filter(|(_, automaton, _)| automaton.source == variant)
            .count() as u64;

        if quantity_owned == current_count {
            continue;
        }

        let stats = &definition.stats;

        if quantity_owned > current_count {
            let model_path = definition
                .model_path()
                .expect("automaton definition should have a model path");
            for new_index in current_count..quantity_owned {
                let scene: Handle<Scene> =
                    asset_server.load(GltfAssetLabel::Scene(0).from_asset(model_path.clone()));

                commands.spawn((
                    Name::new(definition.display_name),
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

            for (i, (_, _automaton, mut transform)) in automatons
                .iter_mut()
                .filter(|(_, automaton, _)| automaton.source == variant)
                .enumerate()
            {
                *transform = circle_transform(
                    i as u64,
                    quantity_owned,
                    stats.distance_from_origin,
                    stats.scale,
                );
            }
        } else {
            let mut to_remove = (current_count - quantity_owned) as usize;
            let mut despawned: Vec<Entity> = Vec::with_capacity(to_remove);
            for (entity, automaton, _) in automatons.iter() {
                if to_remove == 0 {
                    break;
                }
                if automaton.source == variant {
                    commands.entity(entity).despawn();
                    despawned.push(entity);
                    to_remove -= 1;
                }
            }

            for (i, (_, _automaton, mut transform)) in automatons
                .iter_mut()
                .filter(|(entity, automaton, _)| {
                    automaton.source == variant && !despawned.contains(entity)
                })
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
}

pub fn update_automatons(
    mut automatons: Query<(&mut Automaton, &Transform), Without<AutomatonOrb>>,
    time: Res<Time>,
    mut data: ResMut<crate::data::GameData>,
    mut orbs: Query<(&mut Transform, &mut AutomatonOrb, &mut Visibility)>,
    interaction: Res<AudioChannel<crate::audio::InteractionChannel>>,
    audio_state: Res<crate::audio::AudioState>,
) {
    for (mut automaton, entity_transform) in automatons.iter_mut() {
        if automaton.time_left >= 0.0 {
            automaton.time_left -= time.delta_secs();
        } else {
            data.add_income(automaton.source, automaton.currency_per_tick);

            audio::play_pickup_sound(&interaction, &audio_state);

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
        let stats = automaton.source.stats();
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

fn make_automaton_meshes_unpickable(
    mut commands: Commands,
    automatons: Query<Entity, With<Automaton>>,
    children_query: Query<&Children>,
    meshes: Query<Entity, (With<Mesh3d>, Without<Pickable>)>,
) {
    for automaton in &automatons {
        for descendant in children_query.iter_descendants(automaton) {
            if meshes.contains(descendant) {
                commands.entity(descendant).insert(Pickable::IGNORE);
            }
        }
    }
}
