use self::{
    chunkmap::ChunkMap,
    chunkqueue::{ChunkCreateQueue, ChunkDespawnQueue, ChunkSpawnQueue},
    material::WorldMaterial,
    systems::{
        create_chunks, create_from_compute, load_resources, setup, spawn_chunks, update_chunks,
    },
};
use bevy::{prelude::*, utils::HashMap};

pub mod blocks;
pub mod chunkgrid;
pub mod chunkmap;
pub mod chunkqueue;
mod material;
mod systems;

pub const CHUNK_SIZE: usize = 48;
pub const HALF_CHUNK_SIZE: usize = CHUNK_SIZE / 2;
pub const CHUNK_VOLUME: usize = CHUNK_SIZE * CHUNK_SIZE * CHUNK_SIZE;

pub const WORLD_SIZE: usize = 10;

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
            .insert_resource(ChunkSpawnQueue(Vec::new()))
            .insert_resource(ChunkDespawnQueue(Vec::new()))
            .insert_resource(ChunkCreateQueue(Vec::new()))
            .insert_resource(ChunkMap {
                chunks: HashMap::new(),
                entities: HashMap::new(),
            })
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    load_resources.run_if(in_state(WorldResourceLoadState::Loading)),
                    (
                        create_from_compute,
                        update_chunks,
                        spawn_chunks.after(update_chunks),
                        create_chunks.after(spawn_chunks),
                    )
                        .run_if(in_state(WorldResourceLoadState::Loaded)),
                ),
            );
    }
}
