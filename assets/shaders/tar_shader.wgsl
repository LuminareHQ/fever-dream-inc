#import bevy_pbr::{
    mesh_functions,
    view_transformations::position_world_to_clip,
}

struct Vertex {
    @builtin(instance_index) instance_index: u32,
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) world_position: vec3<f32>,
    @location(1) world_normal: vec3<f32>,
};

struct WaterMaterial {
    time: f32,
    wave_speed: f32,
    wave_scale: f32,
    wave_height: f32,
}

@group(2) @binding(0)
var<uniform> material: WaterMaterial;

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;
    var position = vertex.position;

    // Create wavy deformation using multiple sine waves for a more random feel
    let wave1 = sin(position.x * 2.0 + material.time * material.wave_speed) * material.wave_height;
    let wave2 = sin(position.z * 3.0 + material.time * material.wave_speed * 0.7) * material.wave_height * 0.8;
    let wave3 = sin((position.x + position.z) * 1.5 + material.time * material.wave_speed * 1.3) * material.wave_height * 0.6;

    // Combine waves for more complex motion
    position.y += (wave1 + wave2 + wave3) * material.wave_scale;

    // Transform to world space
    let world_from_local = mesh_functions::get_world_from_local(vertex.instance_index);
    let world_position = mesh_functions::mesh_position_local_to_world(world_from_local, vec4<f32>(position, 1.0));

    out.world_position = world_position.xyz;
    out.clip_position = position_world_to_clip(world_position.xyz);

    // Transform normal to world space
    out.world_normal = mesh_functions::mesh_normal_local_to_world(vertex.normal, vertex.instance_index);

    return out;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // Black color
    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
}
