use crate::{components::*, map::CurrentMap, term::pos_index};
use bevy::prelude::*;
use bracket_lib::prelude::*;

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub glyph: Glyph,
    pub pos: Position,
}

pub struct CreaturesPlugin;
impl Plugin for CreaturesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(wants_to_move);
    }
}

fn wants_to_move(
    mut commands: Commands,
    mut movers: Query<(&mut Position, &WantsToMove, Entity)>,
    mut map: ResMut<CurrentMap>,
) {
    for (mut mover, dest, ent) in movers.iter_mut() {
        let dest_x = mover.x as i32 + dest.dx;
        let dest_y = mover.y as i32 + dest.dy;

        if dest_x >= 0 && dest_x < map.width as i32 && dest_y >= 0 && dest_y < map.height as i32 {
            let dest_ind = pos_index(dest_x as usize, dest_y as usize);
            if !map.tiles[dest_ind].blocks_movement {
                let org_ind = pos_index(mover.x, mover.y);
                map.tiles[org_ind].blocks_movement = false;
                map.tiles[dest_ind].blocks_movement = true;
                mover.x = dest_x as usize;
                mover.y = dest_y as usize;
            }
        }
        commands.entity(ent).remove::<WantsToMove>();
    }
}

fn update_fov(mut map: ResMut<CurrentMap>, query: Query<&Position, (With<Player>, Changed<Position>)>) {
    for pos in query.iter() {
        let pt = bracket_lib::prelude::Point::new(pos.x, pos.y);
        let range = 8;
        let visible_coords = field_of_view_set(pt, range, &*map);
        
    }
}