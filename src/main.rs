mod components;
mod creatures;
mod map;
mod player;
mod term;

use bevy::prelude::*;
use map::MapPlugin;
use term::BevyTermPlugin;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 500.,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(BevyTermPlugin)
        .add_plugin(MapPlugin)
        .run();
}
