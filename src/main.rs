use bevy::prelude::*;
use bevy_Tic_Tac_Toe::state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        // .add_systems(Startup, setup)
        // .add_systems(Update, update_bloom_settings)
        .run();
}

fn setup() {}
