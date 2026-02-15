use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    render::render_resource::Face,
};

use crate::{automatons::Automaton, data::GameData};

pub struct WoolyChionoescentPlugin;

static DISTANCE_FROM_ORIGIN: f32 = 2.5;
static COOLDOWN: f32 = 10.0;
static CURRENCY_PER_TICK: u64 = 1;
static SCALE: f32 = 0.25;

fn random_time_left() -> f32 {
    rand::random_range(0.0..COOLDOWN)
}

impl Plugin for WoolyChionoescentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, movement);
        app.add_systems(Update, update_based_on_owned);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Preloading the scene so that it doesn't cause a hitch when we first spawn a WoolyChionoescent
    let _: Handle<Scene> =
        asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wooly_chionoescent.glb"));

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

    commands
        .spawn((
            Name::new("WoolyChionoescent_ring"),
            Mesh3d(meshes.add(Torus {
                minor_radius: 0.25,
                major_radius: DISTANCE_FROM_ORIGIN,
            })),
            MeshMaterial3d(transparent_mat.clone()),
            NotShadowCaster,
            NotShadowReceiver,
        ))
        .observe(update_material_on::<Pointer<Out>>(transparent_mat.clone()))
        .observe(update_material_on::<Pointer<Over>>(hover_mat.clone()))
        .observe(update_interface_state::<Pointer<Out>>(None))
        .observe(update_interface_state::<Pointer<Over>>(Some(
            crate::data::IncomeSource::WoolyChionoescent,
        )))
        .observe(click);
}

fn click(_: On<Pointer<Click>>, mut game_data: ResMut<GameData>) {
    game_data.purchase_source(crate::data::IncomeSource::WoolyChionoescent);
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
    new_hovered: Option<crate::data::IncomeSource>,
) -> impl Fn(On<E>, ResMut<crate::interface::InterfaceState>) {
    move |_event, mut interface_state| {
        interface_state.hovered_automaton = new_hovered;
    }
}

fn update_based_on_owned(
    mut commands: Commands,
    game_data: Res<GameData>,
    asset_server: Res<AssetServer>,
    mut wooly_chionoescents: Query<(Entity, &mut Automaton, &mut Transform)>,
) {
    let quantity_owned =
        game_data.get_quantity_owned_by_source(crate::data::IncomeSource::WoolyChionoescent);
    let current_count = wooly_chionoescents.iter().count() as u64;
    if quantity_owned != current_count {
        // Load the WoolyChionoescent scene
        let scene: Handle<Scene> =
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("models/wooly_chionoescent.glb"));

        // Compute the total number of entities after spawning one more
        let total = current_count + 1;

        // Spawn one new WoolyChionoescent at its position on the circle
        let new_index = current_count;
        let angle = 2.0 * std::f32::consts::PI * (new_index as f32) / (total as f32);
        let x = DISTANCE_FROM_ORIGIN * angle.cos();
        let z = DISTANCE_FROM_ORIGIN * angle.sin();

        commands.spawn((
            Name::new("WoolyChionoescent"),
            SceneRoot(scene),
            Automaton {
                source: crate::data::IncomeSource::WoolyChionoescent,
                currency_per_tick: CURRENCY_PER_TICK,
                cooldown: COOLDOWN,
                time_left: random_time_left(),
            },
            Transform::from_xyz(x, 0.0, z)
                .looking_at(Vec3::ZERO, Vec3::Y)
                .with_scale(Vec3::splat(SCALE)),
        ));

        // Reposition all existing entities to form an evenly spaced circle
        for (i, (_entity, _automaton, mut transform)) in wooly_chionoescents.iter_mut().enumerate() {
            let angle = 2.0 * std::f32::consts::PI * (i as f32) / (total as f32);
            let x = DISTANCE_FROM_ORIGIN * angle.cos();
            let z = DISTANCE_FROM_ORIGIN * angle.sin();
            *transform = Transform::from_xyz(x, 0.0, z)
                .looking_at(Vec3::ZERO, Vec3::Y)
                .with_scale(Vec3::splat(SCALE));
        }
    }
}

fn movement(mut query: Query<(&mut Transform, &Automaton), With<Name>>, time: Res<Time>) {
    let distance_from_origin = 2.5;
    let nudge_amount = 0.1;
    let nudge_recovery_duration = 0.5;

    for (mut transform, automaton) in query.iter_mut() {
        // Handle Movement
        let center = Vec3::ZERO;
        let angle = 0.05 * time.delta_secs();
        let rot = Quat::from_rotation_y(angle);

        let rel = transform.translation - center;
        let new_translation = center + rot * rel;

        // Calculate the nudge offset based on time_left relative to cooldown
        let time_since_tick = automaton.cooldown - automaton.time_left;
        let current_distance = if time_since_tick < nudge_recovery_duration {
            // Lerp from nudged position back to normal distance
            let t = time_since_tick / nudge_recovery_duration;
            distance_from_origin - nudge_amount * (1.0 - t)
        } else {
            distance_from_origin
        };

        // Normalize direction and set to the correct distance
        let direction = new_translation.normalize_or_zero();
        transform.translation = direction * current_distance;

        let look =
            Transform::from_translation(transform.translation).looking_at(Vec3::ZERO, Vec3::Y);
        // let rot90_cw = Quat::from_rotation_y(std::f32::consts::FRAC_PI_2);
        transform.rotation = look.rotation;
    }
}
