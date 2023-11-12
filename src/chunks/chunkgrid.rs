use super::{blocks::*, material::ATTRIBUTE_TEXTURE_INDEX, Chunk, CHUNK_SIZE, CHUNK_VOLUME};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

const NOISE_SCALE: f64 = 1.0 / 32.0;

#[derive(Debug, Copy, Clone)]
pub struct Block {
    pub id: u8,
}

#[derive(Debug)]
pub struct ChunkGrid(pub [Block; CHUNK_VOLUME]);

impl ChunkGrid {
    pub fn new(val: Block) -> Self {
        ChunkGrid([val; CHUNK_VOLUME])
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, val: Block) {
        self.0[Self::pos_to_index(&[x, y, z])] = val;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> Block {
        self.0[Self::pos_to_index(&[x, y, z])]
    }

    pub fn is_filled(&self, x: usize, y: usize, z: usize) -> bool {
        self.get(x, y, z).id > AIR
    }

    pub fn pos_to_index(pos: &[usize; 3]) -> usize {
        pos[0] + pos[1] * CHUNK_SIZE + pos[2] * CHUNK_SIZE * CHUNK_SIZE
    }

    pub fn generate(pos: [i32; 3]) -> ChunkGrid {
        let mut ret = ChunkGrid::new(Block { id: 0 });

        let noise = Perlin::new(5);

        const NOISE_SIZE: usize = CHUNK_SIZE + 2;
        const NOISE_VOLUME: usize = NOISE_SIZE * NOISE_SIZE * NOISE_SIZE;

        let pos_to_noise_index =
            |x: usize, y: usize, z: usize| x + y * NOISE_SIZE + z * NOISE_SIZE * NOISE_SIZE;

        let mut volume_map: [f64; NOISE_VOLUME] = [0.; NOISE_VOLUME];

        for x in 0..NOISE_SIZE {
            for y in 0..NOISE_SIZE {
                for z in 0..NOISE_SIZE {
                    let world_x = (pos[0] as f64 + x as f64 - 1.0) * NOISE_SCALE;
                    let world_y = (pos[1] as f64 + y as f64 - 1.0) * NOISE_SCALE;
                    let world_z = (pos[2] as f64 + z as f64 - 1.0) * NOISE_SCALE;

                    let mut noise_val = 1.0;

                    if world_y > 0.0 {
                        noise_val = noise.get([world_x, world_y, world_z]);
                        noise_val = (noise_val + 1.0) / 2.0;
                        noise_val *= f64::log(world_y * 50.0 + 1.0, 100.0).powf(-1.0);
                    }

                    volume_map[pos_to_noise_index(x, y, z)] = noise_val;
                }
            }
        }

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let world_x = pos[0] + x as i32;
                    let world_y = pos[1] + y as i32;
                    let world_z = pos[2] + z as i32;

                    let noise_val_above = volume_map[pos_to_noise_index(x + 1, y + 1, z + 1)];
                    if noise_val_above > 0.85 {
                        if world_y < 7 {
                            ret.set(x, y, z, Block { id: SAND });
                            continue;
                        }
                        let noise_val_above = volume_map[pos_to_noise_index(x + 1, y + 2, z + 1)];
                        if noise_val_above > 0.85 {
                            ret.set(x, y, z, Block { id: GRASS });
                            continue;
                        }
                        ret.set(x, y, z, Block { id: DIRT });
                    }
                }
            }
        }

        ret
    }

    pub fn to_mesh(&self, pos: &[i32; 3], neighbours: &[Option<&ChunkGrid>; 6]) -> Mesh {
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();
        let mut texture_indices: Vec<u32> = Vec::new();

        let mut add_plane =
            |x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32, flip: bool, block: u8| {
                let i = positions.len() as u32;
                if x0 == x1 {
                    positions.push([x0, y0, z0]);
                    positions.push([x0, y1, z0]);
                    positions.push([x0, y0, z1]);
                    positions.push([x1, y1, z1]);

                    if flip {
                        normals.append(&mut vec![[-1., 0., 0.]; 4]);
                    } else {
                        normals.append(&mut vec![[1., 0., 0.]; 4]);
                    }
                }

                if y0 == y1 {
                    positions.push([x0, y0, z0]);
                    positions.push([x0, y0, z1]);
                    positions.push([x1, y0, z0]);
                    positions.push([x1, y1, z1]);

                    if flip {
                        normals.append(&mut vec![[0., -1., 0.]; 4]);
                    } else {
                        normals.append(&mut vec![[0., 1., 0.]; 4]);
                    }
                }

                if z0 == z1 {
                    positions.push([x0, y0, z0]);
                    positions.push([x1, y0, z1]);
                    positions.push([x0, y1, z0]);
                    positions.push([x1, y1, z1]);

                    if flip {
                        normals.append(&mut vec![[0., 0., -1.]; 4]);
                    } else {
                        normals.append(&mut vec![[0., 0., 1.]; 4]);
                    }
                }

                uvs.append(&mut vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]]);
                texture_indices.append(&mut vec![block as u32; 4]);

                if flip {
                    indices.append(&mut vec![i, i + 3, i + 1, i, i + 2, i + 3]);
                } else {
                    indices.append(&mut vec![i, i + 1, i + 3, i, i + 3, i + 2]);
                }
            };

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let block_id = self.get(x, y, z).id;
                    if self.is_filled(x, y, z) {
                        let fx = pos[0] as f32 + x as f32 - CHUNK_SIZE as f32 * 0.5;
                        let fy = pos[1] as f32 + y as f32 - CHUNK_SIZE as f32 * 0.5;
                        let fz = pos[2] as f32 + z as f32 - CHUNK_SIZE as f32 * 0.5;

                        // dont look at this pls <3
                        if x == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[0] {
                                if !grid.is_filled(0, y, z) {
                                    add_plane(
                                        fx + 0.5,
                                        fy - 0.5,
                                        fz - 0.5,
                                        fx + 0.5,
                                        fy + 0.5,
                                        fz + 0.5,
                                        false,
                                        block_id,
                                    );
                                }
                            }
                        } else if !self.is_filled(x + 1, y, z) {
                            add_plane(
                                fx + 0.5,
                                fy - 0.5,
                                fz - 0.5,
                                fx + 0.5,
                                fy + 0.5,
                                fz + 0.5,
                                false,
                                block_id,
                            );
                        }

                        if x == 0 {
                            if let Some(grid) = neighbours[1] {
                                if !grid.is_filled(CHUNK_SIZE - 1, y, z) {
                                    add_plane(
                                        fx - 0.5,
                                        fy - 0.5,
                                        fz - 0.5,
                                        fx - 0.5,
                                        fy + 0.5,
                                        fz + 0.5,
                                        true,
                                        block_id,
                                    );
                                }
                            }
                        } else if !self.is_filled(x - 1, y, z) {
                            add_plane(
                                fx - 0.5,
                                fy - 0.5,
                                fz - 0.5,
                                fx - 0.5,
                                fy + 0.5,
                                fz + 0.5,
                                true,
                                block_id,
                            );
                        }

                        if y == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[2] {
                                if !grid.is_filled(x, 0, z) {
                                    add_plane(
                                        fx - 0.5,
                                        fy + 0.5,
                                        fz - 0.5,
                                        fx + 0.5,
                                        fy + 0.5,
                                        fz + 0.5,
                                        false,
                                        block_id,
                                    );
                                }
                            }
                        } else if !self.is_filled(x, y + 1, z) {
                            add_plane(
                                fx - 0.5,
                                fy + 0.5,
                                fz - 0.5,
                                fx + 0.5,
                                fy + 0.5,
                                fz + 0.5,
                                false,
                                block_id,
                            );
                        }

                        if y == 0 {
                            if let Some(grid) = neighbours[3] {
                                if !grid.is_filled(x, CHUNK_SIZE - 1, z) {
                                    add_plane(
                                        fx - 0.5,
                                        fy - 0.5,
                                        fz - 0.5,
                                        fx + 0.5,
                                        fy - 0.5,
                                        fz + 0.5,
                                        true,
                                        block_id,
                                    );
                                }
                            }
                        } else if !self.is_filled(x, y - 1, z) {
                            add_plane(
                                fx - 0.5,
                                fy - 0.5,
                                fz - 0.5,
                                fx + 0.5,
                                fy - 0.5,
                                fz + 0.5,
                                true,
                                block_id,
                            );
                        }

                        if z == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[4] {
                                if !grid.is_filled(x, y, 0) {
                                    add_plane(
                                        fx - 0.5,
                                        fy - 0.5,
                                        fz + 0.5,
                                        fx + 0.5,
                                        fy + 0.5,
                                        fz + 0.5,
                                        false,
                                        block_id,
                                    );
                                }
                            }
                        } else if !self.is_filled(x, y, z + 1) {
                            add_plane(
                                fx - 0.5,
                                fy - 0.5,
                                fz + 0.5,
                                fx + 0.5,
                                fy + 0.5,
                                fz + 0.5,
                                false,
                                block_id,
                            );
                        }

                        if z == 0 {
                            if let Some(grid) = neighbours[5] {
                                if !grid.is_filled(x, y, CHUNK_SIZE - 1) {
                                    add_plane(
                                        fx - 0.5,
                                        fy - 0.5,
                                        fz - 0.5,
                                        fx + 0.5,
                                        fy + 0.5,
                                        fz - 0.5,
                                        true,
                                        block_id,
                                    );
                                }
                            }
                        } else if !self.is_filled(x, y, z - 1) {
                            add_plane(
                                fx - 0.5,
                                fy - 0.5,
                                fz - 0.5,
                                fx + 0.5,
                                fy + 0.5,
                                fz - 0.5,
                                true,
                                block_id,
                            );
                        }
                    }
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(ATTRIBUTE_TEXTURE_INDEX, texture_indices);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
