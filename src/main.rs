use std::time::Duration;

use bevy::{core_pipeline::clear_color::ClearColorConfig, input::mouse::MouseWheel, prelude::*};
enum CellState {
    Alive,
    Dead,
}

#[derive(Resource)]
struct Board {
    width: usize,
    height: usize,
    cell_states: Vec<CellState>,
}

#[derive(Component)]
struct Cell {
    row: usize,
    col: usize,
}

#[derive(Resource, PartialEq)]
enum GameState {
    Paused,
    Simulating,
}

#[derive(Resource)]
struct SimulationTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "sim".into(),
                resolution: (1000., 1000.).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(Board {
            width: 250,
            height: 250,
            cell_states: Vec::new(),
        })
        .insert_resource(SimulationTimer(Timer::new(
            Duration::from_secs_f32(0.5),
            TimerMode::Repeating,
        )))
        .insert_resource(GameState::Paused)
        .add_systems(Startup, startup)
        .add_systems(Update, scroll_input_system)
        .add_systems(Update, mouse_input_system)
        .add_systems(Update, keyboard_input_system)
        .add_systems(Update, simulation_system)
        .add_systems(Update, refresh_cells_system.after(simulation_system))
        .run();
}

fn startup(mut commands: Commands, window: Query<&Window>, mut board: ResMut<Board>) {
    let window = window.single();

    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..default()
    });

    for row in 0..board.width {
        for col in 0..board.height {
            board.cell_states.push(CellState::Dead);

            let window_x = window.resolution.physical_width() as f32;
            let window_y = window.resolution.physical_height() as f32;

            let scale: Vec3 = Vec3::new(
                window_x / board.width as f32,
                window_y / board.height as f32,
                0.0,
            );

            let position_x = scale.x * row as f32 - window_x * 0.5 + scale.x * 0.5;
            let position_y = window_y * 0.5 - scale.y * col as f32 - scale.y * 0.5;

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: Color::WHITE,
                        ..default()
                    },
                    transform: Transform {
                        translation: Vec3::new(position_x, position_y, 0.0),
                        scale,
                        ..default()
                    },
                    ..default()
                },
                Cell { row, col },
            ));
        }
    }
}

fn simulation_system(
    mut board: ResMut<Board>,
    game_state: Res<GameState>,
    mut timer: ResMut<SimulationTimer>,
    time: Res<Time>,
) {
    if *game_state == GameState::Paused {
        return;
    }

    timer.0.tick(Duration::from_secs_f32(time.delta_seconds()));

    if !timer.0.finished() {
        return;
    }

    let mut cells_to_toggle: Vec<usize> = Vec::new();

    for row in 0..board.height {
        for col in 0..board.width {
            let mut living_neighbours: u8 = 0;

            for row_offset in -1..2 {
                for col_offset in -1..2 {
                    if row_offset == 0 && col_offset == 0 {
                        continue;
                    }

                    let row_neighbour = row as isize + row_offset;
                    let row_neighbour = if row_neighbour < 0 {
                        board.height - 1
                    } else if row_neighbour as usize >= board.height {
                        0
                    } else {
                        row_neighbour as usize
                    };

                    let col_neighbour = col as isize + col_offset;
                    let col_neighbour = if col_neighbour < 0 {
                        board.width - 1
                    } else if col_neighbour as usize >= board.width {
                        0
                    } else {
                        col_neighbour as usize
                    };

                    let neighbour_index = row_neighbour * board.height + col_neighbour;

                    match board.cell_states[neighbour_index] {
                        CellState::Alive => living_neighbours += 1,
                        CellState::Dead => {}
                    }
                }
            }

            let index = row * board.height + col;

            match board.cell_states[index] {
                CellState::Alive => {
                    if !(living_neighbours == 2 || living_neighbours == 3) {
                        cells_to_toggle.push(index);
                    }
                }
                CellState::Dead => {
                    if living_neighbours == 3 {
                        cells_to_toggle.push(index);
                    }
                }
            }
        }
    }

    cells_to_toggle
        .iter()
        .for_each(|index| match board.cell_states[*index] {
            CellState::Alive => {
                board.cell_states[*index] = CellState::Dead;
            }
            CellState::Dead => {
                board.cell_states[*index] = CellState::Alive;
            }
        })
}

fn refresh_cells_system(board: Res<Board>, mut cell_query: Query<(&mut Sprite, &Cell)>) {
    cell_query.for_each_mut(|(mut sprite, cell)| {
        let cell_index = cell.row * board.height + cell.col;
        match board.cell_states[cell_index] {
            CellState::Alive => sprite.as_mut().color = Color::WHITE,
            CellState::Dead => sprite.as_mut().color = Color::BLACK,
        }
    })
}

fn keyboard_input_system(mut game_state: ResMut<GameState>, keyboard_input: Res<Input<KeyCode>>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        *game_state = match *game_state {
            GameState::Paused => GameState::Simulating,
            GameState::Simulating => GameState::Paused,
        }
    }
}

fn mouse_input_system(
    mut board: ResMut<Board>,
    mouse_input: Res<Input<MouseButton>>,
    window: Query<&Window>,
) {
    let window = window.single();
    let cursor_position = window.cursor_position();

    if let Some(mut pos) = cursor_position {
        pos /= Vec2::new(
            window.resolution.physical_width() as f32,
            window.resolution.physical_height() as f32,
        );

        pos *= Vec2::new(board.width as f32, board.height as f32);

        let row = pos.x as usize;
        let col = pos.y as usize;
        let index = row * board.height + col;

        if mouse_input.pressed(MouseButton::Left) {
            board.cell_states[index] = CellState::Alive;
        } else if mouse_input.pressed(MouseButton::Right) {
            board.cell_states[index] = CellState::Dead;
        }
    }
}

fn scroll_input_system(
    mut scroll_evr: EventReader<MouseWheel>,
    mut simulation_timer: ResMut<SimulationTimer>,
) {
    use bevy::input::mouse::MouseScrollUnit;
    for ev in scroll_evr.iter() {
        if let MouseScrollUnit::Line = ev.unit {
            let duration = simulation_timer.0.duration();
            if ev.y > 0.0 {
                simulation_timer
                    .0
                    .set_duration(duration.mul_f32(ev.y * 0.9));
            } else {
                simulation_timer
                    .0
                    .set_duration(duration.mul_f32(ev.y.abs() * 1.1));
            }
        }
    }
}
