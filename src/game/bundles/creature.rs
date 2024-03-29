use bevy::prelude::*;

use crate::game::components::{Health, MovementStats, Position, Size, Velocity};

#[derive(Bundle, Default)]
pub struct CreatureBundle {
    pub position: Position,
    pub size: Size,
    pub velocity: Velocity,
    pub health: Health,
    pub movement_stats: MovementStats,
    #[bundle]
    pub sprite_bundle: SpriteBundle,
}
