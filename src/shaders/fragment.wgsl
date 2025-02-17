struct Material {
    base_color: vec4<f32>,
    metallic_roughness: vec2<f32>,
    emission: vec3<f32>,
};

struct CameraUniform {
    view_proj: mat4x4<f32>,
    position: vec4<f32>,
};

@group(0) @binding(0) var<uniform> camera: CameraUniform;
@group(1) @binding(0) var<storage> materials: array<Material>;
@group(1) @binding(1) var<uniform> light: vec4<f32>;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) material: vec3<f32>,
};

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let N = normalize(in.normal);
    let V = normalize(camera.position.xyz - in.world_pos);
    let L = normalize(light.xyz - in.world_pos);
    let H = normalize(V + L);

    let metallic = in.material.x;
    let roughness = in.material.y;
    let selected = in.material.z;

    // Implement PBR lighting calculation here
    let ndotl = max(dot(N, L), 0.0);
    
    // Simple lighting for now
    let color = vec3<f32>(1.0, 0.0, 0.0) * ndotl;
    return vec4<f32>(color, 1.0);
}