use super::{
    chunkmap::ChunkMap,
    chunkqueue::ChunkQueue,
    material::{WorldMaterial, WorldTexture},
    utils::NEIGHBOUR_OFFSETS,
    Chunk, WorldResourceLoadState, CHUNK_SIZE, RENDER_DIST, WORLD_SIZE,
};
use crate::{chunks::blocks::NUM_TEXTURES, Player};
use bevy::{asset::LoadState, prelude::*};

pub fn spawn_chunks(
    mut commands: Commands,
    mut chunk_q: ResMut<ChunkQueue>,
    world_texture: ResMut<WorldTexture>,
) {
    println!("spawn chunks");
    for chunk in &chunk_q.spawn_queue.clone() {
        if !chunk_q.spawned_chunks.contains_key(chunk) {
            let spawned_chunk_entity = commands
                .spawn((
                    MaterialMeshBundle {
                        material: world_texture.material_handle.as_ref().unwrap().clone(),
                        ..default()
                    },
                    (*chunk),
                ))
                .id();

            chunk_q.spawned_chunks.insert(*chunk, spawned_chunk_entity);
            if !chunk_q.gen_queue.contains(chunk) {
                chunk_q.gen_queue.push(*chunk);
            }

            for offset in NEIGHBOUR_OFFSETS {
                let x = chunk.0[0] + offset[0];
                let y = chunk.0[1] + offset[1];
                let z = chunk.0[2] + offset[2];
                let chunk = Chunk([x, y, z]);
                if chunk_q.spawned_chunks.contains_key(&chunk)
                    && !chunk_q.remesh_queue.contains(&chunk)
                {
                    chunk_q.remesh_queue.push(chunk);
                }
            }
        }
    }

    chunk_q.spawn_queue.clear();

    for (entity, chunk) in chunk_q.despawn_queue.clone() {
        commands.entity(entity).despawn();
        chunk_q.spawned_chunks.remove(&chunk);
    }
    chunk_q.despawn_queue.clear();
}

pub fn update_chunks(
    mut chunk_q: ResMut<ChunkQueue>,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(&Chunk, Entity), (With<Chunk>, Without<Player>)>,
) {
    println!("update_chunks");
    let player_transform = player_query.single();
    let transform = player_transform.translation.round();
    let current_chunk =
        ChunkMap::coords_to_chunk(&[transform.x as i32, transform.y as i32, transform.z as i32]);

    let lower = (WORLD_SIZE as f32 * -0.5).ceil() as i32;
    let upper = (WORLD_SIZE as f32 * 0.5).ceil() as i32;

    for x in lower..upper {
        for y in lower..upper {
            for z in lower..upper {
                let x = x + current_chunk.0[0];
                let y = y + current_chunk.0[1];
                let z = z + current_chunk.0[2];
                let chunk = Chunk([x, y, z]);
                if chunk_q.spawn_queue.contains(&chunk) {
                    continue;
                }
                chunk_q.spawn_queue.push(chunk);
            }
        }
    }

    for (chunk, entity) in chunk_query.iter() {
        if chunk_q.spawn_queue.contains(chunk) {
            continue;
        }

        let chunk_world_x = chunk.0[0] as f32 * CHUNK_SIZE as f32;
        let chunk_world_y = chunk.0[1] as f32 * CHUNK_SIZE as f32;
        let chunk_world_z = chunk.0[2] as f32 * CHUNK_SIZE as f32;

        let dist_x = chunk_world_x - player_transform.translation.x;
        let dist_y = chunk_world_y - player_transform.translation.y;
        let dist_z = chunk_world_z - player_transform.translation.z;

        if dist_x.abs() > RENDER_DIST || dist_y.abs() > RENDER_DIST || dist_z.abs() > RENDER_DIST {
            chunk_q.despawn_queue.push((entity, *chunk));
        }
    }
}

pub fn remesh_chunks(
    mut commands: Commands,
    chunkmap: Res<ChunkMap>,
    mut chunk_q: ResMut<ChunkQueue>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    println!("remesh_chunks {}", chunk_q.remesh_queue.len());
    for chunk in &chunk_q.remesh_queue {
        if let Some(entity) = chunk_q.spawned_chunks.get(chunk) {
            if let Some(mesh) = chunkmap.remesh(chunk) {
                commands.entity(*entity).try_insert(meshes.add(mesh));
            }
        }
    }
    println!("ended remesh_chunks");

    chunk_q.remesh_queue.clear();
}

pub fn gen_chunks(mut chunkmap: ResMut<ChunkMap>, mut chunk_q: ResMut<ChunkQueue>) {
    for chunk in &chunk_q.gen_queue {
        chunkmap.gen(chunk);
    }

    chunk_q.gen_queue.clear();
}

pub fn load_resources(
    asset_server: Res<AssetServer>,
    mut world_texture: ResMut<WorldTexture>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<WorldMaterial>>,
    mut load_state: ResMut<NextState<WorldResourceLoadState>>,
) {
    println!("load_resources");
    if asset_server.load_state(world_texture.handle.clone()) != LoadState::Loaded {
        return;
    }

    world_texture.is_loaded = true;
    let image = images.get_mut(&world_texture.handle).unwrap();
    image.reinterpret_stacked_2d_as_array(NUM_TEXTURES);

    world_texture.material_handle = Some(materials.add(WorldMaterial {
        array_texture: world_texture.handle.clone(),
    }));

    load_state.set(WorldResourceLoadState::Loaded);
}

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("setup");
    commands.insert_resource(WorldTexture {
        is_loaded: false,
        handle: asset_server.load("textures/world_tilemap.png"),
        material_handle: None,
    })
}
