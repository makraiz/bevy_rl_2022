use bevy::prelude::*;
use crate::components::*;
use crate::consts::*;

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        let z = transform.translation.to_array()[2];
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, MAP_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, MAP_HEIGHT as f32),
            z
        );
    }
}

pub fn input(keys: Res<Input<KeyCode>>, mut commands: Commands, player: Query<Entity, With<Player>>) {
    for p in player.iter() {
        if keys.just_pressed(KeyCode::Left) {
            commands.entity(p).insert(WantsToMove{dx: -1, dy: 0});
        }
        if keys.just_pressed(KeyCode::Right) {
            commands.entity(p).insert(WantsToMove{dx: 1, dy: 0});
        }
        if keys.just_pressed(KeyCode::Down) {
            commands.entity(p).insert(WantsToMove{dx: 0, dy: -1});
        }
        if keys.just_pressed(KeyCode::Up) {
            commands.entity(p).insert(WantsToMove{dx: 0, dy: 1});
        }
    }
}

pub fn movement(mut commands: Commands, mut positions: Query<(&mut Position, &WantsToMove, Entity)>) {
    for (mut pos, dest, ent) in positions.iter_mut() {
        pos.x += dest.dx;
        pos.y += dest.dy;
        commands.entity(ent).remove::<WantsToMove>();
    }
}