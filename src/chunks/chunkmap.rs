use bevy::{prelude::*, utils::HashMap};

use super::{generation::ChunkGrid, Chunk, chunkqueue::ChunkQueue};
use super::{CHUNK_SIZE, HALF_CHUNK_SIZE};

#[derive(Resource)]
pub struct ChunkMap {
    pub chunks: HashMap<Chunk, ChunkGrid>, 
}

impl ChunkMap {
    fn remesh(&self, chunk: &Chunk) -> Option<Mesh> {
        if let Some(chunkgrid) = self.chunks.get(chunk){

            let x = chunk.0[0];
            let y = chunk.0[1];
            let z = chunk.0[2];

            let neighbours: [Option<&ChunkGrid>; 6] = [
                self.chunks.get(&Chunk([x + 1, y, z])),
                self.chunks.get(&Chunk([x - 1, y, z])),
                self.chunks.get(&Chunk([x, y + 1, z])),
                self.chunks.get(&Chunk([x, y - 1, z])),
                self.chunks.get(&Chunk([x, y, z + 1])),
                self.chunks.get(&Chunk([x, y, z - 1])),
            ];

            return Some(chunkgrid.to_mesh(&Self::chunk_to_world_coords(chunk), &neighbours));
        }
        None
    }

    fn regen(&mut self, chunk: &Chunk) {
        if !self.chunks.contains_key(chunk){
            self.chunks.insert(*chunk, ChunkGrid::new(false));
        }
        if let Some(chunkgrid) = self.chunks.get_mut(chunk){
            chunkgrid.generate(chunk.0[0], chunk.0[1], chunk.0[2]);
        }
    }
    
    pub fn set(&mut self, coords: &[i32; 3], val: bool) {
        let chunk = Self::coords_to_chunk(coords);
        if let Some(grid) = self.chunks.get_mut(&chunk){
            let block_coords = Self::coords_to_block_in_chunk(*coords);
            let index = ChunkGrid::pos_to_index(&block_coords); 
            grid.0[index] = val;
        }
    }

    #[allow(dead_code)]
    pub fn get(&self, coords: &[i32; 3]) -> bool {
        let chunk = Self::coords_to_chunk(coords);
        if let Some(grid) = self.chunks.get(&chunk){
            let block_coords = Self::coords_to_block_in_chunk(*coords);
            let index = ChunkGrid::pos_to_index(&block_coords); 
            return grid.0[index];
        }
        return false;
    }

    pub fn coords_to_chunk(coords: &[i32; 3]) -> Chunk {
        let x = (coords[0] + HALF_CHUNK_SIZE as i32) as f32/ CHUNK_SIZE as f32;
        let y = (coords[1] + HALF_CHUNK_SIZE as i32) as f32 / CHUNK_SIZE as f32;
        let z = (coords[2] + HALF_CHUNK_SIZE as i32) as f32 / CHUNK_SIZE as f32;
        Chunk([x.floor() as i32, y.floor() as i32, z.floor() as i32])
    }

    pub fn coords_to_block_in_chunk(mut coords: [i32; 3]) -> [usize; 3] {
        let chunk = Self::coords_to_chunk(&coords);
        coords[0] += chunk.0[0].abs() * CHUNK_SIZE as i32;
        coords[1] += chunk.0[1].abs() * CHUNK_SIZE as i32;
        coords[2] += chunk.0[2].abs() * CHUNK_SIZE as i32;
        let x = (coords[0] + HALF_CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let y = (coords[1] + HALF_CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let z = (coords[2] + HALF_CHUNK_SIZE as i32) % CHUNK_SIZE as i32;

        [x.abs() as usize, y.abs() as usize, z.abs() as usize]
    }

    pub fn chunk_to_world_coords(chunk: &Chunk) -> [i32; 3]{
        [chunk.0[0] * CHUNK_SIZE as i32, chunk.0[1] * CHUNK_SIZE as i32, chunk.0[2] * CHUNK_SIZE as i32]
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
