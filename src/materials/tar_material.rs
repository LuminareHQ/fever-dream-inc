#![allow(dead_code)]

use bevy::pbr::MaterialExtension;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct TarDeformMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(0)]
    pub wave_speed: f32,
    #[uniform(0)]
    pub wave_scale: f32,
    #[uniform(0)]
    pub wave_height: f32,
}

impl Default for TarDeformMaterial {
    fn default() -> Self {
        Self {
            time: 0.0,
            wave_speed: 1.0,
            wave_scale: 1.0,
            wave_height: 0.5,
        }
    }
}

impl MaterialExtension for TarDeformMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/tar_shader.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/tar_shader.wgsl".into()
    }
}
