use crate::{
    components::*, creatures::CreatureBundle, map_builder::*, player::PlayerBundle, term::*,
};
use bevy::prelude::*;
use bracket_lib::prelude::*;

const NUM_TILES: usize = TERM_WIDTH * TERM_HEIGHT;
const ROOM_MAX_SIZE: usize = 10;
const ROOM_MIN_SIZE: usize = 6;
const MAX_ROOMS: usize = 30;
const SHROUD: Color = Color::BLACK;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(dun_gen(
            MAX_ROOMS,
            ROOM_MIN_SIZE,
            ROOM_MAX_SIZE,
            TERM_WIDTH,
            TERM_HEIGHT,
        ))
        .add_startup_system(populate_map)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new().with_system(render_map),
        );
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Copy)]
pub struct TileColor {
    pub fg_color: Color,
    pub bg_color: Color
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub blocks_movement: bool,
    pub blocks_vision: bool,
    pub fg_index: usize,
    pub bg_index: usize,
    pub dark: TileColor,
    pub light: TileColor
}

pub struct CurrentMap {
    pub tiles: Vec<Tile>,
    pub creatures: Vec<Entity>,
    pub width: usize,
    pub height: usize,
    pub rooms: Vec<Room>,
    pub visible: Vec<bool>,
    pub explored: Vec<bool>
}
impl CurrentMap {
    pub fn new(map_width: usize, map_height: usize) -> Self {
        Self {
            tiles: vec![
                Tile {
                    tile_type: TileType::Wall,
                    blocks_movement: true,
                    blocks_vision: true,
                    fg_index: 12,
                    bg_index: 0,
                    dark: TileColor {
                        fg_color: Color::rgb(0.19, 0.19, 0.19), //Dark grey.
                        bg_color: Color::BLACK
                    },
                    light: TileColor {
                        fg_color: Color::rgb(0.83, 0.83, 0.83), //Light grey.
                        bg_color: Color::BLACK
                    }
                };
                NUM_TILES
            ],
            creatures: Vec::new(),
            width: map_width,
            height: map_height,
            rooms: Vec::new(),
            visible: vec![false; NUM_TILES],
            explored: vec![false; NUM_TILES]
        }
    }

    pub fn can_enter_tile<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        self.in_bounds(position) && (
            self.tiles[pos_index(position.x, position.y)].tile_type == TileType::Floor
        )
    }

    pub fn in_bounds<T: Into<Position>> (&self, position: T) -> bool {
        let position = position.into();
        position.x >= 0 
        && position.x < TERM_WIDTH    
        && position.y >= 0 
        && position.
        y < TERM_HEIGHT
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter_tile(destination) {
                let idx = self.point2d_to_index(destination);
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}
impl Algorithm2D for CurrentMap {
    fn dimensions(&self) -> bracket_lib::prelude::Point {
        bracket_lib::prelude::Point::new(TERM_WIDTH, TERM_HEIGHT)
    }

    fn in_bounds(&self, pt: bracket_lib::prelude::Point) -> bool {
        self.in_bounds(pt)
    }
}
impl BaseMap for CurrentMap {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> 
    {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0))
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0))
        }
        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1), self.index_to_point2d(idx2)
        )
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize].tile_type != TileType::Floor
    }
}

//Startup system to spawn/populate the map.
fn populate_map(mut commands: Commands, mut map: ResMut<CurrentMap>) {
    let (p_x, p_y) = map.rooms[0].center();
    map.creatures.push(
        commands
            .spawn_bundle(PlayerBundle {
                player: Player {},
                creature: Creature {},
                glyph: Glyph { index: 64 },
                pos: Position {
                    x: p_x,
                    y: p_y,
                    z: 1,
                },
            })
            .id(),
    );
    let ind = pos_index(p_x, p_y);
    map.tiles[ind].blocks_movement = true;

    map.creatures.push(
        commands
            .spawn_bundle(CreatureBundle {
                creature: Creature {},
                glyph: Glyph { index: 187 },
                pos: Position {
                    x: TERM_WIDTH / 2 + 4,
                    y: TERM_HEIGHT / 2,
                    z: 1,
                },
            })
            .id(),
    );
    let ind = pos_index(TERM_WIDTH / 2 + 4, TERM_HEIGHT / 2);
    map.tiles[ind].blocks_movement = true;
}

//Render to Terminal
fn render_map(
    term: Res<Terminal>,
    map: Res<CurrentMap>,
    mut t_query: Query<&mut TextureAtlasSprite>,
    e_query: Query<(&Position, &Glyph)>,
) {
    if map.is_changed() {
        //Draw map tiles.
        for y in 0..TERM_HEIGHT {
            for x in 0..TERM_WIDTH {
                let index = pos_index(x, y);
                let fg = term.fg_tiles[index];
                let bg = term.bg_tiles[index];
                if map.visible[index] {
                    if let Ok(mut sprite) = t_query.get_mut(fg) {
                        sprite.color = map.tiles[index].light.fg_color;
                        sprite.index = map.tiles[index].fg_index;
                    }
                    if let Ok(mut sprite) = t_query.get_mut(bg) {
                        sprite.color = map.tiles[index].light.bg_color;
                        sprite.index = map.tiles[index].bg_index;
                    }
                } else if map.explored[index] {
                    if let Ok(mut sprite) = t_query.get_mut(fg) {
                        sprite.color = map.tiles[index].dark.fg_color;
                        sprite.index = map.tiles[index].fg_index;
                    }
                    if let Ok(mut sprite) = t_query.get_mut(bg) {
                        sprite.color = map.tiles[index].dark.bg_color;
                        sprite.index = map.tiles[index].bg_index;
                    }
                } else {
                    if let Ok(mut sprite) = t_query.get_mut(fg) {
                        sprite.index = 0;
                        sprite.color = SHROUD;
                    }
                    if let Ok(mut sprite) = t_query.get_mut(bg) {
                        sprite.index = 0;
                        sprite.color = SHROUD;
                    }
                }
            }
        }

        //Draw creatures
        for e in map.creatures.iter() {
            if let Ok((pos, gly)) = e_query.get(*e) {
                let pos_index = pos_index(pos.x, pos.y);
                let id = term.fg_tiles[pos_index];
                if let Ok(mut sprite) = t_query.get_mut(id) {
                    sprite.index = gly.index;
                }
            }
        }
    }
}
