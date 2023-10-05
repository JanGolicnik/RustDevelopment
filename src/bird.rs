use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    gamestate::{GameData, GameState},
    physics::Velocity,
    pipes::{Collider, Pipe},
};

pub struct BirdPlugin;

impl Plugin for BirdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bird_input_system)
            .add_systems(Update, rotate_on_velocity_system)
            .add_systems(Update, bird_collision_system);
    }
}

fn bird_collision_system(
    mut commands: Commands,
    bird: Query<&Transform, With<Bird>>,
    query: Query<&Transform, With<Collider>>,
    mut game_data: ResMut<GameData>,
    pipes: Query<Entity, With<Pipe>>,
) {
    let bird_transform = bird.single();

    for transform in &query {
        let collision = collide(
            bird_transform.translation,
            bird_transform.scale.truncate() * 0.5,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(_collision) = collision {
            game_data.state = GameState::Dead;
            pipes.for_each(|entity| commands.entity(entity).despawn());
        }
    }
}

fn bird_input_system(
    mut query: Query<(&Transform, &mut Velocity), With<Bird>>,
    time: Res<FixedTime>,
    mouse_input: Res<Input<MouseButton>>,
    game_data: ResMut<GameData>,
) {
    let (transform, mut velocity) = query.single_mut();
    velocity.y -= 5.0 * time.period.as_secs_f32();
    match game_data.state {
        GameState::Dead => {
            if velocity.y > 0.0 {
                velocity.y = 0.0
            }
        }
        GameState::Menu => {
            if transform.translation.y < 0.0 {
                velocity.y = 35.0;
            }
        }
        GameState::Playing => {
            if mouse_input.pressed(MouseButton::Left) {
                velocity.y = 35.0;
            }
        }
    }
}

fn rotate_on_velocity_system(mut birds_query: Query<(&mut Transform, &Velocity), With<Bird>>) {
    birds_query.for_each_mut(|(mut transform, velocity)| {
        let mut percentage = velocity.y / 40.0;
        percentage = percentage.max(-1.0);
        percentage = percentage.min(1.0);

        let rad_angle = percentage * std::f32::consts::PI * 0.2;

        transform.rotation = Quat::from_rotation_z(rad_angle);
    });
}

#[derive(Component)]
pub struct Bird;

impl Bird {
    pub fn spawn(commands: &mut Commands, translation: Vec3) {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::BLUE,
                    ..default()
                },
                transform: Transform {
                    translation,
                    scale: Vec3::new(40.0, 40.0, 0.0),
                    ..default()
                },
                ..default()
            },
            Bird,
            Velocity(Vec2::new(0.0, 0.0)),
        ));
    }
}
