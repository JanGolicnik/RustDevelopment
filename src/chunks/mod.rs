use bevy::{prelude::*, utils::HashMap};

use self::{chunkmap::{remesh_chunks, ChunkMap, regen_chunks}, chunkqueue::{ChunkQueue, update_chunks, spawn_chunks}};

pub mod chunkmap;
pub mod chunkqueue;
mod utils;
mod generation;

pub const CHUNK_SIZE: usize = 16;
pub const HALF_CHUNK_SIZE: usize = CHUNK_SIZE / 2;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_SIZE: usize = 10;

const RENDER_DIST: f32 = WORLD_SIZE as f32 * 32.;

#[derive(Component, Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub struct Chunk(pub [i32; 3]);
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(ChunkQueue {
                spawn_queue: Vec::new(),
                despawn_queue: Vec::new(),
                remesh_queue: Vec::new(),
                regen_queue: Vec::new(),
                spawned_chunks: HashMap::new()})
                .insert_resource(ChunkMap {
                    chunks: HashMap::new()})
            .add_systems(Update, 
                (
                    update_chunks,
                    spawn_chunks.after(update_chunks),
                    regen_chunks.after(spawn_chunks),
                    remesh_chunks.after(regen_chunks),
                )
            );
    }
}
