use bevy::{prelude::*, render::texture::ImageSampler, window::WindowResolution};
use bevy_ecs_ldtk::{
    prelude::{LdtkEntityAppExt, LdtkIntCellAppExt},
    LdtkPlugin, LdtkSettings, LevelSelection,
};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use camera::{setup_camera, update_camera};
use dust::animate_dust;
use game_state::GameState;
use level::{level_load_system, setup_level, SpikesBundle, TerrainBundle};
use main_menu::{cleanup_main_menu, main_menu_input, setup_main_menu};
use player::{
    animate_run, player_climb, player_die, player_facing_update, player_grounded_detect,
    player_jump, player_next_to_detect, player_revive, player_run, player_spikes_collision,
    player_state_machine, LastPlayerPosition, PlayerDiedEvent, PlayerGrounded, PlayerNextTo,
    PlayerState,
};

mod animation;
mod camera;
mod common;
mod dust;
mod game_state;
mod level;
mod main_menu;
mod player;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin {
                    default_sampler: ImageSampler::nearest_descriptor(),
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        #[cfg(target_os = "windows")]
                        resolution: WindowResolution::new(1000.0, 1000.0),
                        ..default()
                    }),
                    ..default()
                }),
        )
        .add_plugins(LdtkPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .insert_resource(LdtkSettings {
            level_spawn_behavior: bevy_ecs_ldtk::LevelSpawnBehavior::UseWorldTranslation {
                load_level_neighbors: false,
            },
            ..default()
        })
        .insert_resource(LevelSelection::Index(0))
        .insert_resource(PlayerState::Standing)
        .insert_resource(PlayerNextTo(None))
        .insert_resource(PlayerGrounded(false))
        .insert_resource(LastPlayerPosition(Vec2::ZERO))
        .add_state::<GameState>()
        .add_event::<PlayerDiedEvent>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::MainMenu), setup_main_menu)
        .add_systems(
            Update,
            (main_menu_input).run_if(in_state(GameState::MainMenu)),
        )
        .add_systems(OnExit(GameState::MainMenu), cleanup_main_menu)
        .add_systems(OnEnter(GameState::Playing), setup_level)
        .add_systems(
            Update,
            (
                (
                    update_camera,
                    player_revive.after(update_camera),
                    level_load_system.after(player_revive),
                    player_state_machine.after(player_revive),
                ),
                (
                    player_run,
                    player_jump,
                    player_facing_update,
                    animate_run,
                    player_spikes_collision,
                    player_grounded_detect,
                    player_next_to_detect,
                    player_climb,
                    animate_dust,
                ),
                (player_die),
            )
                .run_if(in_state(GameState::Playing)),
        )
        .register_ldtk_int_cell::<TerrainBundle>(1)
        .register_ldtk_entity::<SpikesBundle>("Spikes")
        .run();
}
