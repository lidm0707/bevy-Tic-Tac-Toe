use bevy::prelude::*;
use bevy_Tic_Tac_Toe::{
    plugins::{game::plugin::game_plugin, menu::plugin::menu_plugin},
    state::GameState,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_camera)
        .init_state::<GameState>()
        .add_plugins((menu_plugin, game_plugin))
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}
