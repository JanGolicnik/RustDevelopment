use self::{
    chunkmap::ChunkMap,
    chunkqueue::ChunkQueue,
    material::WorldMaterial,
    systems::{gen_chunks, load_resources, remesh_chunks, setup, spawn_chunks, update_chunks},
};
use bevy::{prelude::*, utils::HashMap};

pub mod blocks;
pub mod chunkgrid;
pub mod chunkmap;
pub mod chunkqueue;
mod material;
mod systems;
mod utils;

pub const CHUNK_SIZE: usize = 32;
pub const HALF_CHUNK_SIZE: usize = CHUNK_SIZE / 2;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_SIZE: usize = 6;

const RENDER_DIST: f32 = WORLD_SIZE as f32 * CHUNK_SIZE as f32 * 2.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash, States, Default)]
pub enum WorldResourceLoadState {
    #[default]
    Loading,
    Loaded,
}

#[derive(Component, Clone, Eq, PartialEq, Hash, Copy, Debug)]
pub struct Chunk(pub [i32; 3]);
pub struct ChunkPlugin;

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<WorldMaterial>::default())
            .add_state::<WorldResourceLoadState>()
            .insert_resource(ChunkQueue {
                spawn_queue: Vec::new(),
                despawn_queue: Vec::new(),
                remesh_queue: Vec::new(),
                gen_queue: Vec::new(),
                spawned_chunks: HashMap::new(),
            })
            .insert_resource(ChunkMap {
                chunks: HashMap::new(),
            })
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    load_resources.run_if(in_state(WorldResourceLoadState::Loading)),
                    (
                        update_chunks,
                        spawn_chunks.after(update_chunks),
                        gen_chunks.after(spawn_chunks),
                        remesh_chunks.after(gen_chunks),
                    )
                        .run_if(in_state(WorldResourceLoadState::Loaded)),
                ),
            );
    }
}
