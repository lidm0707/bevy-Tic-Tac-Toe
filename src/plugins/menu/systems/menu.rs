use bevy::prelude::*;

use crate::{
    plugins::menu::components::button_custom::{ButtonBundle, ButtonCustome, ButtonText},
    state::GameState,
};

pub fn main_menu_setup(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Menu),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            // ===== TITLE =====
            (
                Text::new("Bevy Game Menu UI"),
                TextFont {
                    font_size: 60.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    margin: UiRect::all(Val::Px(40.0)),
                    ..default()
                },
            ),
            // ===== PLAY BUTTON =====
            (
                ButtonBundle::new(ButtonCustome::Play),
                children![(ButtonText::new("Play"))]
            ),
            // ===== EXIT BUTTON =====
            (
                ButtonBundle::new(ButtonCustome::Exit),
                children![(ButtonText::new("Exit"))]
            ),
        ],
    ));
}
