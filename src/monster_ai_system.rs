use bevy::prelude::*;
use bracket_lib::prelude::Point as Point;
use crate::components::Name as Name;

use crate::{
    components::{Monster, Player, Position, Viewshed},
    RunState,
};

pub fn monster_ai_system(
    query: Query<(&Viewshed, &Position, &Name), With<Monster>>,
    mut state: ResMut<State<RunState>>,
    player: Query<(&Position, &Name), With<Player>>,
) {
    for (view, _pos, name) in query.iter() {
        let (plyr, pname) = player.single();
        let pt = Point::from_tuple((plyr.x, plyr.y));
        if view.visible_tiles.contains(&pt) {
            println!("{} shouts an insult at {}!", name.name, pname.name);
        }
    }
    let _ = state.set(RunState::Paused);
}
