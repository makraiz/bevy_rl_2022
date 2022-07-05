use crate::{components::*, creatures::CreatureBundle, map_builder::dun_gen, player::PlayerBundle, term::*};
use bevy::prelude::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(dun_gen(TERM_WIDTH, TERM_HEIGHT))
            .add_startup_system(build_map)
            .add_system_set_to_stage(
                CoreStage::PostUpdate,
                SystemSet::new().with_system(render_map),
            );
    }
}

const NUM_TILES: usize = TERM_WIDTH * TERM_HEIGHT;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Clone, Copy)]
pub struct Tile {
    pub tile_type: TileType,
    pub is_blocked: bool,
}

pub struct CurrentMap {
    pub tiles: Vec<Tile>,
    pub creatures: Vec<Entity>,
    pub blocked: Vec<Position>,
    pub width: usize,
    pub height: usize,
}
impl CurrentMap {
    pub fn new(map_width: usize, map_height: usize) -> Self {
        Self {
            tiles: vec![
                Tile {
                    tile_type: TileType::Wall,
                    is_blocked: true
                };
                NUM_TILES
            ],
            creatures: Vec::new(),
            blocked: Vec::new(),
            width: map_width,
            height: map_height,
        }
    }
}

//Startup system
fn build_map(mut commands: Commands, mut map: ResMut<CurrentMap>) {
    //populate the map.  
    map.creatures.push(
        commands
            .spawn_bundle(PlayerBundle {
                player: Player {},
                creature: Creature {},
                glyph: Glyph { index: 64 },
                pos: Position {
                    x: TERM_WIDTH / 2,
                    y: TERM_HEIGHT / 2,
                    z: 1,
                },
            })
            .id(),
    );
    let ind = pos_index(TERM_WIDTH / 2, TERM_HEIGHT / 2);
    map.tiles[ind].is_blocked = true;

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
    map.tiles[ind].is_blocked = true;
}

//Render to terminal
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
                match map.tiles[index].tile_type {
                    TileType::Floor => {
                        let id = term.fg_tiles[index];
                        if let Ok(mut sprite) = t_query.get_mut(id) {
                            sprite.index = 0;
                        }
                    }
                    TileType::Wall => {
                        let id = term.fg_tiles[index];
                        if let Ok(mut sprite) = t_query.get_mut(id) {
                            sprite.index = 11;
                        }
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
