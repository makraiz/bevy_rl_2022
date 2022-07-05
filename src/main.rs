mod components;
mod map;
mod term;

use bevy::prelude::*;
use term::BevyTermPlugin;
use map::MapPlugin;

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