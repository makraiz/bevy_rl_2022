use bevy::prelude::*;
use crate::{components::*, creatures::CreatureBundle, player::PlayerBundle, term::*};

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CurrentMap::new())
        .add_startup_system(build_map)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
            .with_system(render_map)
        );

    }
}

const NUM_TILES: usize = TERM_WIDTH * TERM_HEIGHT;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor
}

pub struct CurrentMap {
    pub tiles: Vec<TileType>,
    pub creatures: Vec<Entity>
}
impl CurrentMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
            creatures: Vec::new()
        }
    }
}

//Startup system
fn build_map(mut commands: Commands, mut map: ResMut<CurrentMap>) {
    //Build some walls
    for i in 30..33 {
        map.tiles[i] = TileType::Wall;
    }

    map.creatures.push(commands.spawn_bundle(PlayerBundle {
        player: Player {},
        creature: Creature {},
        glyph: Glyph {index: 64},
        pos: Position {
            x: TERM_WIDTH / 2,
            y: TERM_HEIGHT / 2,
            z: 1
        }
    }).id());

    map.creatures.push(commands.spawn_bundle(CreatureBundle {
        creature: Creature {},
        glyph: Glyph {index: 187},
        pos: Position {x: TERM_WIDTH / 2 + 4, y: TERM_HEIGHT / 2, z: 1}
    }).id());
}

//Render to terminal
fn render_map(term: Res<Terminal>, map: Res<CurrentMap>, mut t_query: Query<&mut TextureAtlasSprite>, e_query: Query<(&Position, &Glyph)>) {
    if map.is_changed() {
        //Draw map tiles.
        for y in 0..TERM_HEIGHT {
            for x in 0..TERM_WIDTH {
                let index = pos_index(x, y);
                match map.tiles[index] {
                    TileType::Floor => {
                        let id = term.fg_tiles[index];
                        if let Ok(mut sprite) = t_query.get_mut(id) {
                            sprite.index = 0;
                        }
                    },
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
