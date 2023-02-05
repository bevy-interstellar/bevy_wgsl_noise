#import noise::common

#import noise::worley_vec2f

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

    let r = noise_worley_vec2f(vec2(world_position.x + material.time, world_position.y));

    if world_position.x > 0.33333333 {
        return vec4(r.x, r.x, r.x, 1.0);
    } else if world_position.x > -0.33333333 {
        return vec4(r.y - r.x, r.y - r.x, r.y - r.x, 1.0);
    } else {
        return vec4(r.y, r.y, r.y, 1.0);
    }
}
