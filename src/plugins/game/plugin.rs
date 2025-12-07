use bevy::prelude::*;

use crate::{
    plugins::game::systems::{
        check_win::check_winner,
        choose_cell::click_spawn_circle,
        create_grid::spawn_grid,
        win_screen::{return_to_menu_on_enter, setup_win_screen},
    },
    state::GameState,
};

#[derive(Resource)]
pub struct Turn(pub bool); // false = X, true = O
#[derive(Resource)]
pub struct Board(pub [[char; 3]; 3]);

#[derive(Resource)]
pub struct Winner(pub Option<char>);

pub const CELL: f32 = 120.0; // ขนาดของแต่ละช่อง
pub const THICK: f32 = 6.0; // ความหนาเส้น

pub fn game_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Playing), spawn_grid)
        .insert_resource(Turn(false))
        .insert_resource(Board([[' '; 3]; 3]))
        .insert_resource(Winner(None))
        .add_systems(
            Update,
            (click_spawn_circle, check_winner)
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .add_systems(OnEnter(GameState::Ended), setup_win_screen)
        .add_systems(
            Update,
            return_to_menu_on_enter.run_if(in_state(GameState::Ended)),
        );
}
