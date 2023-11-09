use bevy::prelude::*;
use crate::Player;
use super::{chunkqueue::ChunkQueue, Chunk, utils::NEIGHBOUR_OFFSETS, chunkmap::ChunkMap, WORLD_SIZE, CHUNK_SIZE, RENDER_DIST};

pub fn spawn_chunks(mut commands: Commands, mut chunk_q: ResMut<ChunkQueue>, mut materials: ResMut<Assets<StandardMaterial>>){
    let spawn_queue = chunk_q.spawn_queue.clone();

    for chunk in &spawn_queue {
        if !chunk_q.spawned_chunks.contains_key(chunk) {
            
            let spawned_chunk_entity = commands.spawn((PbrBundle {
                material: materials.add(Color::rgb(0.35, 0.7, 0.6).into()),
                ..default()
            }, Chunk::from(*chunk))).id();
    
            chunk_q.spawned_chunks.insert(*chunk, spawned_chunk_entity);

            for offset in NEIGHBOUR_OFFSETS {
                let x = chunk.0[0] + offset[0];
                let y = chunk.0[1] + offset[1];
                let z = chunk.0[2] + offset[2];
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
    let transform = player_transform.translation.round();
    let current_chunk = ChunkMap::coords_to_chunk(&[transform.x as i32, transform.y as i32, transform.z as i32]);

    let lower = (WORLD_SIZE as f32 * -0.5).ceil() as i32;
    let upper = (WORLD_SIZE as f32 *  0.5).ceil() as i32;

    for x in lower..upper {
        for y in lower..upper {
            for z in lower..upper {
                let x = x + current_chunk.0[0];
                let y = y + current_chunk.0[1];
                let z = z + current_chunk.0[2];
                chunk_q.spawn_queue.push(Chunk([x,y,z]));
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

pub fn remesh_chunks(mut commands: Commands, chunkmap: Res<ChunkMap>, mut chunk_q: ResMut<ChunkQueue>, mut meshes: ResMut<Assets<Mesh>>){
    for chunk in &chunk_q.remesh_queue {
        if let Some(entity) = chunk_q.spawned_chunks.get(chunk){
            if let Some(mesh) = chunkmap.remesh(&chunk){
                commands.entity(*entity).try_insert(meshes.add(mesh));
            }
        }
    }

    chunk_q.remesh_queue.clear();
}

pub fn regen_chunks(mut chunkmap: ResMut<ChunkMap>, mut chunk_q: ResMut<ChunkQueue>){

    for chunk in &chunk_q.regen_queue {
        chunkmap.regen(&chunk);
    }

    chunk_q.regen_queue.clear();
}
