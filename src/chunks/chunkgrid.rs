use super::{blocks::*, material::ATTRIBUTE_TEXTURE_INDEX, CHUNK_SIZE, CHUNK_VOLUME};
use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
};
use noise::{NoiseFn, Perlin};

const HEIGHT_NOISE_SCALE: f64 = 1.0 / 64.0;
const REGIONAL_HEIGHT_NOISE_SCALE: f64 = 1.0 / 256.0;
const MOUNTAIN_SCALE: f64 = 128.0;

fn sample(noise: &Perlin, pos: [f64; 3]) -> f64 {
    let mut noise_val = 1.0;

    if pos[1] > 0.0 {
        let mut height = noise.get([pos[0] * HEIGHT_NOISE_SCALE, pos[2] * HEIGHT_NOISE_SCALE]);
        height = (height + 1.0) / 2.0;

        let mut regional_height = noise.get([
            pos[0] * REGIONAL_HEIGHT_NOISE_SCALE,
            pos[2] * REGIONAL_HEIGHT_NOISE_SCALE,
        ]);
        regional_height = (regional_height + 1.0) / 2.0;

        noise_val = (pos[1] < height * MOUNTAIN_SCALE * regional_height) as i8 as f64;
    }

    noise_val
}

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

    pub fn generate(pos: [i32; 3], noise: &Perlin) -> ChunkGrid {
        let mut ret = ChunkGrid::new(Block { id: 0 });

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let world_x = pos[0] + x as i32;
                    let world_y = pos[1] + y as i32;
                    let world_z = pos[2] + z as i32;

                    let noise_val_above =
                        sample(noise, [world_x as f64, world_y as f64, world_z as f64]);
                    if noise_val_above > 0.85 {
                        if world_y < 7 {
                            ret.set(x, y, z, Block { id: SAND });
                            continue;
                        }
                        let above_value = sample(
                            noise,
                            [world_x as f64, world_y as f64 + 1.0, world_z as f64],
                        );
                        if above_value > 0.85 {
                            let above_value = sample(
                                noise,
                                [world_x as f64, world_y as f64 + 5.0, world_z as f64],
                            );
                            if above_value > 0.85 {
                                ret.set(x, y, z, Block { id: STONE });
                            } else {
                                ret.set(x, y, z, Block { id: DIRT });
                            }
                            continue;
                        }
                        ret.set(x, y, z, Block { id: GRASS });
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

        enum BlockSide {
            X,
            Y,
            Z,
        }

        let mut add_plane = |x: f32, y: f32, z: f32, side: BlockSide, flip: bool, block: u8| {
            let i = positions.len() as u32;

            let mut verts: [[f32; 3]; 4] = [[x, y, z]; 4];
            let offset: usize;
            let mut normal: [f32; 3];
            let texture_offset: u32;

            match side {
                BlockSide::X => {
                    offset = 0;
                    normal = [1., 0., 0.];
                    texture_offset = 1;
                    uvs.append(&mut vec![[0., 0.], [1., 0.], [0., 1.], [1., 1.]]);
                }
                BlockSide::Y => {
                    offset = 1;
                    normal = [0., 1., 0.];
                    if flip {
                        texture_offset = 2;
                    } else {
                        texture_offset = 0;
                    }
                    uvs.append(&mut vec![[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
                }
                BlockSide::Z => {
                    offset = 2;
                    normal = [0., 0., 1.];
                    texture_offset = 1;
                    uvs.append(&mut vec![[0., 0.], [0., 1.], [1., 0.], [1., 1.]]);
                }
            }

            verts[0][0] -= 0.5;
            verts[0][1] -= 0.5;
            verts[0][2] -= 0.5;
            if !flip {
                verts[0][offset] += 1.0;
            }

            verts[1] = verts[0];
            verts[1][(offset + 2) % 3] += 1.0;

            verts[2] = verts[0];
            verts[2][(offset + 1) % 3] += 1.0;

            verts[3] = verts[0];
            verts[3][(offset + 1) % 3] += 1.0;
            verts[3][(offset + 2) % 3] += 1.0;

            if flip {
                normal[0] = -normal[0];
                normal[1] = -normal[1];
                normal[2] = -normal[2];
                indices.append(&mut vec![i, i + 1, i + 3, i, i + 3, i + 2]);
            } else {
                indices.append(&mut vec![i, i + 3, i + 1, i, i + 2, i + 3]);
            }

            positions.push(verts[0]);
            positions.push(verts[1]);
            positions.push(verts[2]);
            positions.push(verts[3]);
            normals.append(&mut vec![normal; 4]);

            texture_indices.append(&mut vec![block as u32 + texture_offset; 4]);
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
                                    add_plane(fx, fy, fz, BlockSide::X, false, block_id);
                                }
                            }
                        } else if !self.is_filled(x + 1, y, z) {
                            add_plane(fx, fy, fz, BlockSide::X, false, block_id);
                        }

                        if x == 0 {
                            if let Some(grid) = neighbours[1] {
                                if !grid.is_filled(CHUNK_SIZE - 1, y, z) {
                                    add_plane(fx, fy, fz, BlockSide::X, true, block_id);
                                }
                            }
                        } else if !self.is_filled(x - 1, y, z) {
                            add_plane(fx, fy, fz, BlockSide::X, true, block_id);
                        }

                        if y == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[2] {
                                if !grid.is_filled(x, 0, z) {
                                    add_plane(fx, fy, fz, BlockSide::Y, false, block_id);
                                }
                            }
                        } else if !self.is_filled(x, y + 1, z) {
                            add_plane(fx, fy, fz, BlockSide::Y, false, block_id);
                        }

                        if y == 0 {
                            if let Some(grid) = neighbours[3] {
                                if !grid.is_filled(x, CHUNK_SIZE - 1, z) {
                                    add_plane(fx, fy, fz, BlockSide::Y, true, block_id);
                                }
                            }
                        } else if !self.is_filled(x, y - 1, z) {
                            add_plane(fx, fy, fz, BlockSide::Y, true, block_id);
                        }

                        if z == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[4] {
                                if !grid.is_filled(x, y, 0) {
                                    add_plane(fx, fy, fz, BlockSide::Z, false, block_id);
                                }
                            }
                        } else if !self.is_filled(x, y, z + 1) {
                            add_plane(fx, fy, fz, BlockSide::Z, false, block_id);
                        }

                        if z == 0 {
                            if let Some(grid) = neighbours[5] {
                                if !grid.is_filled(x, y, CHUNK_SIZE - 1) {
                                    add_plane(fx, fy, fz, BlockSide::Z, true, block_id);
                                }
                            }
                        } else if !self.is_filled(x, y, z - 1) {
                            add_plane(fx, fy, fz, BlockSide::Z, true, block_id);
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
