use super::Chunk;
use bevy::{prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct ChunkQueue {
    pub spawn_queue: Vec<Chunk>,
    pub despawn_queue: Vec<(Entity, Chunk)>,
    pub remesh_queue: Vec<Chunk>,
    pub gen_queue: Vec<Chunk>,
    pub spawned_chunks: HashMap<Chunk, Entity>,
}
