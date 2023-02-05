#import noise::common

#import noise::simplex_vec4f

struct CustomMaterial {
    time: f32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    // return 1.0;
    let r = noise_simplex_vec4f(vec4(material.time, world_position.x, world_position.y, world_position.z));
    return vec4(r, r, r, 1.0);
}
