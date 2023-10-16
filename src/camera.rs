use bevy::{core_pipeline::clear_color::ClearColorConfig, prelude::*};

use crate::{common::CAMERA_SCALE, player::Player};

pub fn setup_camera(mut commands: Commands) {
    let mut bundle = Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::rgba(0.0, 0.02, 0.12, 1.0)),
        },
        transform: Transform {
            translation: Vec3::new(100.0, -100.0, 0.0),
            ..default()
        },
        ..default()
    };
    bundle.projection.scale = CAMERA_SCALE;
    commands.spawn(bundle);
}

pub fn update_camera(
    player_transform: Query<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera_transform: Query<&mut Transform, With<Camera2d>>,
) {
    if player_transform.is_empty() || camera_transform.is_empty() {
        return;
    }

    let player_transform = player_transform.single();
    let mut camera_transform = camera_transform.single_mut();

    let pos_x = ((player_transform.translation.x / 256.0).floor() + 0.5) * 256.0;
    let pos_y = ((player_transform.translation.y / 256.0).floor() + 0.5) * 256.0;

    camera_transform.translation.x = pos_x;
    camera_transform.translation.y = pos_y;
}
