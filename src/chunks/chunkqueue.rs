use bevy::{prelude::*, utils::HashMap};
use super::Chunk;

#[derive(Resource)]
pub struct ChunkQueue{
    pub spawn_queue: Vec<Chunk>,
    pub despawn_queue: Vec<(Entity, Chunk)>,
    pub remesh_queue: Vec<Chunk>,
    pub regen_queue: Vec<Chunk>,
    pub spawned_chunks: HashMap<Chunk, Entity>,
}