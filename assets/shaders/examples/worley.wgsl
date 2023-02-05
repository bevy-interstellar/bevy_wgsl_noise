#import noise::prelude

struct CustomMaterial {
    time: f32,
    dimension: i32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let i = world_position * 10.0;

    var r: vec2<f32>;
    if material.dimension == 2 {
        r = noise_worley_vec2f(vec2(i.x + material.time, i.y));
    } else if material.dimension == 3 {
        r = noise_worley_vec3f(vec3(i.x + material.time, i.y, i.z));
    } else {
        r = vec2(1.0);
    }

    if world_position.x > 0.16666667 {
        return vec4(r.x, r.x, r.x, 1.0);
    } else if world_position.x > -0.16666667 {
        return vec4(r.y - r.x, r.y - r.x, r.y - r.x, 1.0);
    } else {
        return vec4(r.y, r.y, r.y, 1.0);
    }
}
