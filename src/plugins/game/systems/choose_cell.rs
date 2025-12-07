use bevy::prelude::*;

use crate::{
    plugins::game::plugin::{Board, CELL, Turn},
    state::GameState,
};

pub fn click_spawn_circle(
    buttons: Res<ButtonInput<MouseButton>>,
    windows: Query<&Window>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut commands: Commands,
    mut turn: ResMut<Turn>,
    mut board: ResMut<Board>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    let window = windows.single().unwrap();
    let Some(cursor_pos) = window.cursor_position() else {
        return;
    };

    let (camera, cam_tf) = camera_q.single().unwrap();
    let ray = camera.viewport_to_world(cam_tf, cursor_pos).unwrap();

    let world = ray.origin.truncate();

    // === 1) คำนวณ cell index ===
    let half_board = CELL * 1.5; // 3 ช่อง
    if world.x < -half_board
        || world.x > half_board
        || world.y < -half_board
        || world.y > half_board
    {
        return;
    }

    let col = ((world.x + half_board) / CELL).floor() as usize;
    let row = ((world.y + half_board) / CELL).floor() as usize;

    if row > 2 || col > 2 {
        return;
    }

    if board.0[row][col] != ' ' {
        return; // มีแล้ว ห้ามเขียนซ้ำ
    }

    // === 3) คำนวณตำแหน่ง center ของ cell ===
    let cell_center_x = -half_board + (col as f32) * CELL + CELL / 2.0;
    let cell_center_y = -half_board + (row as f32) * CELL + CELL / 2.0;

    // === 4) วาง X หรือ O ===
    let symbol = if turn.0 { 'O' } else { 'X' };
    board.0[row][col] = symbol;

    match symbol {
        'X' => {
            // ใช้ Sprite เป็น X
            commands.spawn((
                DespawnOnExit(GameState::Playing),
                Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(80., 10.)),
                Transform::from_xyz(cell_center_x, cell_center_y, 10.0)
                    .with_rotation(Quat::from_rotation_z(0.8)),
            ));
            commands.spawn((
                DespawnOnExit(GameState::Playing),
                Sprite::from_color(Color::srgb(1.0, 0.2, 0.2), Vec2::new(80., 10.)),
                Transform::from_xyz(cell_center_x, cell_center_y, 10.0)
                    .with_rotation(Quat::from_rotation_z(-0.8)),
            ));
        }
        'O' => {
            commands.spawn((
                DespawnOnExit(GameState::Playing),
                Sprite::from_color(Color::srgb(0.2, 0.6, 1.0), Vec2::splat(70.)),
                Transform::from_xyz(cell_center_x, cell_center_y, 10.0),
            ));
        }
        _ => {}
    }

    //switch turn
    turn.0 = !turn.0;
}
