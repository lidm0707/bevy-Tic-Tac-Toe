use bevy::prelude::*;

use crate::{
    plugins::game::{
        components::grids::GridBundle,
        plugin::{CELL, THICK},
    },
    state::GameState,
};
pub fn spawn_grid(mut commands: Commands) {
    let half = CELL / 2.0;

    // ------------ เส้นตั้ง 2 เส้น ------------
    for x in [-half, half] {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            GridBundle::vertical(x, 0.0, 1.0, THICK, CELL),
        ));
    }

    // ------------ เส้นนอน 2 เส้น ------------
    for y in [-half, half] {
        commands.spawn((
            DespawnOnExit(GameState::Playing),
            GridBundle::horizontal(0.0, y, 1.0, THICK, CELL),
        ));
    }
}
