use crate::{
    plugins::menu::systems::{button::button_interaction, menu::main_menu_setup},
    state::GameState,
};
use bevy::prelude::*;

pub fn menu_plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Menu), main_menu_setup)
        .add_systems(Update, button_interaction.run_if(in_state(GameState::Menu)));
}
