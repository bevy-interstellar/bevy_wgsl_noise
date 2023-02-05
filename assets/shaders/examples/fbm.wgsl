#import noise::prelude

struct CustomMaterial {
    time: f32,
    dimension: i32,
    perlin: i32,
};

@group(1) @binding(0)
var<uniform> material: CustomMaterial;

@fragment
fn fragment(
    #import bevy_pbr::mesh_vertex_output
) -> @location(0) vec4<f32> {
    let i = world_position * 10.0;
    if material.perlin == 1 {
        if material.dimension == 2 {
            let r = noise_fbm_perlin_vec2f(vec2(i.x + material.time, i.y), 10, 2.0, 0.5);
            return vec4(r, r, r, 1.0);
        } else if material.dimension == 3 {
            let r = noise_fbm_perlin_vec3f(vec3(i.x + material.time, i.y, i.z), 10, 2.0, 0.5);
            return vec4(r, r, r, 1.0);
        } else if material.dimension == 4 {
            let r = noise_fbm_perlin_vec4f(vec4(material.time, i.x, i.y, i.z), 10, 2.0, 0.5);
            return vec4(r, r, r, 1.0);
        }
    } else {
        if material.dimension == 2 {
            let r = noise_fbm_simplex_vec2f(vec2(i.x + material.time, i.y), 10, 2.0, 0.5);
            return vec4(r, r, r, 1.0);
        } else if material.dimension == 3 {
            let r = noise_fbm_simplex_vec3f(vec3(i.x + material.time, i.y, i.z), 10, 2.0, 0.5);
            return vec4(r, r, r, 1.0);
        } else if material.dimension == 4 {
            let r = noise_fbm_simplex_vec4f(vec4(material.time, i.x, i.y, i.z), 10, 2.0, 0.5);
            return vec4(r, r, r, 1.0);
        }
    }

    return vec4(1.0);
}
