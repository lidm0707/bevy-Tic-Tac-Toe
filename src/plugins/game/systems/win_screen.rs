use bevy::prelude::*;

use crate::{
    plugins::game::plugin::{Board, Winner},
    state::GameState,
};

pub fn setup_win_screen(mut commands: Commands, winner: Res<Winner>) {
    let text = match winner.0 {
        Some('X') => "Player X Wins!",
        Some('O') => "Player O Wins!",
        Some('T') => "Tie!",
        _ => "No Winner",
    };

    commands.spawn((
        DespawnOnExit(GameState::Ended),
        Text::new(text),
        TextFont {
            font_size: 48.0,
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

pub fn return_to_menu_on_enter(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut winner: ResMut<Winner>,
    mut board: ResMut<Board>,
) {
    if keyboard.just_pressed(KeyCode::Enter) {
        // reset winner
        winner.0 = None;
        board.0 = [[' '; 3]; 3];

        // back to menu
        next_state.set(GameState::Menu);
    }
}
