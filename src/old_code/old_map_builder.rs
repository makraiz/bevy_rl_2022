use crate::{
    map::{CurrentMap, TileType},
    term::pos_index,
};
use bracket_lib::prelude::*;
use rand::Rng;

pub struct Room {
    pub x1: usize,
    pub x2: usize,
    pub y1: usize,
    pub y2: usize,
}
impl Room {
    pub fn new(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            x1: x,
            x2: x + width,
            y1: y,
            y2: y + height,
        }
    }

    pub fn center(&self) -> (usize, usize) {
        let center_x = (self.x1 + self.x2) / 2;
        let center_y = (self.y1 + self.y2) / 2;
        (center_x, center_y)
    }

    pub fn inner(&self) -> Vec<(usize, usize)> {
        let mut v = Vec::new();
        for y in self.y1 + 1..self.y2 {
            for x in self.x1 + 1..self.x2 {
                v.push((x, y))
            }
        }
        v
    }

    pub fn intersects(&self, other: &Room) -> bool {
        return self.x1 <= other.x2
            && self.x2 >= other.x1
            && self.y1 <= other.y2
            && self.y2 >= other.y1;
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

    for pt in Bresenham::new(
        Point {
            x: x1 as i32,
            y: y1 as i32,
        },
        Point {
            x: corner_x as i32,
            y: corner_y as i32,
        },
    ) {
        tunnel.push((pt.x as usize, pt.y as usize))
    }

    for pt in Bresenham::new(
        Point {
            x: corner_x as i32,
            y: corner_y as i32,
        },
        Point {
            x: x2 as i32,
            y: y2 as i32,
        },
    ) {
        tunnel.push((pt.x as usize, pt.y as usize))
    }

    tunnel
}

pub fn dun_gen(
    max_rooms: usize,
    room_min_size: usize,
    room_max_size: usize,
    map_width: usize,
    map_height: usize,
) -> CurrentMap {
    let mut dungeon = CurrentMap::new(map_width, map_height);
    let mut rng = rand::thread_rng();

    for _r in 0..max_rooms {
        let room_width = rng.gen_range(room_min_size..=room_max_size);
        let room_height = rng.gen_range(room_min_size..=room_max_size);

        let x = rng.gen_range(0..dungeon.width - room_width - 1);
        let y = rng.gen_range(0..dungeon.height - room_height - 1);

        let new_room = Room::new(x, y, room_width, room_height);
        let mut intersects = false;

        for other_room in &dungeon.rooms {
            if new_room.intersects(&other_room) {
                intersects = true;
            }
        }
        if intersects {
            continue;
        }

        for (x, y) in new_room.inner() {
            let index = pos_index(x, y);
            dungeon.tiles[index].tile_type = TileType::Floor;
            dungeon.tiles[index].blocks_movement = false;
        }

        if dungeon.rooms.is_empty() {
            dungeon.rooms.push(new_room);
            continue;
        } else {
            for (x, y) in tunnel(
                dungeon.rooms[dungeon.rooms.len() - 1].center(),
                new_room.center(),
            ) {
                let index = pos_index(x, y);
                dungeon.tiles[index].tile_type = TileType::Floor;
                dungeon.tiles[index].blocks_movement = false;
            }
        }
        dungeon.rooms.push(new_room);
    }

    dungeon
}
