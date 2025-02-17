struct VertexInput {
    @location(0) position: vec3<f32>,
};

struct InstanceInput {
    @location(1) position: vec3<f32>,
    @location(2) radius: f32,
    @location(3) material_index: u32,
};

struct Camera {
    view_proj: mat4x4<f32>,
};

@group(0) @binding(0) var<uniform> camera: Camera;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) material: vec3<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) instance_position: vec3<f32>,
    @location(3) instance_material: vec3<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    let world_position = position + instance_position;
    out.position = camera.view_proj * vec4<f32>(world_position, 1.0);
    out.world_pos = world_position;
    out.normal = normal;
    out.material = instance_material;
    return out;
}