use bevy::{render::{render_resource::PrimitiveTopology, mesh::Indices}, prelude::*};
use noise::{Perlin, NoiseFn};

use super::{CHUNK_SIZE, CHUNK_VOLUME};

#[derive(Debug)]
pub struct ChunkGrid(pub [bool; CHUNK_VOLUME]);

impl ChunkGrid{
    pub fn new(val: bool) -> Self{
        ChunkGrid([val; CHUNK_VOLUME])
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, val: bool) {
        self.0[Self::pos_to_index(&[x, y, z])] = val;
    }

    pub fn get(&self, x: usize, y: usize, z: usize) -> bool {
        self.0[Self::pos_to_index(&[x, y, z])]
    }

    pub fn pos_to_index(pos: &[usize; 3]) -> usize {
        pos[0] + pos[1] * CHUNK_SIZE + pos[2] * CHUNK_SIZE * CHUNK_SIZE
    }
        
    pub fn generate(&mut self, mut position_x: i32, mut position_y: i32, mut position_z: i32){
        position_x *= CHUNK_SIZE as i32;
        position_y *= CHUNK_SIZE as i32;
        position_z *= CHUNK_SIZE as i32;

        let noise = Perlin::new(5);

        let mut volume_map: [f64; CHUNK_VOLUME ] = [0.; CHUNK_VOLUME]; 

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let fx = position_x as f64 + x as f64; 
                    let fy = position_y as f64 + y as f64; 
                    let fz = position_z as f64 + z as f64;
                    
                    let noise_val = noise.get([fx as f64 / 16., fy as f64 / 16., fz as f64 / 16.]);
                    volume_map[ChunkGrid::pos_to_index(&[x,y,z])] = (noise_val + 1.) / 2.;
                }
            }
        }

        for x in 0..CHUNK_SIZE {
            for y in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    let volume = volume_map[ChunkGrid::pos_to_index(&[x,y,z])];
                    if volume > 0.85 {
                    self.set(x, y, z, true);
                }
                }
            }
        }
    }

    pub fn to_mesh(&self, pos: &[i32; 3], neighbours: &[Option<&ChunkGrid>; 6]) -> Mesh {
       
        let mut positions: Vec<[f32; 3]> = Vec::new();
        let mut normals: Vec<[f32; 3]> = Vec::new();
        let mut uvs: Vec<[f32; 2]> = Vec::new();
        let mut indices: Vec<u32> = Vec::new();

        let mut add_plane = |x0: f32, y0: f32, z0: f32, x1: f32, y1: f32, z1: f32, flip: bool| {
            
            if x0 == x1 {
                positions.push([x0, y0, z0]);
                positions.push([x0, y0, z1]);
                positions.push([x1, y1, z1]);

                positions.push([x0, y0, z0]);
                positions.push([x0, y1, z0]);
                positions.push([x1, y1, z1]);

                uvs.append(&mut vec![[0., 0.]; 6]);

                let i = indices.len() as u32;
                if flip {
                    normals.append(&mut vec![[-1., 0., 0.]; 6]);
                    indices.append(&mut vec![i, i + 1, i + 2, i + 3, i + 5, i + 4]);
                } else {
                    normals.append(&mut vec![[1., 0., 0.]; 6]);
                    indices.append(&mut vec![i, i + 2, i + 1, i + 3, i + 4, i + 5]);
                }
            }
            
            if y0 == y1 {
                positions.push([x0, y0, z0]);
                positions.push([x1, y0, z0]);
                positions.push([x1, y1, z1]);

                positions.push([x0, y0, z0]);
                positions.push([x0, y0, z1]);
                positions.push([x1, y1, z1]);

                uvs.append(&mut vec![[0., 0.]; 6]);

                let i = indices.len() as u32;
                if flip {
                    normals.append(&mut vec![[0., -1., 0.]; 6]);
                    indices.append(&mut vec![i, i + 1, i + 2, i + 3, i + 5, i + 4]);
                } else {
                    normals.append(&mut vec![[0., 1., 0.]; 6]);
                    indices.append(&mut vec![i, i + 2, i + 1, i + 3, i + 4, i + 5]);
                }
            }

            if z0 == z1 {
                positions.push([x0, y0, z0]);
                positions.push([x0, y1, z0]);
                positions.push([x1, y1, z1]);

                positions.push([x0, y0, z0]);
                positions.push([x1, y0, z1]);
                positions.push([x1, y1, z1]);

                uvs.append(&mut vec![[0., 0.]; 6]);

                let i = indices.len() as u32;
                if flip {
                    normals.append(&mut vec![[0., 0., -1.]; 6]);
                    indices.append(&mut vec![i, i + 1, i + 2, i + 3, i + 5, i + 4]);
                } else {
                    normals.append(&mut vec![[0., 0., 1.]; 6]);
                    indices.append(&mut vec![i, i + 2, i + 1, i + 3, i + 4, i + 5]);
                }
            }

        };

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                for z in 0..CHUNK_SIZE {
                    if self.get(x, y, z) {
                        let fx = pos[0] as f32 + x as f32 - CHUNK_SIZE as f32 * 0.5; 
                        let fy = pos[1] as f32 + y as f32 - CHUNK_SIZE as f32 * 0.5;
                        let fz = pos[2] as f32 + z as f32 - CHUNK_SIZE as f32 * 0.5;

                        // dont look at this pls <3
                        if x == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[0]{
                                if !grid.get(0, y, z) {
                                    add_plane(  fx + 0.5, fy - 0.5, fz - 0.5,
                                        fx + 0.5, fy + 0.5, fz + 0.5, false);
                                }
                            }
                        }
                        else if !self.get(x + 1, y, z) {
                            add_plane(  fx + 0.5, fy - 0.5, fz - 0.5,
                                fx + 0.5, fy + 0.5, fz + 0.5, false);
                        }

                        if x == 0 {
                            if let Some(grid) = neighbours[1]{
                                if !grid.get(CHUNK_SIZE - 1, y, z) {
                                    add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                                        fx - 0.5, fy + 0.5, fz + 0.5, true);
                                }
                            }
                        }
                        else if !self.get(x - 1, y, z) {
                            add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                                fx - 0.5, fy + 0.5, fz + 0.5, true);
                        }

                        if y == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[2]{
                                if !grid.get(x, 0, z) {
                                    add_plane(  fx - 0.5, fy + 0.5, fz - 0.5,
                                        fx + 0.5, fy + 0.5, fz + 0.5, false);
                                }
                            }
                        }
                        else if !self.get(x, y + 1, z) {
                            add_plane(  fx - 0.5, fy + 0.5, fz - 0.5,
                                fx + 0.5, fy + 0.5, fz + 0.5, false);
                        }

                        if y == 0 {
                            if let Some(grid) = neighbours[3]{
                                if !grid.get(x, CHUNK_SIZE - 1, z) {
                                    add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                                        fx + 0.5, fy - 0.5, fz + 0.5, true);
                                }
                            }
                        }
                        else if !self.get(x, y - 1, z) {
                            add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                                fx + 0.5, fy - 0.5, fz + 0.5, true);
                        }

                        if z == CHUNK_SIZE - 1 {
                            if let Some(grid) = neighbours[4]{
                                if !grid.get(x, y, 0) {
                                    add_plane(  fx - 0.5, fy - 0.5, fz + 0.5,
                                        fx + 0.5, fy + 0.5, fz + 0.5, false);
                                }
                            }
                        }
                        else if !self.get(x, y, z + 1) {
                            add_plane(  fx - 0.5, fy - 0.5, fz + 0.5,
                                fx + 0.5, fy + 0.5, fz + 0.5, false);
                        }

                        if z == 0 {
                            if let Some(grid) = neighbours[5]{
                                if !grid.get(x, y, CHUNK_SIZE - 1) {
                                    add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                                        fx + 0.5, fy + 0.5, fz - 0.5, true);
                                }
                            }
                        }
                        else if !self.get(x, y, z - 1) {
                            add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                                fx + 0.5, fy + 0.5, fz - 0.5, true);
                        }
                    }
                }
            }
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            positions,
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            normals,
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            uvs,
        );
        mesh.set_indices(Some(Indices::U32(indices)));
        return mesh;

    } 
}
