use bevy::state::state::States;

#[derive(Debug, Default, States, Clone, Hash, Eq, PartialEq)]
pub enum GameState {
    #[default]
    Loading,
    Menu,
    Playing,
    Ended,
}
