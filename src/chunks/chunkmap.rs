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

            let mut x = chunk.0[0];
            let mut y = chunk.0[1];
            let mut z = chunk.0[2];

            let neighbours: [Option<&ChunkGrid>; 6] = [
                self.chunks.get(&Chunk([x + 1, y, z])),
                self.chunks.get(&Chunk([x - 1, y, z])),
                self.chunks.get(&Chunk([x, y + 1, z])),
                self.chunks.get(&Chunk([x, y - 1, z])),
                self.chunks.get(&Chunk([x, y, z + 1])),
                self.chunks.get(&Chunk([x, y, z - 1])),
            ];

            x *= CHUNK_SIZE as i32;
            y *= CHUNK_SIZE as i32;
            z *= CHUNK_SIZE as i32;

            return Some(chunkgrid.to_mesh(x as f32, y as f32, z as f32, &neighbours));
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
            let block_coords = Self::coords_to_block_in_chunk(coords);
            let index =ChunkGrid::pos_to_index(&block_coords); 
            println!("set on index {}", index);
            grid.0[index] = val;
        }
    }

    pub fn get(&self, coords: &[i32; 3]) -> bool {
        let chunk = Self::coords_to_chunk(coords);
        if let Some(grid) = self.chunks.get(&chunk){
            let block_coords = Self::coords_to_block_in_chunk(coords);
            let index = ChunkGrid::pos_to_index(&block_coords); 
            return grid.0[index];
        }
        return false;
    }

    pub fn coords_to_chunk(coords: &[i32; 3]) -> Chunk {
        let x = coords[0] / CHUNK_SIZE as i32;
        let y = coords[1] / CHUNK_SIZE as i32;
        let z = coords[2] / CHUNK_SIZE as i32;
        Chunk([x, y, z])
    }

    fn coords_to_block_in_chunk(coords: &[i32; 3]) -> [usize; 3] {
        let x = (coords[0] + HALF_CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let y = (coords[1] + HALF_CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let z = (coords[2] + HALF_CHUNK_SIZE as i32) % CHUNK_SIZE as i32;
        let x = x.abs() as usize;
        let y = y.abs() as usize;
        let z = z.abs() as usize;
        [x, y, z]
    }
}

pub fn remesh_chunks(mut commands: Commands, chunkmap: Res<ChunkMap>, mut chunk_q: ResMut<ChunkQueue>, mut meshes: ResMut<Assets<Mesh>>){
    for chunk in &chunk_q.remesh_queue {
        let x = chunk.0[0] as i32;
        let y = chunk.0[1] as i32;
        let z = chunk.0[2] as i32;
        let chunk = Chunk([x,y,z]);
        if let Some(entity) = chunk_q.spawned_chunks.get(&chunk){
            if let Some(mesh) = chunkmap.remesh(&chunk){
                commands.entity(*entity).try_insert(meshes.add(mesh));
            }
        }
    }

    chunk_q.remesh_queue.clear();
}

pub fn regen_chunks(mut chunkmap: ResMut<ChunkMap>, mut chunk_q: ResMut<ChunkQueue>){

    for chunk in &chunk_q.regen_queue {
        let x = chunk.0[0] as i32;
        let y = chunk.0[1] as i32;
        let z = chunk.0[2] as i32;
        let chunk = Chunk([x,y,z]);

        chunkmap.regen(&chunk);
    }

    chunk_q.regen_queue.clear();
}
