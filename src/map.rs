use crate::rect::Rect;
use crate::*;
use bevy::prelude::*;
use bracket_lib::prelude::*;
use std::cmp::{max, min};

//Constants
pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 50;

//Components
#[derive(Component)]
pub struct MapTile;

// Resources
#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

#[derive(Clone)]
pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
    pub blocked: Vec<bool>,
    pub tile_content: Vec<Vec<Entity>>
}
impl Map {
    ///Returns a tiles index for a tile at position x, y
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        ((y * self.width) + x) as usize
    }

    ///Applies a Rect-shaped room
    fn apply_room_to_map(&mut self, room: &Rect) {
        for y in room.y1 + 1..=room.y2 {
            for x in room.x1 + 1..=room.x2 {
                let idx = self.xy_idx(x, y);
                self.tiles[idx] = TileType::Floor;
            }
        }
    }

    ///Applies a horizontal tunnel
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        for x in min(x1, x2)..=max(x1, x2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    //Applies a vertical tunnel
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        for y in min(y1, y2)..=max(y1, y2) {
            let idx = self.xy_idx(x, y);
            if idx > 0 && idx < self.width as usize * self.height as usize {
                self.tiles[idx as usize] = TileType::Floor;
            }
        }
    }

    //Helper function for BaseMap implementation.  
    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y > self.height - 1 || y < 1 {return false}
        let idx = self.xy_idx(x, y);
        !self.blocked[idx]
    }

    ///Creates a new Map.
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![TileType::Wall; MAP_WIDTH * MAP_HEIGHT],
            rooms: Vec::new(),
            width: MAP_WIDTH as i32,
            height: MAP_HEIGHT as i32,
            revealed_tiles: vec![false; MAP_WIDTH * MAP_HEIGHT],
            visible_tiles: vec![false; MAP_WIDTH * MAP_HEIGHT],
            blocked: vec![false; MAP_WIDTH * MAP_HEIGHT],
            tile_content: vec![Vec::new(); MAP_WIDTH * MAP_HEIGHT],
        };

        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, 80 - w - 1) - 1;
            let y = rng.roll_dice(1, 50 - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                map.apply_room_to_map(&new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        map.apply_horizontal_tunnel(prev_x, new_x, prev_y);
                        map.apply_vertical_tunnel(prev_y, new_y, new_x);
                    } else {
                        map.apply_vertical_tunnel(prev_y, new_y, prev_x);
                        map.apply_horizontal_tunnel(prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }

        map
    }

    pub fn populate_blocked(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked[i] = *tile == TileType::Wall;
        }
    }

    pub fn clear_content_index(&mut self) {
        for content in self.tile_content.iter_mut() {
            content.clear();
        }
    }
}
impl Algorithm2D for Map {
    fn dimensions(&self) -> bracket_lib::prelude::Point {
        bracket_lib::prelude::Point::new(self.width, self.height)
    }
}
impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }

    fn get_available_exits(&self, idx:usize) -> bracket_lib::prelude::SmallVec<[(usize, f32); 10]> {
        let mut exits = bracket_lib::prelude::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;
    
        // Cardinal directions
        if self.is_exit_valid(x-1, y) { exits.push((idx-1, 1.0)) };
        if self.is_exit_valid(x+1, y) { exits.push((idx+1, 1.0)) };
        if self.is_exit_valid(x, y-1) { exits.push((idx-w, 1.0)) };
        if self.is_exit_valid(x, y+1) { exits.push((idx+w, 1.0)) };

        //Diagonals
        if self.is_exit_valid(x-1, y-1) { exits.push(((idx-w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y-1) { exits.push(((idx-w)+1, 1.45)); }
        if self.is_exit_valid(x-1, y+1) { exits.push(((idx+w)-1, 1.45)); }
        if self.is_exit_valid(x+1, y+1) { exits.push(((idx+w)+1, 1.45)); }
    
        exits
    }
    
    fn get_pathing_distance(&self, idx1:usize, idx2:usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        bracket_lib::prelude::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }
}

//Renders the map.
pub fn draw_map(
    map: Res<Map>,
    mut commands: Commands,
    glyphs: Res<Glyphs>,
    mut tiles: Query<(&Position, &mut TextureAtlasSprite), With<MapTile>>,
) {
    if map.is_changed() {
        let mut x = 0;
        let mut y = 0;

        'outer: for (idx, tile) in map.tiles.iter().enumerate() {
            if map.revealed_tiles[idx] {
                //To ensure we don't create more entities for already existing tiles.
                for (pos, mut render) in tiles.iter_mut() {
                    if pos.x == x && pos.y == y {
                        if !map.visible_tiles[idx] {
                            render.color = Color::GRAY;
                        } else if map.tiles[idx] == TileType::Wall {
                            render.color = Color::GREEN;
                        }
                        x += 1;
                        if x > 79 {
                            x = 0;
                            y += 1;
                        }
                        continue 'outer;
                    }
                }

                //Spawn a revealed tile.
                match tile {
                    TileType::Floor => {
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    color: Color::GRAY,
                                    index: 0,
                                    ..default()
                                },
                                texture_atlas: glyphs.handle.clone(),
                                ..default()
                            })
                            .insert(TileSize { size: 1. })
                            .insert(Position { x, y, z: 0 })
                            .insert(Renderable {
                                glyph: 0,
                                color: Color::GRAY,
                            })
                            .insert(MapTile {});
                    }
                    TileType::Wall => {
                        commands
                            .spawn_bundle(SpriteSheetBundle {
                                sprite: TextureAtlasSprite {
                                    color: Color::GREEN,
                                    index: 11,
                                    ..default()
                                },
                                texture_atlas: glyphs.handle.clone(),
                                ..default()
                            })
                            .insert(TileSize { size: 1. })
                            .insert(Position { x, y, z: 1 })
                            .insert(Renderable {
                                glyph: 11,
                                color: Color::GREEN,
                            })
                            .insert(MapTile {});
                    }
                };
            }

            //Increase the iterator
            x += 1;
            if x > 79 {
                x = 0;
                y += 1;
            }
        }
    }
}

