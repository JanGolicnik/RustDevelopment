use bevy::{prelude::*, render::{mesh::Indices, render_resource::PrimitiveTopology}};
use noise::{Perlin, NoiseFn};

const CHUNK_X: usize = 16;
const CHUNK_Z: usize = 16;
const CHUNK_Y: usize = 16;

struct Chunk([bool; CHUNK_X * CHUNK_Y * CHUNK_Z]);

impl Chunk{
    pub fn new(val: bool) -> Self{
        Chunk([val; CHUNK_X * CHUNK_Y * CHUNK_Z])
    }
    pub fn set(&mut self, mut x: usize, mut y: usize, mut z: usize, val: bool) {
        if x >= CHUNK_X { x = CHUNK_X - 1; }
        if y >= CHUNK_Y { y = CHUNK_Y - 1; }
        if z >= CHUNK_Z { z = CHUNK_Z - 1; }
        self.0[x + z * CHUNK_X + y * CHUNK_X * CHUNK_Y] = val;
    }
    pub fn get(&self, x: usize, y: usize, z: usize) -> bool {
        if x >= CHUNK_X { return false; }
        if y >= CHUNK_Y { return false; }
        if z >= CHUNK_Z { return false; }
        self.0[x + z * CHUNK_X + y * CHUNK_X * CHUNK_Y]
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins).add_systems(Startup, setup).add_systems(Update, update_camera).run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>){

    for x in -2..3 {
        for z in -2..3 {

            commands.spawn(PbrBundle {
                mesh: meshes.add(chunk_mesh(x, z)),
                material: materials.add(Color::rgb(0.35, 0.7, 0.6).into()),
                ..default()
            });

        }
    }

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-20., 20., 20.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true, 
            illuminance: 80_000.,
            ..default() 
        },
        transform: Transform::from_xyz(2.0, 1.0, 2.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn chunk_mesh(mut position_x: i32, mut position_z: i32) -> Mesh {
    position_x *= CHUNK_X as i32;
    position_z *= CHUNK_Z as i32;

    let noise = Perlin::new(5);

    let mut height_map: [f64; CHUNK_X * CHUNK_Z] = [0.; CHUNK_X * CHUNK_Z]; 

    for x in 0..CHUNK_X {
        for z in 0..CHUNK_Z {
            let fx = position_x as f64 + x as f64; 
            let fz = position_z as f64 + z as f64;

            let noise_val = noise.get([fx / CHUNK_X as f64 , fz / CHUNK_Z as f64]);
            height_map[x + z * CHUNK_X] = (noise_val + 1.) / 2.;
        }
    }

    let mut chunk = Chunk::new(false);

    for x in 0..CHUNK_X {
        for z in 0..CHUNK_Z {

            let i = x + z * CHUNK_X;
            let height = (height_map[i] * CHUNK_Y as f64) as usize;
            for y in 0..height {
                chunk.set(x, y, z, true);
            }
        }
    }
    
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

    for y in 0..CHUNK_Y {
        for x in 0..CHUNK_X {
            for z in 0..CHUNK_Z {
                if chunk.get(x, y, z) {
                    let fy = y as f32;
                    let fx = position_x as f32 + x as f32; 
                    let fz = position_z as f32 + z as f32;

                    if x == 15 || !chunk.get(x + 1, y, z) {
                        add_plane(  fx + 0.5, fy - 0.5, fz - 0.5,
                            fx + 0.5, fy + 0.5, fz + 0.5, false);
                    }
                    if x == 0 || !chunk.get(x - 1, y, z) {
                        add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                            fx - 0.5, fy + 0.5, fz + 0.5, true);
                    }

                    if y == 15 || !chunk.get(x, y + 1, z) {
                        add_plane(  fx - 0.5, fy + 0.5, fz - 0.5,
                            fx + 0.5, fy + 0.5, fz + 0.5, false);
                    }
                    if y == 0 || !chunk.get(x, y - 1, z) {
                        add_plane(  fx - 0.5, fy - 0.5, fz - 0.5,
                            fx + 0.5, fy - 0.5, fz + 0.5, true);
                    }

                    if z == 15 || !chunk.get(x, y, z + 1) {
                        add_plane(  fx - 0.5, fy - 0.5, fz + 0.5,
                            fx + 0.5, fy + 0.5, fz + 0.5, false);
                    }
                    if z == 0 || !chunk.get(x, y, z - 1) {
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

    // let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

    // mesh.insert_attribute(
    //     Mesh::ATTRIBUTE_POSITION,
    //     vec![[0., 0., 0.], [0., 0., -2.], [1., 0., 0.]],
    // );
    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0., 1., 0.]; 3]);
    // mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; 3]);
    
    // mesh.set_indices(Some(Indices::U32(vec![0, 2, 1])));

    // mesh
} 


fn update_camera(mut camera_query: Query<&mut Transform, (With<Camera>, Without<DirectionalLight>)>, mut light_query: Query<&mut Transform, With<DirectionalLight>>, time: Res<Time>){
    let elapsed_time = time.elapsed().as_secs_f32();
    for mut transform in camera_query.iter_mut() {
        *transform = Transform::from_xyz(f32::sin(elapsed_time * 0.3) * 50., 40., f32::cos(elapsed_time * 0.3) * 40.).looking_at(Vec3::ZERO, Vec3::Y);
    }

    for mut transform in light_query.iter_mut() {
        *transform = Transform::from_xyz(f32::sin(elapsed_time), f32::cos(elapsed_time), 0.).looking_at(Vec3::ZERO, Vec3::Y);
    }
}