use super::{
    chunkgrid::ChunkGrid,
    chunkmap::ChunkMap,
    chunkqueue::{ChunkCreateQueue, ChunkDespawnQueue, ChunkSpawnQueue},
    material::{WorldMaterial, WorldTexture},
    Chunk, WorldResourceLoadState, CHUNK_SIZE, RENDER_DIST, WORLD_SIZE,
};
use crate::{chunks::blocks::NUM_TEXTURES, craftdebug, Player};
use bevy::{
    asset::LoadState,
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task},
};
use futures_lite::future;
use noise::Perlin;

#[derive(Component)]
pub struct ComputeChunk(pub Task<Option<(Entity, Chunk, ChunkGrid, Mesh)>>);

pub fn spawn_chunks(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut spawn_queue: ResMut<ChunkSpawnQueue>,
    mut create_queue: ResMut<ChunkCreateQueue>,
    mut despawn_queue: ResMut<ChunkDespawnQueue>,
    world_texture: ResMut<WorldTexture>,
) {
    craftdebug!(println!("spawn chunks"));
    for chunk in &spawn_queue.0.clone() {
        if !chunk_map.entities.contains_key(chunk) {
            let spawned_chunk_entity = commands
                .spawn((
                    MaterialMeshBundle {
                        material: world_texture.material_handle.as_ref().unwrap().clone(),
                        ..default()
                    },
                    (*chunk),
                ))
                .id();

            chunk_map.entities.insert(*chunk, spawned_chunk_entity);
            let mut found = false;
            for (c, _) in create_queue.0.iter() {
                if c == chunk {
                    found = true;
                    break;
                }
            }
            if !found {
                create_queue.0.push((*chunk, spawned_chunk_entity));
            }
        }
    }

    spawn_queue.0.clear();

    for (entity, chunk) in &despawn_queue.0 {
        commands.entity(*entity).despawn();
        chunk_map.entities.remove(chunk);
        chunk_map.chunks.remove(chunk);
    }
    despawn_queue.0.clear();
}

pub fn update_chunks(
    mut spawn_queue: ResMut<ChunkSpawnQueue>,
    mut despawn_queue: ResMut<ChunkDespawnQueue>,
    player_query: Query<&Transform, With<Player>>,
    chunk_query: Query<(&Chunk, Entity), Without<Player>>,
) {
    craftdebug!(println!("update_chunks"));
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
                spawn_queue.0.push(chunk);
            }
        }
    }

    for (chunk, entity) in chunk_query.iter() {
        let chunk_world_x = chunk.0[0] as f32 * CHUNK_SIZE as f32;
        let chunk_world_y = chunk.0[1] as f32 * CHUNK_SIZE as f32;
        let chunk_world_z = chunk.0[2] as f32 * CHUNK_SIZE as f32;

        let dist_x = chunk_world_x - player_transform.translation.x;
        let dist_y = chunk_world_y - player_transform.translation.y;
        let dist_z = chunk_world_z - player_transform.translation.z;

        if dist_x.abs() > RENDER_DIST || dist_y.abs() > RENDER_DIST || dist_z.abs() > RENDER_DIST {
            despawn_queue.0.push((entity, *chunk));
        }
    }
}

pub fn create_chunks(
    mut commands: Commands,
    chunkmap: Res<ChunkMap>,
    mut create_queue: ResMut<ChunkCreateQueue>,
) {
    let thread_pool = AsyncComputeTaskPool::get();
    for (chunk, entity) in create_queue.0.clone() {
        let coords = chunk.0;

        if chunkmap.chunks.contains_key(&chunk) {
            continue;
        }
        let noise = Perlin::new(4);
        let task = thread_pool.spawn(async move {
            let grid = ChunkMap::gen(&chunk, &noise);
            let mesh = grid.to_mesh(&ChunkMap::chunk_to_world_coords(&Chunk(coords)), &noise);
            Some((entity, chunk, grid, mesh))
        });
        commands.spawn(ComputeChunk(task));
    }

    create_queue.0.clear();
}

pub fn load_resources(
    asset_server: Res<AssetServer>,
    mut world_texture: ResMut<WorldTexture>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<WorldMaterial>>,
    mut load_state: ResMut<NextState<WorldResourceLoadState>>,
) {
    craftdebug!(println!("load_resources"));
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
    craftdebug!(println!("setup"));
    commands.insert_resource(WorldTexture {
        is_loaded: false,
        handle: asset_server.load("textures/world_tilemap.png"),
        material_handle: None,
    })
}

pub fn create_from_compute(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut compute_chunk_query: Query<(Entity, &mut ComputeChunk)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for (compute_entity, mut compute_chunk) in compute_chunk_query.iter_mut() {
        if let Some(Some((chunk_entity, chunk, grid, mesh))) =
            future::block_on(future::poll_once(&mut compute_chunk.0))
        {
            commands.entity(compute_entity).despawn();
            chunk_map.chunks.insert(chunk, grid);

            commands.entity(chunk_entity).try_insert(meshes.add(mesh));
        }
    }
}

pub fn rebuild_chunks(
    mut commands: Commands,
    mut chunk_map: ResMut<ChunkMap>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for chunk in chunk_map.chunks_to_rebuild.clone() {
        let coords = chunk.0;

        if let Some(grid) = chunk_map.chunks.get(&chunk) {
            if let Some(entity) = chunk_map.entities.get(&chunk) {
                let noise = Perlin::new(4);
                let mesh = grid.to_mesh(&ChunkMap::chunk_to_world_coords(&Chunk(coords)), &noise);
                commands.entity(*entity).try_insert(meshes.add(mesh));
            }
        }
    }

    chunk_map.chunks_to_rebuild.clear();
}