//Creates a new Map.  Old, ugly version.  Randomly splatted walls.
pub fn new_map_test() -> Map {
    let mut map = Map {
        tiles: vec![TileType::Floor; MAP_WIDTH * MAP_HEIGHT],
        rooms: Vec::new(),
        width: MAP_WIDTH as i32,
        height: MAP_HEIGHT as i32,
        revealed_tiles: vec![false; MAP_WIDTH * MAP_HEIGHT],
        visible_tiles: vec![false; MAP_WIDTH * MAP_HEIGHT],
        blocked: vec![false; MAP_WIDTH * MAP_HEIGHT],
        tile_content: vec![Vec::new(); MAP_WIDTH * MAP_HEIGHT],
    };

    //Make the boundaries walls
    for x in 0..MAP_WIDTH {
        let idx = map.xy_idx(x as i32, 0);
        let idx2 = map.xy_idx(x as i32, (MAP_HEIGHT - 1) as i32);
        map.tiles[idx] = TileType::Wall;
        map.tiles[idx2] = TileType::Wall;
    }
    for y in 0..MAP_HEIGHT {
        let idx = map.xy_idx(0, y as i32);
        let idx2 = map.xy_idx((MAP_WIDTH - 1) as i32, y as i32);
        map.tiles[idx] = TileType::Wall;
        map.tiles[idx2] = TileType::Wall;
    }

    //Random wall placement
    let mut rng = RandomNumberGenerator::new();
    for _ in 0..400 {
        let x = rng.roll_dice(1, (MAP_WIDTH - 1) as i32);
        let y = rng.roll_dice(1, (MAP_HEIGHT - 1) as i32);
        let index = map.xy_idx(x, y);
        if index != map.xy_idx(40, 25) {
            map.tiles[index] = TileType::Wall;
        }
    }

    map
}
