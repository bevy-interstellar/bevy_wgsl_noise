use bevy::{asset::HandleId, prelude::*};

pub struct WgslNoisePlugin;

impl Plugin for WgslNoisePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(load_noise_shader_file);
    }
}

impl Default for WgslNoisePlugin {
    fn default() -> Self {
        WgslNoisePlugin {}
    }
}

fn load_noise_shader_file(mut shaders: ResMut<Assets<Shader>>) {
    for shader in [
        // common utility functions
        Shader::from_wgsl(include_str!("../assets/shaders/noise/common.wgsl")),
        // perlin noise
        Shader::from_wgsl(include_str!("../assets/shaders/noise/perlin_vec2f.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/perlin_vec3f.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/perlin_vec4f.wgsl")),
        // simplex noise
        Shader::from_wgsl(include_str!("../assets/shaders/noise/simplex_vec2f.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/simplex_vec3f.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/simplex_vec4f.wgsl")),
        // worley noise
        Shader::from_wgsl(include_str!("../assets/shaders/noise/worley_vec2f.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/worley_vec3f.wgsl")),
        // fractal brownian motion
        Shader::from_wgsl(include_str!("../assets/shaders/noise/fbm_all.wgsl")),
        // domain distortion on fbm
        Shader::from_wgsl(include_str!("../assets/shaders/noise/distortion_all.wgsl")),
        // prelude
        Shader::from_wgsl(include_str!("../assets/shaders/noise/prelude.wgsl")),
    ] {
        let handle = HandleId::random::<Shader>();
        shaders.set_untracked(handle, shader);
    }
}
