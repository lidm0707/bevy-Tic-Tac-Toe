use bevy::state::state::States;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    // Loading,
    #[default]
    Menu,
    Playing,
    Ended,
}
