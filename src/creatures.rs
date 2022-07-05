use crate::components::*;
use bevy::prelude::*;

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub glyph: Glyph,
    pub pos: Position,
}
