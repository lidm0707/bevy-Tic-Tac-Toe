use bevy::prelude::*;

#[derive(Component)]
pub enum ButtonCustome {
    Play,
    Exit,
}

#[derive(Bundle)]
pub struct ButtonBundle {
    pub button: Button,
    pub node: Node,
    pub interaction: Interaction,
    pub tag: ButtonCustome,
    pub background_color: BackgroundColor,
}

impl ButtonBundle {
    pub fn new(tag: ButtonCustome) -> Self {
        Self {
            button: Button,
            node: Node {
                width: Val::Px(300.0),
                height: Val::Px(65.0),
                margin: UiRect::all(Val::Px(20.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            interaction: Interaction::default(),
            tag,
            background_color: BackgroundColor(Color::srgb_u8(40, 40, 40)),
        }
    }
}

#[derive(Bundle)]
pub struct ButtonText {
    pub text: Text,
    pub font: TextFont,
    pub color: TextColor,
}

impl ButtonText {
    pub fn new(text: &str) -> Self {
        Self {
            text: Text::new(text),
            font: TextFont {
                font_size: 32.0,
                ..default()
            },
            color: TextColor(Color::WHITE),
        }
    }
}
