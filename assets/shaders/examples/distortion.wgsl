#import noise::prelude

struct CustomMaterial {
    time: f32,
    lacu: f32,
    iter: i32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let i = world_position * 4.0;
    let r = noise_distortion_simplex_vec4f(vec4(material.time * 0.1, i.x, i.y, i.z), material.iter, 4, material.lacu, 0.5);
    return vec4(r, r, r, 1.0);
}
