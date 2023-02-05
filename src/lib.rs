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
        Shader::from_wgsl(include_str!("../assets/shaders/noise/common.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/perlin_vec2f.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/perlin_vec3f.wgsl")),
        Shader::from_wgsl(include_str!("../assets/shaders/noise/perlin_vec4f.wgsl")),
    ] {
        let handle = HandleId::random::<Shader>();
        shaders.set_untracked(handle, shader);
    }
}
