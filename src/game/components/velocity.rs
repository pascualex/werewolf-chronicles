use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Velocity {
    pub value: Vec2,
}

impl Velocity {
    pub fn from_vec2(value: Vec2) -> Self {
        Self { value }
    }
}
