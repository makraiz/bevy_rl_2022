use crate::prelude::*;

pub fn indexing_system(mut map: ResMut<Map>, query: Query<(&Position, Option<&BlocksTile>, Entity), Without<UiTile>>) {
    map.populate_blocked();
    map.clear_content_index();

    for (position, blocker, entity) in query.iter() {
        let idx = map.xy_idx(position.x, position.y);
        if let Some(_) = blocker {
            map.blocked[idx] = true;
        }
        map.tile_content[idx].push(entity);
    }
}