use bevy::{app::AppExit, prelude::*};
use crate::components::{Player, WantsToMove};

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(keyboard_input);
    }
}

fn keyboard_input(keys: Res<Input<KeyCode>>, query: Query<Entity, With<Player>>, mut commands: Commands, mut exit: EventWriter<AppExit>) {
    let q = query.single();
    if keys.just_released(KeyCode::Up) {
        commands.entity(q).insert(WantsToMove {dx: 0, dy: 1});
    }
    if keys.just_released(KeyCode::Down) {
        commands.entity(q).insert(WantsToMove {dx: 0, dy: -1});
    }
    if keys.just_released(KeyCode::Left) {
        commands.entity(q).insert(WantsToMove {dx: -1, dy: 0});
    }
    if keys.just_released(KeyCode::Right) {
        commands.entity(q).insert(WantsToMove {dx: 1, dy: 0});
    }
    if keys.just_released(KeyCode::Escape) {
        exit.send(AppExit)
    }
}