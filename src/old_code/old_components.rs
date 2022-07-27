use bevy::prelude::*;
use bracket_lib::prelude::*;

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

#[derive(Component, PartialEq, PartialOrd, Eq, Ord, Clone, Copy)]
pub struct Position {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}
impl From<Point> for Position {
    fn from(item: Point) -> Self {
        Position { x:item.x as usize, y:item.y as usize, z:0 }
    }
}
impl From<Position> for Point {
    fn from(item: Position) -> Self {
        Point { x:item.x as i32, y:item.y as i32}
    }
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

#[derive(Component)]
pub struct WantsToMove {
    pub dx: i32,
    pub dy: i32,
}
