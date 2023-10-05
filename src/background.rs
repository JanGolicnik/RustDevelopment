use bevy::prelude::*;

use crate::{
    gamestate::{GameData, GameState},
    ysort::YSort,
};

pub struct BackgroundPlugin;

#[derive(Component)]
struct GameText;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, game_text_system)
            .add_systems(Update, background_system);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "123",
            TextStyle {
                font_size: 100.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            margin: UiRect {
                left: Val::Auto,
                right: Val::Auto,
                top: Val::Px(30.0),
                ..default()
            },
            ..default()
        }),
        GameText,
        YSort(0.0),
    ));
}

fn game_text_system(
    mut query: Query<&mut Text, With<GameText>>,
    game_data: Res<GameData>,
    clear_color: ResMut<ClearColor>,
) {
    query.for_each_mut(|mut text| {
        text.sections[0].value = game_data.score.to_string();
        text.sections[0].style.color = clear_color.as_rgba() * 0.5;
    })
}

fn background_system(
    mut clear_color: ResMut<ClearColor>,
    game_data: Res<GameData>,
    time: Res<FixedTime>,
) {
    let target_color = match game_data.state {
        GameState::Menu => Color::BEIGE,
        GameState::Playing => Color::AZURE,
        GameState::Dead => Color::CRIMSON,
    };

    *clear_color = ClearColor(Color::from(
        Vec4::from(clear_color.as_rgba()).lerp(Vec4::from(target_color), time.period.as_secs_f32()),
    ));
}
