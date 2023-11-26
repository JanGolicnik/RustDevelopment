use super::blocks::AIR;
use super::chunkgrid::Block;
use super::{chunkgrid::ChunkGrid, Chunk};
use super::{CHUNK_SIZE, HALF_CHUNK_SIZE};
use bevy::utils::HashSet;
use bevy::{prelude::*, utils::HashMap};
use noise::Perlin;

#[derive(Resource)]
pub struct ChunkMap {
    pub chunks: HashMap<Chunk, ChunkGrid>,
    pub entities: HashMap<Chunk, Entity>,
    pub chunks_to_rebuild: HashSet<Chunk>,
}

impl ChunkMap {
    pub fn gen(chunk: &Chunk, noise: &Perlin) -> ChunkGrid {
        ChunkGrid::generate(Self::chunk_to_world_coords(chunk), noise)
    }

    pub fn set(&mut self, coords: &[i32; 3], val: u8) {
        let chunk = Self::coords_to_chunk(coords);
        if let Some(grid) = self.chunks.get_mut(&chunk) {
            let block_coords = Self::coords_to_block_in_chunk(*coords);
            let index = ChunkGrid::pos_to_index(&block_coords);
            grid.0[index] = Block { id: val };
            self.chunks_to_rebuild.insert(chunk);
        }
    }

    pub fn get(&self, coords: &[i32; 3]) -> u8 {
        let chunk = Self::coords_to_chunk(coords);
        if let Some(grid) = self.chunks.get(&chunk) {
            let block_coords = Self::coords_to_block_in_chunk(*coords);
            let index = ChunkGrid::pos_to_index(&block_coords);
            return grid.0[index].id;
        }
        AIR
    }

    pub fn coords_to_chunk(coords: &[i32; 3]) -> Chunk {
        let x = (coords[0] + HALF_CHUNK_SIZE as i32) as f32 / CHUNK_SIZE as f32;
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

        [
            x.unsigned_abs() as usize,
            y.unsigned_abs() as usize,
            z.unsigned_abs() as usize,
        ]
    }

    pub fn chunk_to_world_coords(chunk: &Chunk) -> [i32; 3] {
        [
            chunk.0[0] * CHUNK_SIZE as i32,
            chunk.0[1] * CHUNK_SIZE as i32,
            chunk.0[2] * CHUNK_SIZE as i32,
        ]
    }

    pub fn collide(&self, ray: Ray, length: f32) -> Option<(Vec3, Block)> {
        let mut pos = ray.origin;

        let step_x = if ray.direction.x >= 0.0 { 1.0 } else { -1.0 };
        let step_y = if ray.direction.y >= 0.0 { 1.0 } else { -1.0 };
        let step_z = if ray.direction.z >= 0.0 { 1.0 } else { -1.0 };

        let delta_x = step_x / ray.direction.x.abs();
        let delta_y = step_y / ray.direction.y.abs();
        let delta_z = step_z / ray.direction.z.abs();

        let mut t_max_x = (pos.x + step_x * 0.5 - ray.origin.x) / ray.direction.x;
        let mut t_max_y = (pos.y + step_y * 0.5 - ray.origin.y) / ray.direction.y;
        let mut t_max_z = (pos.z + step_z * 0.5 - ray.origin.z) / ray.direction.z;

        let delta_t_x = delta_x.abs();
        let delta_t_y = delta_y.abs();
        let delta_t_z = delta_z.abs();

        while (pos - ray.origin).length() < length {
            let block = self.get(&[pos.x as i32, pos.y as i32, pos.z as i32]);
            if block > 0 {
                return Some((Vec3::new(pos.x, pos.y, pos.z), Block { id: block }));
            }

            if t_max_x < t_max_y {
                if t_max_x < t_max_z {
                    pos.x += step_x;
                    t_max_x += delta_t_x;
                } else {
                    pos.z += step_z;
                    t_max_z += delta_t_z;
                }
            } else if t_max_y < t_max_z {
                pos.y += step_y;
                t_max_y += delta_t_y;
            } else {
                pos.z += step_z;
                t_max_z += delta_t_z;
            }
        }
        None
    }
}
