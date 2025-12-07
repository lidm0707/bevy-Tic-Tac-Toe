use bevy::prelude::*;

use crate::{plugins::menu::components::button_custom::ButtonCustome, state::GameState};

pub fn button_interaction(
    mut interaction_q: Query<
        (&Interaction, &ButtonCustome, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, button, mut color) in &mut interaction_q {
        // let mut text = text_q.get_mut(children[0]).unwrap();

        match *interaction {
            Interaction::Pressed => {
                // สีเวลาถูกกด
                // *color = BackgroundColor(Color::srgb(0.2, 0.7, 0.2));
                match button {
                    ButtonCustome::Play => {
                        next_state.set(GameState::Playing);
                    }
                    ButtonCustome::Exit => {
                        exit.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.3, 0.3, 0.3));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.15, 0.15, 0.15));
            }
        }
    }
}
