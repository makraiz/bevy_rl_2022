use bevy::prelude::*;

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Name {
    pub name: String,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: usize,
    pub color: Color,
}

#[derive(Component)]
pub struct TileSize {
    pub size: f32,
}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<bracket_lib::prelude::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct WantsToMove {
    pub delta_x: i32,
    pub delta_y: i32,
}
