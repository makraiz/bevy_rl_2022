use bevy::prelude::{Color, Component};
use crate::consts::*;

struct TileGfx {
    glyph: usize,
    fg: Color,
    bg: Color
}

#[derive(Component, Copy, Clone)]
struct Tile {
    passable: bool, //False if tile blocks movement
    transparent: bool, //False if tile blocks visibility
    dark: TileGfx, //gfx for when tile is seen but not in fov
}
impl Tile {
    fn new_floor() -> Self {
        Tile {
            passable: true,
            transparent: true,
            dark: TileGfx {
                glyph: 13,
                fg: UNVISIBLE,
                bg: CLEAR
            }
        }
    }

    fn new_wall() -> Self {
        Tile {
            passable: false,
            transparent: false,
            dark: TileGfx {
                glyph: 12,
                fg: UNVISIBLE,
                bg: CLEAR
            }
        }
    }
}