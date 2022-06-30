use bevy::prelude::Color;

pub const PLAYER_COLOR: Color = Color::rgb(0., 1., 0.);
pub const NPC_COLOR: Color = Color::rgb(1., 0., 0.);
pub const UNVISIBLE: Color = Color::rgb(0.5, 0.5, 0.4); //Color for seen but no longer in fov. 
pub const CLEAR: Color = Color::rgb(0., 0., 0.);
pub const TERM_WIDTH: i32 = 80;
pub const TERM_HEIGHT: i32 = 50;