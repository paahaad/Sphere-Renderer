struct Sphere {
    position: vec3<f32>,
    radius: f32,
    material_index: u32,
    padding: vec3<u32>,
};

struct DrawCommand {
    vertex_count: u32,
    instance_count: u32,
    base_vertex: u32,
    base_instance: u32,
};

@group(0) @binding(0) var<storage, read_write> spheres: array<Sphere>;
@group(1) @binding(0) var<storage, read> sphere_data: array<Sphere>;
@group(1) @binding(1) var<uniform> light: vec4<f32>;
@group(0) @binding(1) var<storage, read_write> draw_commands: array<DrawCommand>;
@group(0) @binding(2) var<storage, read_write> visible_spheres: array<Sphere>;

@compute @workgroup_size(256)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let index = global_id.x;
    if (index >= arrayLength(&spheres)) {
        return;
    }
    
    // Implement frustum culling here
    // Update draw commands and visible_spheres

    // Your compute shader logic here
    // Example: Update sphere positions
    var sphere = spheres[index];
    // ... update logic ...
    spheres[index] = sphere;
}