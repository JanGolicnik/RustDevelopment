use bevy::{prelude::*, utils::HashMap};

use crate::Player;

use super::{Chunk, utils::NEIGHBOUR_OFFSETS, RENDER_DIST};
use super::{WORLD_SIZE, CHUNK_SIZE};

#[derive(Resource)]
pub struct ChunkQueue{
    pub spawn_queue: Vec<Chunk>,
    pub despawn_queue: Vec<(Entity, Chunk)>,
    pub remesh_queue: Vec<Chunk>,
    pub regen_queue: Vec<Chunk>,
    pub spawned_chunks: HashMap<Chunk, Entity>,
}

pub fn spawn_chunks(mut commands: Commands, mut chunk_q: ResMut<ChunkQueue>, mut materials: ResMut<Assets<StandardMaterial>>){
    let spawn_queue = chunk_q.spawn_queue.clone();

    for c in &spawn_queue {
        let x = c.0[0] as i32;
        let y = c.0[1] as i32;
        let z = c.0[2] as i32;

        if !chunk_q.spawned_chunks.contains_key(&Chunk([x, y, z])) {
            
            let spawned_chunk_entity = commands.spawn((PbrBundle {
                material: materials.add(Color::rgb(0.35, 0.7, 0.6).into()),
                ..default()
            }, Chunk([x, y, z]))).id();
    
            chunk_q.spawned_chunks.insert(Chunk([x, y, z]), spawned_chunk_entity);

            for offset in NEIGHBOUR_OFFSETS {
                let x = x + offset[0];
                let y = y + offset[1];
                let z = z + offset[2];
                let chunk = Chunk([x, y, z]);
                if chunk_q.spawned_chunks.contains_key(&chunk) {
                    chunk_q.regen_queue.push(chunk);
                    chunk_q.remesh_queue.push(chunk);
                }
            }
        }
    }

    for (entity, chunk) in chunk_q.despawn_queue.clone() {
        commands.entity(entity).despawn();
        chunk_q.spawned_chunks.remove(&chunk);
    }

    chunk_q.spawn_queue.clear();
    chunk_q.despawn_queue.clear();
}

pub fn update_chunks(mut chunk_q: ResMut<ChunkQueue>, player_query: Query<&Transform, With<Player>>, chunk_query: Query<(&Chunk, Entity), (With<Chunk>, Without<Player>)>) {

    let player_transform = player_query.single();

    let lower = (WORLD_SIZE as f32 * -0.5).ceil() as i32;
    let upper = (WORLD_SIZE as f32 * 0.5).ceil() as i32;

    for x in lower..upper {
        for y in lower..upper {
            for z in lower..upper {

                let inv_chunk_size = 1.0 / CHUNK_SIZE as f32;

                let x = x as f32 + player_transform.translation.x * inv_chunk_size; 
                let y = y as f32 + player_transform.translation.y * inv_chunk_size; 
                let z = z as f32 + player_transform.translation.z * inv_chunk_size; 

                chunk_q.spawn_queue.push(Chunk([x as i32, y as i32, z as i32]));
            }
        }
    }

    for (chunk, entity) in chunk_query.iter(){
        if chunk_q.spawn_queue.contains(&chunk) 
        {
            continue
        }

        let chunk_world_x = chunk.0[0] as f32 * CHUNK_SIZE as f32;
        let chunk_world_y = chunk.0[1] as f32 * CHUNK_SIZE as f32;
        let chunk_world_z = chunk.0[2] as f32 * CHUNK_SIZE as f32;

        let dist_x = chunk_world_x - player_transform.translation.x; 
        let dist_y = chunk_world_y - player_transform.translation.y; 
        let dist_z = chunk_world_z - player_transform.translation.z; 
        
        if  dist_x.abs() > RENDER_DIST ||
            dist_y.abs() > RENDER_DIST ||
            dist_z.abs() > RENDER_DIST {

            chunk_q.despawn_queue.push((entity, *chunk));
        }

    }

}
