use crate::prelude::*;

//Moves the player if they want to move and the new position is in bounds
pub fn try_move(
    mut commands: Commands,
    mut movers: Query<(
        &mut Position,
        &WantsToMove,
        Entity,
        &mut Viewshed,
        Option<&Player>,
    )>,
    targets: Query<(&CombatStats, Entity)>,
    map: Res<Map>,
) {
    for (mut pos, delta, entity, mut viewshed, player) in movers.iter_mut() {
        let dest_idx = map.xy_idx(pos.x + delta.delta_x, pos.y + delta.delta_y);

        //Bump attack
        for (_stats, target) in targets.iter() {
            if map.tile_content[dest_idx].contains(&target) {
                commands.entity(entity).insert(WantsToMelee{target});
                commands.entity(entity).remove::<WantsToMove>();
                if let Some(_) = player {
                    commands.insert_resource(TurnState::MonsterTurn);
                }
                return
            }
        }

        if !map.blocked[dest_idx] {
            pos.x = min(MAP_WIDTH as i32 - 1, max(0, pos.x + delta.delta_x));
            pos.y = min(MAP_HEIGHT as i32 - 1, max(0, pos.y + delta.delta_y));

            viewshed.dirty = true;
        }
        commands.entity(entity).remove::<WantsToMove>();
        if let Some(_) = player {
            commands.insert_resource(TurnState::MonsterTurn);
        }
    }
}

//Listens and responds to keyboard input
pub fn keyboard_input(
    mut commands: Commands,
    player: Query<Entity, With<Player>>,
    keys: Res<bevy::input::Input<KeyCode>>,
) {
    let plyr = player.single();
    if keys.just_released(KeyCode::Left) || keys.just_released(KeyCode::Numpad4) {
        commands.entity(plyr).insert(WantsToMove {
            delta_x: -1,
            delta_y: 0,
        });
    } else if keys.just_released(KeyCode::Right) || keys.just_released(KeyCode::Numpad6) {
        commands.entity(plyr).insert(WantsToMove {
            delta_x: 1,
            delta_y: 0,
        });
    } else if keys.just_released(KeyCode::Up) || keys.just_released(KeyCode::Numpad8) {
        commands.entity(plyr).insert(WantsToMove {
            delta_x: 0,
            delta_y: -1,
        });
    } else if keys.just_released(KeyCode::Down) || keys.just_released(KeyCode::Numpad2) {
        commands.entity(plyr).insert(WantsToMove {
            delta_x: 0,
            delta_y: 1,
        });
    } else if keys.just_released(KeyCode::Numpad9) {
        commands.entity(plyr).insert(WantsToMove{
            delta_x: 1, delta_y: -1,
        });
    } else if keys.just_released(KeyCode::Numpad7) {
        commands.entity(plyr).insert(WantsToMove{
            delta_x: -1, delta_y: -1,
        });
    } else if keys.just_released(KeyCode::Numpad3) {
        commands.entity(plyr).insert(WantsToMove{
            delta_x: 1, delta_y: 1,
        });
    } else if keys.just_released(KeyCode::Numpad1) {
        commands.entity(plyr).insert(WantsToMove{
            delta_x: -1, delta_y: 1,
        });
    }
    else {
        return
    }
    commands.insert_resource(TurnState::PlayerTurn);
}
