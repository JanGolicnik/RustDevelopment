use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    sprite::{
        collide_aabb::{collide, Collision},
        MaterialMesh2dBundle,
    },
    window::PrimaryWindow,
};

const PADDLE_SIZE: Vec3 = Vec3::new(120.0, 20.0, 0.0);
const PADDLE_POSITION_Y: f32 = -300.0;
const PADDLE_SPEED: f32 = 1300.0;
const PADDLE_COLOR: Color = Color::rgb(38.0 / 255.0, 87.0 / 255.0, 124.0 / 255.0);

const BALL_COLOR: Color = Color::rgb(229.0 / 255.0, 86.0 / 255.0, 4.0 / 255.0);
const BALL_SIZE: Vec3 = Vec3::new(30.0, 30.0, 30.0);
const BALL_SPEED: f32 = 600.0;
const BALL_STARTING_POSITION: Vec3 = Vec3::new(0.0, 0.0, 0.0);
const BALL_STARTING_DIRECTION: Vec2 = Vec2::new(-1.0, -1.0);

const BRICK_SIZE: Vec3 = Vec3::new(120.0, 45.0, 0.0);
const BRICK_SPACING: f32 = 20.0;
const BRICK_COLOR: Color = Color::rgb(235.0 / 255.0, 228.0 / 255.0, 209.0 / 255.0);
const N_BRICK_ROWS: u8 = 3;
const N_BRICK_COLUMNS: u8 = 6;

const BOUNDS: (Vec2, Vec2) = (Vec2::new(-650.0, -350.0), Vec2::new(650.0, 350.0));

const BACKGROUND_COLOR: Color = Color::rgb(180.0 / 255.0, 180.0 / 255.0, 179.0 / 255.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                check_for_collisions,
                apply_velocity.before(check_for_collisions),
                move_paddle
                    .before(check_for_collisions)
                    .after(apply_velocity),
            ),
        )
        .run();
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Paddle;

#[derive(Component)]
struct Ball;

#[derive(Component)]
struct Brick;

#[derive(Component)]
struct Collider;
#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Event, Default)]
struct CollisionEvent;

fn check_for_collisions(
    mut commands: Commands,
    mut ball_query: Query<(&mut Velocity, &Transform), (With<Ball>, Without<Collider>)>,
    collider_query: Query<(Entity, &Transform, Option<&Brick>), With<Collider>>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    let (mut ball_velocity, ball_transform) = ball_query.single_mut();
    let ball_size = ball_transform.scale.truncate();

    if ball_transform.translation.x < BOUNDS.0.x && ball_velocity.x < 0.0 {
        ball_velocity.x = -ball_velocity.x;
    }

    if ball_transform.translation.x > BOUNDS.1.x && ball_velocity.x > 0.0 {
        ball_velocity.x = -ball_velocity.x;
    }

    if ball_transform.translation.y < BOUNDS.0.y && ball_velocity.y < 0.0 {
        ball_velocity.y = -ball_velocity.y;
    }

    if ball_transform.translation.y > BOUNDS.1.y && ball_velocity.y > 0.0 {
        ball_velocity.y = -ball_velocity.y;
    }

    for (_collider_entity, transform, maybe_brick) in &collider_query {
        let collision = collide(
            ball_transform.translation,
            ball_size,
            transform.translation,
            transform.scale.truncate(),
        );

        if let Some(collision) = collision {
            collision_events.send_default();

            let mut reflect_x = false;
            let mut reflect_y = false;

            if maybe_brick.is_some() {
                commands.entity(_collider_entity).despawn();
            }

            match collision {
                Collision::Left => reflect_x = ball_velocity.x > 0.0,
                Collision::Right => reflect_x = ball_velocity.x < 0.0,
                Collision::Top => reflect_y = ball_velocity.y < 0.0,
                Collision::Bottom => reflect_y = ball_velocity.y > 0.0,
                Collision::Inside => {}
            }

            if reflect_x {
                ball_velocity.x = -ball_velocity.x;
            }

            if reflect_y {
                ball_velocity.y = -ball_velocity.y;
            }
        }
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &mut Velocity)>, time_step: Res<FixedTime>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time_step.period.as_secs_f32();
        transform.translation.y += velocity.y * time_step.period.as_secs_f32();
    }
}

fn move_paddle(
    mouse_input: Res<Input<MouseButton>>,
    mut query: Query<(&mut Transform, &mut Velocity), With<Paddle>>,
    time_step: Res<FixedTime>,
    window: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (mut paddle_transform, mut paddle_velocity) = query.single_mut();
    let direction = 0.0;

    let (camera, camera_transform) = camera_q.single();

    if paddle_transform.translation.x > BOUNDS.1.x {
        paddle_transform.translation.x = BOUNDS.0.x;
    }

    if paddle_transform.translation.x < BOUNDS.0.x {
        paddle_transform.translation.x = BOUNDS.1.x;
    }

    if let Some(position) = window.single().cursor_position() {
        if let Some(position) = camera
            .viewport_to_world(camera_transform, position)
            .map(|ray| ray.origin.truncate())
        {
            let mut multiplier = 1.0;
            if mouse_input.pressed(MouseButton::Left) {
                multiplier = 3.0;
            }
            let target_vel =
                (position.x - paddle_transform.translation.x).signum() * PADDLE_SPEED * multiplier;
            let vel_diff = target_vel - paddle_velocity.x;
            paddle_velocity.x += vel_diff / (time_step.period.as_secs_f32() * 1000.0);
        }
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        Camera2dBundle {
            camera_2d: Camera2d {
                clear_color: ClearColorConfig::Custom(BACKGROUND_COLOR),
                ..Default::default()
            },
            ..Default::default()
        },
        MainCamera,
    ));
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, PADDLE_POSITION_Y, 0.0),
                scale: PADDLE_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: PADDLE_COLOR,
                ..default()
            },
            ..default()
        },
        Paddle,
        Collider,
        Velocity(Vec2::new(0.0, 0.0)),
    ));

    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::default().into()).into(),
            material: materials.add(ColorMaterial::from(BALL_COLOR)),
            transform: Transform::from_translation(BALL_STARTING_POSITION).with_scale(BALL_SIZE),
            ..default()
        },
        Ball,
        Velocity(BALL_STARTING_DIRECTION.normalize() * BALL_SPEED),
    ));

    for x in 0..N_BRICK_COLUMNS {
        for y in 0..N_BRICK_ROWS {
            let x: f32 = x as f32;
            let y: f32 = y as f32;
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            -BRICK_SIZE.x * N_BRICK_COLUMNS as f32 * 0.5
                                + x * (BRICK_SPACING + BRICK_SIZE.x),
                            BOUNDS.1.y - BRICK_SPACING * (y + 1.0) - BRICK_SIZE.y * y,
                            0.0,
                        ),
                        scale: BRICK_SIZE,
                        ..default()
                    },
                    sprite: Sprite {
                        color: BRICK_COLOR,
                        ..default()
                    },
                    ..default()
                },
                Brick,
                Collider,
            ));
        }
    }
}
