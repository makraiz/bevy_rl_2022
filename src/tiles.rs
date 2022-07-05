use crate::consts::*;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct Map {
    pub tiles: Vec<TileType>
}
impl Map {
    pub fn new() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Floor; (MAP_WIDTH * MAP_HEIGHT) as usize]
        };
        for i in 30..33 { 
            map.tiles[i] = TileType::Wall;
        }
        map
    }
}

pub fn map_index(x: i32, y: i32) -> usize {
    ((y * MAP_WIDTH) + x) as usize
}

pub fn map_index_xy(index: usize) -> (i32, i32) {
    (index as i32 % MAP_WIDTH, index as i32 / MAP_WIDTH)
}