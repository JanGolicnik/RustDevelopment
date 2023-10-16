use bevy::prelude::*;
use bevy_ecs_ldtk::EntityInstance;
use bevy_rapier2d::prelude::{
    ActiveEvents, Collider, CollisionEvent, GravityScale, LockedAxes, QueryFilter, RapierContext,
    Restitution, RigidBody, Velocity,
};

use crate::{
    animation::{AnimationIndices, AnimationTimer},
    common::{PLAYER_GRAVITY_SCALE, PLAYER_JUMP_SPEED, PLAYER_RUN_SPEED, TILE_SIZE},
    dust::spawn_dust,
    level::{ColliderBundle, Spikes, Terrain},
};

#[derive(Event)]
pub struct PlayerDiedEvent;

#[derive(Resource)]
pub struct LastPlayerPosition(pub Vec2);

#[derive(Debug, Resource, Clone, Copy, Default, PartialEq, Eq, Reflect)]
pub enum PlayerState {
    #[default]
    Standing,
    Running,
    Jumping,
    Climbing,
}

#[derive(Component, Default, PartialEq, Eq)]
pub enum Facing {
    Left,
    #[default]
    Right,
}

#[derive(Component, Default)]
pub struct Player;

#[derive(Debug, Default, Resource)]
pub struct PlayerGrounded(pub bool);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NextToSomething {
    Left,
    Right,
}

#[derive(Debug, Default, Resource)]
pub struct PlayerNextTo(pub Option<NextToSomething>);

#[derive(Default, Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub sprite_bundle: SpriteSheetBundle,
    pub collider_bundle: ColliderBundle,
    pub velocity: Velocity,
    pub gravity_scale: GravityScale,
    pub rotation_constraints: LockedAxes,
    pub animation_timer: AnimationTimer,
    pub animation_indices: AnimationIndices,
    pub facing: Facing,
}

pub fn setup_player(
    commands: &mut Commands,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    asset_server: &Res<AssetServer>,
    position: Vec2,
) {
    let texture_handle = asset_server.load("textures/atlas.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(8.0, 8.0), 16, 11, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn(PlayerBundle {
        player: Player,
        sprite_bundle: SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            sprite: TextureAtlasSprite::new(1),
            transform: Transform::from_translation(position.extend(4.0)),
            ..default()
        },
        collider_bundle: ColliderBundle {
            collider: Collider::ball(TILE_SIZE / 2.0),
            rigid_body: RigidBody::Dynamic,
            restitution: Restitution::new(0.0),
            active_events: ActiveEvents::COLLISION_EVENTS,
        },
        rotation_constraints: LockedAxes::ROTATION_LOCKED,
        velocity: Velocity::zero(),
        gravity_scale: GravityScale(PLAYER_GRAVITY_SCALE),
        animation_timer: AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        animation_indices: AnimationIndices {
            index: 0,
            sprite_indices: vec![1, 2, 3, 4],
        },
        facing: Facing::Right,
    });
}

pub fn player_run(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    player_next_to: Res<PlayerNextTo>,
) {
    if player_query.is_empty() {
        return;
    }
    let mut velocity = player_query.single_mut();

    if keyboard_input.pressed(KeyCode::A) {
        if player_next_to.0.is_none() || player_next_to.0.unwrap() != NextToSomething::Left {
            velocity.linvel.x = -PLAYER_RUN_SPEED;
        }
    } else if keyboard_input.pressed(KeyCode::D) {
        if player_next_to.0.is_none() || player_next_to.0.unwrap() != NextToSomething::Right {
            velocity.linvel.x = PLAYER_RUN_SPEED;
        }
    } else {
        velocity.linvel.x = 0.0;
    }
}

pub fn player_jump(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Velocity, With<Player>>,
    player_state: ResMut<PlayerState>,
) {
    if player_query.is_empty() {
        return;
    }

    if *player_state == PlayerState::Standing
        || *player_state == PlayerState::Running
        || *player_state == PlayerState::Climbing
    {
        let mut velocity = player_query.single_mut();
        if keyboard_input.just_pressed(KeyCode::Space) {
            velocity.linvel = Vec2::new(0.0, PLAYER_JUMP_SPEED);
        }
    }
}

pub fn player_facing_update(mut player_query: Query<(&Velocity, &mut Facing), With<Player>>) {
    if player_query.is_empty() {
        return;
    }
    let (velocity, mut facing) = player_query.single_mut();
    if velocity.linvel.x > 0. {
        *facing = Facing::Right;
    } else if velocity.linvel.x < 0. {
        *facing = Facing::Left;
    }
}

