use super::Chunk;
use bevy::prelude::*;

#[derive(Resource)]
pub struct ChunkSpawnQueue(pub Vec<Chunk>);

#[derive(Resource)]
pub struct ChunkDespawnQueue(pub Vec<(Entity, Chunk)>);

#[derive(Resource)]
pub struct ChunkCreateQueue(pub Vec<(Chunk, Entity)>);
