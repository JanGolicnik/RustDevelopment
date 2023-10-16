use bevy::prelude::*;

use crate::animation::{AnimationIndices, AnimationTimer};

#[derive(Component)]
pub struct Dust;

pub fn spawn_dust(
    commands: &mut Commands,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    dust_pos: Vec2,
    dust_color: Color,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 11, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn((
        Dust,
        SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 29,
                color: dust_color,
                ..default()
            },
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_translation(dust_pos.extend(5.0)),
            ..default()
        },
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        AnimationIndices {
            index: 0,
            sprite_indices: vec![29, 30, 31],
        },
    ));
}

pub fn animate_dust(
    mut commands: Commands,
    mut q_dust: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Dust>,
    >,
    time: Res<Time>,
) {
    for (entity, mut timer, mut indices, mut sprite) in &mut q_dust {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            // 切换到下一个sprite
            if indices.index == indices.sprite_indices.len() - 1 {
                commands.entity(entity).despawn();
            } else {
                indices.index += 1;
                sprite.index = indices.sprite_indices[indices.index];
            };
        }
    }
}
