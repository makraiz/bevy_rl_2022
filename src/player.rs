use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub creature: Creature,
    pub glyph: Glyph,
    pub pos: Position,
}
