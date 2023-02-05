use bevy::{
    prelude::{shape::Cube, *},
    reflect::TypeUuid,
    render::render_resource::*,
};

use bevy_wgsl_noise::WgslNoisePlugin;

#[derive(Component)]
struct Movable;

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "f3a5acb9-88f8-4f84-b54c-d113138451d8"]
struct CustomMaterial {
    #[uniform(0)]
    _time: f32,
    #[uniform(0)]
    _luca: f32,
    #[uniform(0)]
    _gain: f32,
}

impl Material for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/examples/fbm_simplex.wgsl".into()
    }
}

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(MaterialPlugin::<CustomMaterial>::default())
        .add_plugin(WgslNoisePlugin::default())
        .add_startup_system(setup)
        .add_system(object_rotate)
        .add_system(update_time)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    let mesh: Mesh = Cube::new(1.0).into();

    for s in 0..5 {
        for t in 0..5 {
            let luca = s as f32 / 2.0 + 0.5;
            let gain = t as f32 / 2.0 + 0.5;

            commands.spawn(MaterialMeshBundle::<CustomMaterial> {
                mesh: meshes.add(mesh.clone()),
                material: materials.add(CustomMaterial {
                    _time: 0.0,
                    _luca: luca,
                    _gain: gain,
                }),
                transform: Transform::from_xyz(s as f32 - 2.0, t as f32 - 2.0, 0.0),
                ..default()
            });
        }
    }

    let camera_origin = commands.spawn((TransformBundle::default(), Movable)).id();

    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .id();

    commands.entity(camera_origin).add_child(camera);
}

fn update_time(time: Res<Time>, mut materials: ResMut<Assets<CustomMaterial>>) {
    for (_, material) in materials.iter_mut() {
        material._time = time.elapsed_seconds_f64() as f32;
    }
}

fn object_rotate(
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Movable>>,
) {
    for mut transform in &mut query {
        if input.pressed(KeyCode::Up) {
            transform.rotate_x(time.delta_seconds());
        }
        if input.pressed(KeyCode::Down) {
            transform.rotate_x(-time.delta_seconds());
        }
        if input.pressed(KeyCode::Left) {
            transform.rotate_y(time.delta_seconds());
        }
        if input.pressed(KeyCode::Right) {
            transform.rotate_y(-time.delta_seconds());
        }
    }
}
