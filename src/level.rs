use bevy::prelude::*;
use bevy_ecs_ldtk::{
    prelude::LdtkIntCell, EntityInstance, IntGridCell, LdtkEntity, LdtkWorldBundle, LevelSelection,
};
use bevy_rapier2d::prelude::{ActiveEvents, Collider, LockedAxes, Restitution, RigidBody, Sensor};

use crate::{common::TILE_SIZE, player::Player};

pub const LEVELS: [&str; 9] = [
    "f8493dd0-6280-11ee-bc45-7be909942002",
    "8584c690-6280-11ee-a62a-712a834bb96b",
    "5bd355d0-6280-11ee-a62a-f3a96b194a86",
    "c619fdc0-6280-11ee-a62a-c38e14c026e3",
    "5d7ce720-6280-11ee-a62a-3d027ed4abce",
    "5f2c44d0-6280-11ee-a62a-398cc8d89a9b",
    "c77bfec0-6280-11ee-a62a-b164e5213fab",
    "9b8de140-6280-11ee-a62a-8bec7c7cc6d6",
    "688240c0-6280-11ee-a62a-178c252f6d69",
];

#[derive(Clone, Debug, Default, Bundle)]
pub struct SensorBundle {
    pub collider: Collider,
    pub sensor: Sensor,
    pub active_events: ActiveEvents,
    pub rotation_constraints: LockedAxes,
}

#[derive(Component, Default)]
pub struct Terrain;

#[derive(Bundle, LdtkIntCell)]
pub struct TerrainBundle {
    pub terrain: Terrain,
    #[from_int_grid_cell]
    pub collider_bundle: ColliderBundle,
}

#[derive(Clone, Debug, Default, Bundle, LdtkIntCell)]
pub struct ColliderBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub restitution: Restitution,
    pub active_events: ActiveEvents,
}

impl From<IntGridCell> for ColliderBundle {
    fn from(int_grid_cell: IntGridCell) -> ColliderBundle {
        if int_grid_cell.value == 1 {
            ColliderBundle {
                collider: Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
                rigid_body: RigidBody::Fixed,
                active_events: ActiveEvents::COLLISION_EVENTS,
                restitution: Restitution::new(0.0),
            }
        } else {
            panic!("Unsupported int grid cell value")
        }
    }
}

#[derive(Debug, Component, Clone, Copy, Default)]
pub struct Spikes;

#[derive(Clone, Default, Bundle, LdtkEntity)]
pub struct SpikesBundle {
    pub spikes: Spikes,
    #[sprite_sheet_bundle("textures/atlas.png", 8.0, 8.0, 16, 11, 0.0, 0.0, 17)]
    sprite_bundle: SpriteSheetBundle,
    #[from_entity_instance]
    sensor_bundle: SensorBundle,
}

impl From<&EntityInstance> for SensorBundle {
    fn from(entity_instance: &EntityInstance) -> SensorBundle {
        match entity_instance.identifier.as_ref() {
            "Spikes" => SensorBundle {
                collider: Collider::cuboid(TILE_SIZE / 2.2, TILE_SIZE / 2.2),
                sensor: Sensor,
                rotation_constraints: LockedAxes::ROTATION_LOCKED,
                active_events: ActiveEvents::COLLISION_EVENTS,
            },
            _ => SensorBundle::default(),
        }
    }
}

pub fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("level.ldtk"),
        ..Default::default()
    });
}

pub fn level_load_system(
    mut level_selection: ResMut<LevelSelection>,
    player_transform: Query<&Transform, With<Player>>,
) {
    if player_transform.is_empty() {
        return;
    }

    let transform = player_transform.single();

    let index = position_to_level_index(transform.translation.truncate());
    *level_selection = LevelSelection::Iid(LEVELS[index].to_string());
}

pub fn position_to_level_index(pos: Vec2) -> usize {
    let x_index = (pos.x / 256.0) as usize;
    let y_index = ((pos.y / 256.0) + 1.0) as usize;

    let mut index = x_index + y_index * 3;

    if index > 8 {
        index = 8;
    }

    index
}
