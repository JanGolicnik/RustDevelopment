use bevy::{
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

#[derive(Component)]
struct ScreenQuad;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            Material2dPlugin::<CustomMaterial>::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    window: Query<&Window>,
) {
    let window = window.single();
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    //     transform: Transform::from_xyz(0.0, 0.5, 0.0),
    //     material: materials.add(CustomMaterial {
    //         time: 0.0,
    //         alpha_mode: AlphaMode::Blend,
    //     }),
    //     ..default()
    // });

    // commands.spawn(Camera3dBundle {
    //     transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    //     ..default()
    // });

    let resolution = Vec2::new(
        window.physical_width() as f32,
        window.physical_height() as f32,
    );

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Quad::default().into()).into(),
            material: materials.add(CustomMaterial {
                time: 0.0,
                resolution,
            }),
            transform: Transform {
                scale: Vec3::new(resolution.x, resolution.y, 1.0),
                ..default()
            },
            ..default()
        },
        ScreenQuad,
    ));

    commands.spawn(Camera2dBundle::default());
}

fn update(
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
    window: Query<&Window>,
    mut screen_quad: Query<&mut Transform, With<ScreenQuad>>,
) {
    let window = window.single();

    for (_, material) in materials.iter_mut() {
        material.time = time.elapsed_seconds();
        material.resolution = Vec2::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
        );
    }

    for mut transform in screen_quad.iter_mut() {
        transform.scale = Vec3::new(
            window.physical_width() as f32,
            window.physical_height() as f32,
            1.0,
        );
    }
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders\\material.wgsl".into()
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
pub struct CustomMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    resolution: Vec2,
}
