use background::BackgroundPlugin;
use bevy::prelude::*;
mod background;
mod bird;
mod gamestate;
mod offscreen_deletion;
mod physics;
mod pipes;
mod ysort;

use bird::{Bird, BirdPlugin};
use gamestate::{GameData, GameState, GameStatePlugin};
use offscreen_deletion::OffscreenDeletionPlugin;
use physics::PhysicsPlugin;
use pipes::PipePlugin;

#[derive(Resource, Deref)]
pub struct ResolutionSettings(Vec2);
fn main() {
    App::new()
        .insert_resource(ResolutionSettings(Vec2::new(450.0, 800.0)))
        .insert_resource(GameData {
            state: GameState::Menu,
            score: 0,
        })
        .insert_resource(ClearColor(Color::BEIGE))
        .add_plugins(DefaultPlugins)
        .add_plugins(PipePlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(BirdPlugin)
        .add_plugins(GameStatePlugin)
        .add_plugins(OffscreenDeletionPlugin)
        .add_plugins(BackgroundPlugin)
        .add_systems(Startup, startup)
        .run();
}

fn startup(
    mut commands: Commands,
    resolution_settings: Res<ResolutionSettings>,
    mut windows: Query<&mut Window>,
) {
    commands.spawn(Camera2dBundle::default());

    let mut window = windows.single_mut();

    window
        .resolution
        .set(resolution_settings.x, resolution_settings.y);

    Bird::spawn(
        &mut commands,
        Vec3::new(-resolution_settings.x * 0.35, 0.0, 0.0),
    );
}
