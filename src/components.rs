use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32
}

impl Default for Velocity {
    fn default() -> Self {
       Self {
           x: 0.0,
           y: 0.0
       }
    }
}

#[derive(Component)]
pub struct Movement;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(value: (f32, f32)) -> Self {
       Self(Vec2::new(value.0, value.1))
    }
}

impl From<f32> for SpriteSize {
    fn from(value: f32) -> Self {
        Self(Vec2::new(value, value))
    }
}

#[derive(Component)]
pub struct Qi {
    pub amount: f32,
    pub speed: f32,
    pub max_value: f32,
}

impl Default for Qi {
    fn default() -> Self {
       Self {
           amount: 0.,
           speed: 0.1,
           max_value: 100.0,
       }
    }
}

#[derive(Component)]
pub struct TextChanges;