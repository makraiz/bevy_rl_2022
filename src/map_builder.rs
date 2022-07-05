use bracket_lib::prelude::*;
use crate::{map::{CurrentMap, TileType}, term::pos_index};
use rand::Rng;


const NUM_ROOMS: usize = 20;

struct Rect {
    x1: usize,
    x2: usize,
    y1: usize,
    y2: usize
}
impl Rect {
    fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x1: x,
            x2: x + width,
            y1: y,
            y2: y + height
        }
    }

    fn center(&self) -> (usize, usize) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    fn inner(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for y in self.y1 + 1..self.y2 {
            for x in self.x1 + 1..self.x2 {
                v.push((x, y))
            }
        }
        v
    }
}

fn tunnel(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    let mut tunnel = Vec::new();
    let mut rng = rand::thread_rng();
    let corner_x;
    let corner_y;

    let (x1, y1) = start;
    let (x2, y2) = end;

    if rng.gen_range(0..10) >= 5 as i32 {
        (corner_x, corner_y) = (x2, y1);
    } else {
        (corner_x, corner_y) = (x1, y2);
    }

    for (x, y) in Bresenham::new(Point{x1, y1}, Point{corner_x, corner_y}) {
        tunnel.push()
    }

    tunnel
}

pub fn dun_gen(map_width: usize, map_height: usize) -> CurrentMap {
    let mut dungeon = CurrentMap::new(map_width, map_height);

    let room1 = Rect::new(20, 15, 10, 15);
    let room2 = Rect::new(35, 15, 10, 15);

    for (x, y) in room1.inner() {
        let index = pos_index(x, y);
        dungeon.tiles[index].tile_type = TileType::Floor;
        dungeon.tiles[index].is_blocked = false;
    }

    for (x, y) in room2.inner() {
        let index = pos_index(x, y);
        dungeon.tiles[index].tile_type = TileType::Floor;
        dungeon.tiles[index].is_blocked = false;
    }

    dungeon
}