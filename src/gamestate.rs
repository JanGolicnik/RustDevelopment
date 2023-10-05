use bevy::prelude::*;

use crate::{bird::Bird, physics::Velocity, ResolutionSettings};

pub enum GameState {
    Menu,
    Playing,
    Dead,
}

#[derive(Resource)]
pub struct GameData {
    pub state: GameState,
    pub score: i32,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, gamestate_system);
    }
}

fn gamestate_system(
    mut game_data: ResMut<GameData>,
    mouse_input: Res<Input<MouseButton>>,
    mut bird_query: Query<(&mut Transform, &mut Velocity), With<Bird>>,
    resolution_settings: Res<ResolutionSettings>,
) {
    let game_data = game_data.as_mut();

    match game_data.state {
        GameState::Menu => {
            if mouse_input.just_pressed(MouseButton::Left) {
                game_data.state = GameState::Playing;
            }
        }
        GameState::Playing => {}
        GameState::Dead => {
            if mouse_input.just_pressed(MouseButton::Left) {
                game_data.state = GameState::Menu;
                bird_query.for_each_mut(|(mut transform, mut velocity)| {
                    transform.translation = Vec3::new(-resolution_settings.x * 0.35, 0.0, 0.0);
                    velocity.0 = Vec2::new(0.0, 0.0);
                });
                game_data.score = 0;
            }
        }
    }
}
