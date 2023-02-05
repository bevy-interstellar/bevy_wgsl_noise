#import noise::prelude

struct CustomMaterial {
    time: f32,
    lacu: f32,
    gain: f32
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let i = world_position * 10.0;
    let r = noise_fbm_perlin_vec4f(vec4(material.time, i.x, i.y, i.z), 8, material.lacu, material.gain);
    return vec4(r, r, r, 1.0);
}
