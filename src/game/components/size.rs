use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Size {
    pub value: Vec2,
}

impl Size {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            value: Vec2::new(x, y),
        }
    }

    pub fn from_vec2(value: Vec2) -> Self {
        Self { value }
    }
}