pub fn animate_run(
    mut player_query: Query<
        (
            &Facing,
            &mut AnimationTimer,
            &mut AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    player_state: ResMut<PlayerState>,
) {
    if player_query.is_empty() {
        return;
    }

    let (facing, mut animation_timer, mut animation_indices, mut texture_atlas_sprite) =
        player_query.single_mut();

    if *player_state == PlayerState::Running {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.just_finished() {
            texture_atlas_sprite.index =
                if animation_indices.index == animation_indices.sprite_indices.len() - 1 {
                    animation_indices.index = 0;
                    animation_indices.sprite_indices[animation_indices.index]
                } else {
                    animation_indices.index += 1;
                    animation_indices.sprite_indices[animation_indices.index]
                };
        }
    } else {
        animation_indices.index = 1;
        texture_atlas_sprite.index = animation_indices.sprite_indices[animation_indices.index];
    }

    texture_atlas_sprite.flip_x = *facing == Facing::Left;
}

pub fn player_spikes_collision(
    mut collision_er: EventReader<CollisionEvent>,
    trap_query: Query<Entity, With<Spikes>>,
    mut player_died_event: EventWriter<PlayerDiedEvent>,
) {
    for event in collision_er.iter() {
        if let CollisionEvent::Started(entity1, entity2, _) = event {
            if trap_query.contains(*entity1) || trap_query.contains(*entity2) {
                player_died_event.send(PlayerDiedEvent);
            }
        }
    }
}

pub fn player_climb(
    mut player_query: Query<(&mut Velocity, &mut GravityScale), With<Player>>,
    player_state: Res<PlayerState>,
    mut last_player_state: Local<PlayerState>,
) {
    if player_query.is_empty() {
        return;
    }
    let (mut velocity, mut gravity_scale) = player_query.single_mut();

    if *player_state == PlayerState::Climbing && *last_player_state != PlayerState::Climbing {
        velocity.linvel = Vec2::new(0.0, -20.0);
        gravity_scale.0 = 0.0;
    }

    if *player_state != PlayerState::Climbing && *last_player_state == PlayerState::Climbing {
        gravity_scale.0 = PLAYER_GRAVITY_SCALE;
    }

    *last_player_state = *player_state;
}

pub fn player_grounded_detect(
    player_query: Query<&Transform, With<Player>>,
    mut player_grounded: ResMut<PlayerGrounded>,
    // mut last: Local<(f32, isize)>,
    rapier_context: Res<RapierContext>,
    terrain_query: Query<&GlobalTransform, With<Terrain>>,
) {
    if player_query.is_empty() {
        return;
    }

    let pos = player_query.single().translation.truncate();
    player_grounded.0 = if let Some((entity, _)) = rapier_context.cast_ray(
        pos + Vec2::new(0.0, -TILE_SIZE / 2. - 0.1),
        Vec2::NEG_Y,
        1.0,
        true,
        QueryFilter::default(),
    ) {
        terrain_query.contains(entity)
    } else {
        false
    };
}

pub fn player_next_to_detect(
    rapier_context: Res<RapierContext>,
    player_query: Query<&Transform, With<Player>>,
    terrain_query: Query<&GlobalTransform, With<Terrain>>,
    mut player_next_to: ResMut<PlayerNextTo>,
) {
    if player_query.is_empty() {
        return;
    }
    let player_pos = player_query.single().translation.truncate();

    if let Some((entity, _)) = rapier_context.cast_ray(
        player_pos + Vec2::new(-TILE_SIZE / 2. - 0.1, 0.),
        Vec2::NEG_X,
        1.0,
        true,
        QueryFilter::default(),
    ) {
        if terrain_query.contains(entity) {
            player_next_to.0 = Some(NextToSomething::Left);
        }
    } else if let Some((entity, _)) = rapier_context.cast_ray(
        player_pos + Vec2::new(TILE_SIZE / 2. + 0.1, 0.),
        Vec2::X,
        1.0,
        true,
        QueryFilter::default(),
    ) {
        if terrain_query.contains(entity) {
            player_next_to.0 = Some(NextToSomething::Right);
        }
    } else {
        player_next_to.0 = None;
    }
}

pub fn player_state_machine(
    player_query: Query<&Velocity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_state: ResMut<PlayerState>,
    player_grounded: Res<PlayerGrounded>,
    player_next_to: Res<PlayerNextTo>,
) {
    if player_query.is_empty() {
        return;
    }

    let velocity = player_query.single();

    if player_grounded.0 {
        if velocity.linvel.x.abs() < 0.1 {
            *player_state = PlayerState::Standing;
            return;
        } else {
            *player_state = PlayerState::Running;
            return;
        }
    }

    if let Some(next_to) = player_next_to.0 {
        match next_to {
            NextToSomething::Right => {
                if keyboard_input.pressed(KeyCode::D) {
                    *player_state = PlayerState::Climbing;
                    return;
                }
            }
            NextToSomething::Left => {
                if keyboard_input.pressed(KeyCode::A) {
                    *player_state = PlayerState::Climbing;
                    return;
                }
            }
        }
    }

    if !player_grounded.0 {
        *player_state = PlayerState::Jumping;
    }
}

pub fn player_revive(
    mut commands: Commands,
    player_query: Query<&Player>,
    entity_query: Query<(&Transform, &EntityInstance)>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    if player_query.is_empty() {
        for (transform, entity_instance) in &entity_query {
            if entity_instance.identifier == *"Player" {
                let camera_transform = camera_query.single();

                let mut player_transform = camera_transform.translation + transform.translation;
                player_transform -= 256.0 * 0.5;

                setup_player(
                    &mut commands,
                    &mut texture_atlases,
                    &asset_server,
                    player_transform.truncate(),
                );

                spawn_dust(
                    &mut commands,
                    &mut texture_atlases,
                    &asset_server,
                    player_transform.truncate(),
                    Color::CRIMSON,
                );
                break;
            }
        }
    }
}

pub fn player_die(
    player_query: Query<(&Transform, Entity), With<Player>>,
    player_died_event: EventReader<PlayerDiedEvent>,
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
    if player_query.is_empty() || player_died_event.is_empty() {
        return;
    }

    let (transform, entity) = player_query.single();
    commands.entity(entity).despawn_recursive();

    spawn_dust(
        &mut commands,
        &mut texture_atlases,
        &asset_server,
        transform.translation.truncate(),
        Color::WHITE,
    );
}
