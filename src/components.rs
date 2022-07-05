use bevy::prelude::Component;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct MapTile;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
pub struct WantsToMove {
    pub dx: i32,
    pub dy: i32
}
