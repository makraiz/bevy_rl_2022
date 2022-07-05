mod components;
mod creatures;
mod input;
mod map;
mod map_builder;
mod player;
mod term;

use bevy::prelude::*;
use creatures::CreaturesPlugin;
use input::InputPlugin;
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
        .add_plugin(CreaturesPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(InputPlugin)
        .run();
}
