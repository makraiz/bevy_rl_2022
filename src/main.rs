use bevy::prelude::*;

mod components;
mod consts;
mod systems;
mod tiles;

use components::*;
use consts::*;
use systems::*;
use tiles::*;

//Still to implement from Chapter 2: bounds checking on movement, map, tiles

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Rogue Tut 2022".to_string(),
            width: 800.0,
            height: 500.0,
            ..default()
        })
        .insert_resource(ClearColor(CLEAR))
        .add_startup_system(setup)
        .add_plugins(DefaultPlugins)
        .add_system(input.before(movement))
        .add_system(movement.after(input))
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
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Position {x: MAP_WIDTH / 2, y: MAP_HEIGHT / 2});

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle.clone(),
        sprite: TextureAtlasSprite {
            color: NPC_COLOR,
            index: 176,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 1.),
            ..default()
        },
        ..default()
    })
    .insert(Npc)
    .insert(Position {x: MAP_WIDTH / 2 + 6, y: MAP_HEIGHT / 2});

    //Demo map
    let map = Map::new();
    for y in 0..MAP_HEIGHT {
        for x in 0..MAP_WIDTH {
            let index = map_index(x, y);
            match map.tiles[index] {
                TileType::Wall => {
                    commands.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite {
                            color: WALL_COLOR,
                            index: 12,
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0., 0., 1.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(MapTile)
                    .insert(Position{x, y});
                },
                TileType::Floor => {
                    commands.spawn_bundle(SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle.clone(),
                        sprite: TextureAtlasSprite {
                            color: FLOOR_COLOR,
                            index: 20,
                            ..default()
                        },
                        transform: Transform {
                            translation: Vec3::new(0., 0., 0.),
                            ..default()
                        },
                        ..default()
                    })
                    .insert(MapTile)
                    .insert(Position{x, y});
                }
            }
        }
    }

    commands.insert_resource(map);
}