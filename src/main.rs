use bevy::prelude::*;

mod components;
mod consts;
mod systems;

use components::*;
use consts::*;
use systems::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rogue Tut 2022".to_string(),
            width: 800.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(input)
        .add_system(movement)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
        )
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("glyphs.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8., 8.), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite {
            color: PLAYER_COLOR,
            index: 64,
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Position {x: TERM_WIDTH / 2, y: TERM_HEIGHT / 2});

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite {
            color: NPC_COLOR,
            index: 176,
            ..default()
        },
        ..default()
    })
    .insert(Npc)
    .insert(Position {x: TERM_WIDTH / 2 + 6, y: TERM_HEIGHT / 2});
}