use bevy::prelude::*;

#[derive(Component)]
pub struct GridLine;

#[derive(Bundle)]
pub struct GridBundle {
    pub grid: GridLine,
    pub transform: Transform,
    pub sprite: Sprite,
}

impl GridBundle {
    pub fn vertical(x: f32, y: f32, z: f32, thick: f32, cell: f32) -> Self {
        Self {
            grid: GridLine,
            transform: Transform::from_xyz(x, y, z),
            sprite: Sprite {
                color: Color::srgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(thick, cell * 3.0)),
                ..default()
            },
        }
    }

    pub fn horizontal(x: f32, y: f32, z: f32, thick: f32, cell: f32) -> Self {
        Self {
            grid: GridLine,
            transform: Transform::from_xyz(x, y, z),
            sprite: Sprite {
                color: Color::srgb(0.8, 0.8, 0.8),
                custom_size: Some(Vec2::new(cell * 3.0, thick)),
                ..default()
            },
        }
    }
}
