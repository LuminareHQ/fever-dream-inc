use bevy::{
    input::mouse::{AccumulatedMouseMotion, AccumulatedMouseScroll},
    post_process::bloom::Bloom,
    prelude::*,
};
use std::f32::consts::FRAC_PI_2;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, orbit_camera);
    }
}

/// Marker + settings for the orbit camera.
#[derive(Component)]
pub struct OrbitCamera {
    /// The point the camera orbits around / looks at.
    pub target: Vec3,
    /// Distance from target (the "arm length").
    pub distance: f32,
    /// Horizontal angle (radians).
    pub yaw: f32,
    /// Vertical angle (radians), clamped to avoid gimbal flip.
    pub pitch: f32,
    /// Mouse sensitivity for orbiting (radians per pixel).
    pub orbit_sensitivity: f32,
    /// Mouse sensitivity for panning (world-units per pixel).
    pub dolly_sensitivity: f32,
    /// Minimum arm distance (prevents zooming through the target).
    pub min_distance: f32,
    /// Maximum arm distance.
    pub max_distance: f32,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            target: Vec3::ZERO,
            distance: 10.0,
            yaw: 45.0_f32.to_radians(),
            pitch: -25.0_f32.to_radians(),
            orbit_sensitivity: 0.005,
            #[cfg(target_arch = "wasm32")]
            dolly_sensitivity: 0.01,
            #[cfg(not(target_arch = "wasm32"))]
            dolly_sensitivity: 1.0,
            min_distance: 5.0,
            max_distance: 35.0,
        }
    }
}

fn setup(
    mut commands: Commands,
    mut _meshes: ResMut<Assets<Mesh>>,
    mut _materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(PerspectiveProjection {
            fov: 65.0_f32.to_radians(),
            ..default()
        }),
        OrbitCamera::default(),
        // Transform will be set by the orbit_camera system on the first frame.
        Transform::from_translation(Vec3::ZERO).looking_at(Vec3::ZERO, Vec3::Y),
        Bloom::NATURAL,
        DistanceFog {
            color: Color::srgb(0.0, 0.0, 0.0),
            falloff: FogFalloff::Linear {
                start: 5.0,
                end: 200.0,
            },
            ..default()
        },
    ));
}

fn orbit_camera(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    mouse_scroll: Res<AccumulatedMouseScroll>,
    mut query: Query<(&mut Transform, &mut OrbitCamera)>,
) {
    let delta = mouse_motion.delta; // pixels moved this frame
    let scroll = mouse_scroll.delta.y; // scroll lines this frame

    for (mut transform, mut cam) in &mut query {
        // --- Orbit: right mouse button + drag ---
        if mouse_buttons.pressed(MouseButton::Left) {
            cam.yaw -= delta.x * cam.orbit_sensitivity;
            cam.pitch -= delta.y * cam.orbit_sensitivity;
            // Clamp pitch to avoid flipping (just under ±90°).
            cam.pitch = cam.pitch.clamp(-FRAC_PI_2 + 0.15, -0.25);
        }

        // --- Dolly arm: scroll wheel ---
        cam.distance -= scroll * cam.dolly_sensitivity;
        cam.distance = cam.distance.clamp(cam.min_distance, cam.max_distance);

        // --- Reconstruct transform from spherical coords ---
        let rot = Quat::from_euler(EulerRot::YXZ, cam.yaw, cam.pitch, 0.0);
        // The "arm" points along +Z in camera space, rotated into world space.
        transform.translation = cam.target + rot * Vec3::new(0.0, 0.0, cam.distance);
        transform.look_at(cam.target, Vec3::Y);
    }
}
