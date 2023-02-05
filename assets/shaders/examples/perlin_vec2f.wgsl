#import noise::common

#import noise::perlin_vec2f

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
    let r = noise_perlin_vec2f(vec2(world_position.x, world_position.y));
    return vec4(r, r, r, 1.0);
}
