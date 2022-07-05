use bevy::prelude::*;
use crate::components::*;

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub glyph: Glyph,
    pub pos: Position
}