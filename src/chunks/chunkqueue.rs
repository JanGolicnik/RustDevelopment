use super::Chunk;
use bevy::prelude::*;

// #[derive(Resource)]
// pub struct ChunkQueue {
//     pub spawn_queue: Vec<Chunk>,
//     pub despawn_queue: Vec<(Entity, Chunk)>,
//     pub remesh_queue: Vec<Chunk>,
//     pub create_queue: Vec<(Chunk, Entity)>,
//     pub spawned_chunks: HashMap<Chunk, Entity>,
// }

#[derive(Resource)]
pub struct ChunkSpawnQueue(pub Vec<Chunk>);

#[derive(Resource)]
pub struct ChunkDespawnQueue(pub Vec<(Entity, Chunk)>);

#[derive(Resource)]
pub struct ChunkRemeshQueue(pub Vec<Chunk>);

#[derive(Resource)]
pub struct ChunkCreateQueue(pub Vec<(Chunk, Entity)>);
