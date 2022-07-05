use bevy::prelude::*;

#[derive(Component)]
pub struct BgTile;

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct FgTile;

#[derive(Component)]
pub struct Glyph {
    pub index: usize,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

#[derive(Component)]
pub struct TileSize {
    pub width: f32,
    pub height: f32,
}
impl TileSize {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}
