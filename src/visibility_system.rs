use crate::{
    components::{Player, Position, Viewshed},
    map::Map,
};
use bevy::prelude::*;
use bracket_lib::prelude::{field_of_view, Point};

pub fn visibility_system(
    mut map: ResMut<Map>,
    mut query: Query<(&mut Viewshed, &Position, Option<&Player>)>,
) {
    for (mut viewshed, pos, player) in query.iter_mut() {
        if viewshed.dirty {
            viewshed.dirty = false;
            viewshed.visible_tiles.clear();
            viewshed.visible_tiles = field_of_view(Point::new(pos.x, pos.y), viewshed.range, &*map);
            viewshed
                .visible_tiles
                .retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);

            if let Some(_player) = player {
                for t in map.visible_tiles.iter_mut() {
                    *t = false
                }
                for vis in viewshed.visible_tiles.iter() {
                    let idx = map.xy_idx(vis.x, vis.y);
                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        }
    }
}
