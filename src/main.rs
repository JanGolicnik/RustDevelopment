use bevy::prelude::*;
use bevy_flycam::prelude::*;
use chunks::{ChunkPlugin, chunkmap::ChunkMap, Chunk, chunkqueue::ChunkQueue};

mod chunks;

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, NoCameraPlayerPlugin, ChunkPlugin))
    .insert_resource(MovementSettings {
        speed: 10.0,
        sensitivity: 0.00015,
    })
    .add_systems(Startup, setup)
    .add_systems(Update, (update_lights, update_lights2, update_player))
    .run();
}

fn setup(mut commands: Commands){
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 32.0, 0.5),
            ..default()
        },
        FlyCam,
        Player
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
        point_light: PointLight {
            ..default() 
        },
        transform: Transform::from_xyz(2.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}



fn update_lights(mut light_query: Query<&mut Transform, (Without<PointLight>, With<DirectionalLight>)>, time: Res<Time>){
    let elapsed_time = time.elapsed().as_secs_f32();

    for mut transform in light_query.iter_mut() {
        *transform = Transform::from_xyz(f32::sin(elapsed_time), f32::cos(elapsed_time).abs(), 0.).looking_at(Vec3::ZERO, Vec3::Y);
    }

}

fn update_lights2(mut point_light_query: Query<&mut Transform, (Without<Player>, With<PointLight>)>, player_query: Query<&Transform, (Without<PointLight>, With<Player>)>){
    let player_transform = player_query.single();

    for mut transform in point_light_query.iter_mut() {
        *transform = *player_transform;
    }
}

fn update_player(player_query: Query<&Transform, With<Player>>, mut chunk_q: ResMut<ChunkQueue>, mut chunk_map: ResMut<ChunkMap>, input: Res<Input<KeyCode>>){
    if input.just_pressed(KeyCode::Q){
        let player_transform = player_query.single();

        let translation = player_transform.translation;
        let pos = [translation.x as i32, translation.y as i32, translation.z as i32];
        println!("{:?}", pos);
        chunk_map.set(&pos, true);
        chunk_q.remesh_queue.push(ChunkMap::coords_to_chunk(&pos));
    }

    if input.just_pressed(KeyCode::P){

        let player_transform = player_query.single();
        let translation = player_transform.translation;
        let mut pos = [translation.x as i32, translation.y as i32, translation.z as i32];

        for _ in 0..3{
            chunk_map.set(&pos, true);
            chunk_q.remesh_queue.push(ChunkMap::coords_to_chunk(&pos));
            pos[0] += 1;
        }
    }
}