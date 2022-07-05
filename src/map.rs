use bevy::prelude::*;
use crate::term::*;

pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(CurrentMap::new())
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
    pub tiles: Vec<TileType>
}
impl CurrentMap {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }
}

fn render_map(term: Res<Terminal>, map: Res<CurrentMap>, mut query: Query<&mut TextureAtlasSprite>) {
    if map.is_changed() {
        for y in 0..TERM_HEIGHT {
            for x in 0..TERM_WIDTH {
                let index = pos_index(x, y);
                match map.tiles[index] {
                    TileType::Floor => {
                        let id = term.fg_tiles[index];
                        if let Ok(mut sprite) = query.get_mut(id) {
                            sprite.index = 0;
                        }
                    },
                    TileType::Wall => {
                        let id = term.fg_tiles[index];
                        if let Ok(mut sprite) = query.get_mut(id) {
                            sprite.index = 11;
                        }
                    }
                }
            }
        }
    }
}