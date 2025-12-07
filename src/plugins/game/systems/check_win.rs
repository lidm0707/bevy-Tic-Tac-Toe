use crate::plugins::game::plugin::{Board, Winner};
use crate::state::GameState;
use bevy::prelude::*;

// ===============================
//  CHECK WINNER (ระบบ Bevy)
// ===============================
pub fn check_winner(
    board: Res<Board>,
    mut next_state: ResMut<NextState<GameState>>,
    mut winner: ResMut<Winner>,
) {
    if let Some(winner_char) = get_winner(&board.0) {
        winner.0 = Some(winner_char); // <<=== ตรงนี้คือสิ่งที่ต้องใช้

        println!("Winner = {}", winner_char);
        next_state.set(GameState::Ended);
    }
}

// ===============================
//  GET WINNER (pure function)
// ===============================
pub fn get_winner(board: &[[char; 3]; 3]) -> Option<char> {
    // --- แนวนอน ---
    for row in 0..3 {
        if board[row][0] != ' ' && board[row][0] == board[row][1] && board[row][1] == board[row][2]
        {
            return Some(board[row][0]);
        }
    }

    // --- แนวตั้ง ---
    for col in 0..3 {
        if board[0][col] != ' ' && board[0][col] == board[1][col] && board[1][col] == board[2][col]
        {
            return Some(board[0][col]);
        }
    }

    // --- ทแยงมุมหลัก ---
    if board[0][0] != ' ' && board[0][0] == board[1][1] && board[1][1] == board[2][2] {
        return Some(board[0][0]);
    }

    // --- ทแยงมุมรอง ---
    if board[0][2] != ' ' && board[0][2] == board[1][1] && board[1][1] == board[2][0] {
        return Some(board[0][2]);
    }

    // --- เสมอ ---
    let mut full = true;
    for row in 0..3 {
        for col in 0..3 {
            if board[row][col] == ' ' {
                full = false;
            }
        }
    }

    if full {
        Some('T') // tie
    } else {
        None
    }
}
