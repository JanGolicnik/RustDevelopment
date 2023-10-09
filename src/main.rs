use std::f32::consts::PI;

use bevy::{
    pbr::{MaterialPipeline, MaterialPipelineKey},
    prelude::*,
    reflect::{TypePath, TypeUuid},
    render::{
        camera::RenderTarget,
        mesh::MeshVertexBufferLayout,
        render_resource::{
            AsBindGroup, Extent3d, RenderPipelineDescriptor, ShaderRef,
            SpecializedMeshPipelineError, TextureDescriptor, TextureDimension, TextureFormat,
            TextureUsages,
        },
        view::RenderLayers,
    },
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MaterialPlugin::<WaterMaterial>::default())
        .add_plugins(Material2dPlugin::<PostProcessingMaterial>::default())
        .add_systems(Startup, startup)
        .add_systems(Update, update)
        .run();
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e0"]
struct WaterMaterial {
    #[uniform(0)]
    time: f32,
    #[uniform(1)]
    cam_pos: Vec3,
}

impl Material for WaterMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/water.wgsl".into()
    }
    fn fragment_shader() -> ShaderRef {
        "shaders/water.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        let vertex_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ])?;
        descriptor.vertex.buffers = vec![vertex_layout];
        Ok(())
    }
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "bc2f08eb-a0fb-43f1-a908-54871ea597d5"]
struct PostProcessingMaterial {
    #[uniform(0)]
    time: f32,
    #[texture(1)]
    #[sampler(2)]
    source_image: Handle<Image>,
}

impl Material2d for PostProcessingMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/aberration.wgsl".into()
    }
}

fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut popr_materials: ResMut<Assets<PostProcessingMaterial>>,
    windows: Query<&Window>,
    mut images: ResMut<Assets<Image>>,
) {
    let window = windows.get_single().unwrap();
    let size = Extent3d {
        width: window.physical_width(),
        height: window.physical_height(),
        ..default()
    };

    commands.spawn(Camera2dBundle::default());

    let mut ocean = Mesh::from(shape::Plane {
        subdivisions: 3000,
        size: 100.0,
    });

    ocean.remove_attribute(Mesh::ATTRIBUTE_JOINT_INDEX);
    ocean.remove_attribute(Mesh::ATTRIBUTE_JOINT_WEIGHT);
    ocean.remove_attribute(Mesh::ATTRIBUTE_COLOR);
    ocean.remove_attribute(Mesh::ATTRIBUTE_TANGENT);

    let ocean = meshes.add(ocean);

    let first_pass_layer = RenderLayers::layer(1);

    let cam_transform =
        Transform::from_xyz(0.0, 1.5, 0.0).looking_at(Vec3::new(50.0, 1.3, 0.0), Vec3::Y);

    commands.spawn((
        MaterialMeshBundle {
            mesh: ocean,
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..default()
            },
            material: materials.add(WaterMaterial {
                time: 0.0,
                cam_pos: cam_transform.translation,
            }),
            ..default()
        },
        first_pass_layer,
    ));

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        },
        ..default()
    };

    image.resize(size);

    let image_handle = images.add(image);

    commands.spawn((
        Camera3dBundle {
            camera_3d: Camera3d {
                clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                    Color::AZURE,
                ),
                ..default()
            },
            camera: Camera {
                target: RenderTarget::Image(image_handle.clone()),
                ..default()
            },
            transform: cam_transform,
            ..default()
        },
        first_pass_layer,
    ));

    let quad_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
        size.width as f32,
        size.height as f32,
    ))));

    commands.spawn(MaterialMesh2dBundle {
        mesh: quad_handle.into(),
        material: popr_materials.add(PostProcessingMaterial {
            time: 0.0,
            source_image: image_handle,
        }),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 1.5),
            ..default()
        },
        ..default()
    });
}

fn update(
    mut materials: ResMut<Assets<WaterMaterial>>,
    mut popr_materials: ResMut<Assets<PostProcessingMaterial>>,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut camera_transform_query: Query<&mut Transform, With<Camera3d>>,
) {
    for (_, material) in materials.iter_mut() {
        material.time = time.elapsed_seconds_wrapped();
    }

    for (_, popr_material) in popr_materials.iter_mut() {
        popr_material.time = time.elapsed_seconds_wrapped();
    }

    let mut camera_transform = camera_transform_query.single_mut();
    if keyboard_input.pressed(KeyCode::A) {
        camera_transform.rotate_axis(Vec3::Y, PI * time.delta_seconds());
    }
    if keyboard_input.pressed(KeyCode::D) {
        camera_transform.rotate_axis(Vec3::Y, -PI * time.delta_seconds());
    }
}
