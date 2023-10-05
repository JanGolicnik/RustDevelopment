use crate::{
    gamestate::{GameData, GameState},
    offscreen_deletion::OffscreenDeletion,
    physics::Velocity,
    ysort::YSort,
    ResolutionSettings,
};
use bevy::prelude::*;
use rand::prelude::*;
use std::time::Duration;

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct Collider;

#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

#[derive(Resource)]
pub struct PipeSpawnSettings {
    pub min_time: f32,
    pub max_time: f32,
    pub speed: f32,
}

pub struct PipePlugin;
impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_pipes_system)
            //.add_systems(FixedUpdate, count_pipes)
            .insert_resource(PipeSpawnSettings {
                min_time: 1.0,
                max_time: 2.0,
                speed: -20.0,
            })
            .insert_resource(SpawnTimer {
                timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            });
    }
}

fn _count_pipes(pipes: Query<&Pipe>) {
    println!("{}", pipes.iter().count());
}

fn spawn_pipes_system(
    mut commands: Commands,
    time: Res<Time>,
    mut spawn_timer: ResMut<SpawnTimer>,
    resolution_settings: Res<ResolutionSettings>,
    pipe_spawn_settings: Res<PipeSpawnSettings>,
    mut game_data: ResMut<GameData>,
) {
    match game_data.state {
        GameState::Dead | GameState::Menu => return,
        _ => {}
    }

    spawn_timer
        .timer
        .tick(Duration::from_secs_f32(time.delta_seconds()));

    if !spawn_timer.timer.finished() {
        return;
    }

    game_data.score += 1;

    let pipe_height = resolution_settings.y;
    let space_between = resolution_settings.y * 0.3;
    let center_pos = resolution_settings.y * rand::thread_rng().gen_range(-0.1..0.1);

    let offset = (pipe_height + space_between) * 0.5;
    let additional_top_offset = rand::thread_rng().gen_range(0.0..40.0);

    spawn_pipe(
        &mut commands,
        pipe_height,
        Vec3::new(
            resolution_settings.x,
            center_pos - additional_top_offset + offset,
            0.0,
        ),
        &pipe_spawn_settings,
    );
    spawn_pipe(
        &mut commands,
        pipe_height,
        Vec3::new(resolution_settings.x, center_pos - offset, 0.0),
        &pipe_spawn_settings,
    );
}

fn spawn_pipe(
    commands: &mut Commands,
    pipe_height: f32,
    translation: Vec3,
    spawn_settings: &Res<PipeSpawnSettings>,
) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::RED,
                ..default()
            },
            transform: Transform {
                translation,
                scale: Vec3::new(40.0, pipe_height, 0.0),
                ..default()
            },
            ..default()
        },
        Pipe,
        Velocity(Vec2::new(spawn_settings.speed, 0.0)),
        Collider,
        OffscreenDeletion,
        YSort(1.0),
    ));
}
