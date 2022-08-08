use crate::prelude::*;

pub fn monster_ai_system(
    mut commands: Commands,
    map: Res<Map>,
    mut query: Query<(Entity, &mut Viewshed, &mut Position), (With<Monster>, Without<Player>)>,
    player: Query<(Entity, &Position), With<Player>>,
) {
    for (ent, view, mut pos) in query.iter_mut() {
        let (p_ent, p_pos) = player.single();
        let pt = Point::from_tuple((p_pos.x, p_pos.y));
        let distance = bracket_lib::prelude::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), pt);
        if distance < 1.5 {
            commands.entity(ent).insert(WantsToMelee{target: p_ent});
            continue
        }
        if view.visible_tiles.contains(&pt) {
            let path = bracket_lib::prelude::a_star_search(map.xy_idx(pos.x, pos.y) as i32, map.xy_idx(p_pos.x, p_pos.y) as i32, &*map);
            if path.success && path.steps.len() > 1 {
                pos.x = path.steps[1] as i32 % map.width;
                pos.y = path.steps[1] as i32 / map.width;
            }
        }
    }
    commands.insert_resource(TurnState::AwaitingInput);
}
