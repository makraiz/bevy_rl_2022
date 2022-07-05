use bevy::prelude::*;
use crate::components::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub creature: Creature,
    pub glyph: Glyph,
    pub pos: Position
}