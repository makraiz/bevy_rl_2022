use bevy::prelude::*;
use bracket_lib::prelude::Point as Point;

use crate::{
    components::{Monster, Player, Position, Viewshed, Name as Name},
    RunState, Map
};

pub fn monster_ai_system(
    map: Res<Map>,
    mut query: Query<(&mut Viewshed, &mut Position, &Name), (With<Monster>, Without<Player>)>,
    player: Query<(&Position, &Name), With<Player>>,
    mut state: ResMut<State<RunState>>,
) {
    for (view, mut pos, name) in query.iter_mut() {
        let (plyr, pname) = player.single();
        let pt = Point::from_tuple((plyr.x, plyr.y));
        let distance = bracket_lib::prelude::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), pt);
        if distance < 1.5 {
            //attack goes here.
            println!("{} shouts \"Your mother was a hamster and your father smelled of elderberries!\" at {}!", name.name, pname.name);
            continue
        }
        if view.visible_tiles.contains(&pt) {
            let path = bracket_lib::prelude::a_star_search(map.xy_idx(pos.x, pos.y) as i32, map.xy_idx(plyr.x, plyr.y) as i32, &*map);
            if path.success && path.steps.len() > 1 {
                pos.x = path.steps[1] as i32 % map.width;
                pos.y = path.steps[1] as i32 / map.width;
            }
        }
    }
    let _ = state.set(RunState::Paused);
}
