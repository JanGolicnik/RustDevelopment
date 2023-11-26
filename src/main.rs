use bevy::prelude::*;
use bevy_flycam::prelude::*;
use chunks::{chunkmap::ChunkMap, ChunkPlugin};

mod chunks;
mod craftdebug;

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            NoCameraPlayerPlugin,
            ChunkPlugin,
        ))
        .insert_resource(MovementSettings {
            speed: 145.0,
            sensitivity: 0.00015,
        })
        .add_systems(Startup, setup)
        .add_systems(Update, (update_lights, update_lights2, update_player))
        .run();
}

fn setup(mut commands: Commands) {
    craftdebug!(println!("setup"));
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        FlyCam,
        Player,
    ));

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 80_000.,
            ..default()
        },
        transform: Transform::from_xyz(2.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight { ..default() },
        transform: Transform::from_xyz(2.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn update_lights(
    mut light_query: Query<&mut Transform, (Without<PointLight>, With<DirectionalLight>)>,
    time: Res<Time>,
) {
    craftdebug!(println!("update_lights"));
    let elapsed_time = time.elapsed().as_secs_f32();

    for mut transform in light_query.iter_mut() {
        *transform = Transform::from_xyz(f32::sin(elapsed_time), f32::cos(elapsed_time).abs(), 0.)
            .looking_at(Vec3::ZERO, Vec3::Y);
    }
}

fn update_lights2(
    mut point_light_query: Query<&mut Transform, (Without<Player>, With<PointLight>)>,
    player_query: Query<&Transform, (Without<PointLight>, With<Player>)>,
) {
    craftdebug!(println!("update_lights2"));
    let player_transform = player_query.single();

    for mut transform in point_light_query.iter_mut() {
        *transform = *player_transform;
    }
}

fn update_player(
    player_query: Query<&Transform, With<Player>>,
    mut chunk_map: ResMut<ChunkMap>,
    input: Res<Input<MouseButton>>,
) {
    let player_transform = player_query.single();
    if input.pressed(MouseButton::Left) {
        let pos = player_transform.translation;
        let rot = player_transform.rotation;
        let dir = rot * Vec3::new(0.0, 0.0, -1.0);
        if let Some((pos, block)) = chunk_map.collide(
            Ray {
                origin: pos,
                direction: dir,
            },
            500.0,
        ) {
            for x in -4..4 {
                for y in -4..4 {
                    for z in -4..4 {
                        chunk_map.set(&[pos.x as i32 + x, pos.y as i32 + y, pos.z as i32 + z], 0);
                    }
                }
            }
        }
    }
}
