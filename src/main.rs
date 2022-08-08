mod components;
mod combat_system;
mod map;
mod map_indexing_system;
mod monster_ai_system;
mod player;
mod rect;
mod visibility_system;

mod prelude {
    pub use crate::components::{*, Name as Name};
    pub use crate::combat_system::*;
    pub use crate::map::*;
    pub use crate::map_indexing_system::*;
    pub use crate::monster_ai_system::*;
    pub use crate::player::*;
    pub use crate::rect::*;
    pub use crate::TurnState;
    pub use crate::visibility_system::*;

    pub use bevy::prelude::*;
    pub use bracket_lib::prelude::*;
    pub use std::cmp::{max, min};
}

pub use prelude::*;
use iyes_loopless::prelude::*;

//Consts
const GLYPH_SIZE: f32 = 8.;

//States
#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    MovePlayer,
    MoveMonsters,
    MonsterCollisions,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    MonsterTurn,
}

//Resources
#[derive(Clone)]
pub struct Glyphs {
    pub handle: Handle<TextureAtlas>,
}

//Systems
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: (MAP_WIDTH * 10) as f32,
            height: (MAP_HEIGHT * 10) as f32,
            ..default()
        })
        .insert_resource(Map::new_map_rooms_and_corridors())
        .add_event::<SufferDamage>()
        .add_startup_system(initial_setup)
        .insert_resource(TurnState::AwaitingInput)
        .add_stage_after(CoreStage::Update, GameStage::MovePlayer, SystemStage::parallel())
        .add_stage_after(GameStage::MovePlayer, GameStage::MoveMonsters, SystemStage::parallel())
        .add_stage_after(GameStage::MoveMonsters, GameStage::MonsterCollisions, SystemStage::parallel())
        .add_system_set(SystemSet::new().with_system(draw_map).with_system(renderables).with_system(size_scaling).with_system(position_translation.after(size_scaling)))
        .add_system(keyboard_input.run_if_resource_equals(TurnState::AwaitingInput))
        .add_system_set_to_stage(GameStage::MovePlayer, ConditionSet::new().run_if_resource_equals(TurnState::PlayerTurn).with_system(try_move).into())
        .add_system_set_to_stage(GameStage::MoveMonsters, ConditionSet::new().run_if_resource_equals(TurnState::MonsterTurn).with_system(monster_ai_system).into())
        .add_system_set_to_stage(GameStage::MonsterCollisions, ConditionSet::new().with_system(visibility_system).with_system(melee_combat_system).with_system(indexing_system).with_system(damage_system).with_system(bring_out_your_dead).into())
        .run();
}

fn initial_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    map: Res<Map>,
) {
    //Setting up the spritesheet
    let texture_handle = asset_server.load("glyphs.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(GLYPH_SIZE, GLYPH_SIZE), 16, 16);
    let glyphs = Glyphs {
        handle: texture_atlases.add(texture_atlas),
    };
    commands.insert_resource(glyphs.clone());

    //Adding camera
    commands.spawn_bundle(Camera2dBundle::default());

    //Spawning player
    let (x, y) = map.rooms[0].center();
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                color: Color::YELLOW,
                index: 64,
                ..default()
            },
            texture_atlas: glyphs.handle.clone(),
            ..default()
        })
        .insert(Player {})
        .insert(Position { x, y, z: 1 })
        .insert(Renderable {
            glyph: 64,
            color: Color::YELLOW,
        })
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .insert(Name {name: "Player".to_string()})
        .insert(BlocksTile {})
        .insert(CombatStats {max_hp: 30, hp: 30, defense: 2, power: 5});

    //Spawning some monsters
    let mut rng = RandomNumberGenerator::new();
    let mut i = 1;
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        let roll = rng.roll_dice(1, 2);
        let (glyph, name) = match roll {
            1 => (176, "Goblin".to_string()),
            _ => (182, "Orc".to_string()),
        };

        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::RED,
                    index: glyph,
                    ..default()
                },
                texture_atlas: glyphs.handle.clone(),
                ..default()
            })
            .insert(Position { x, y, z: 1 })
            .insert(Renderable {
                glyph: glyph,
                color: Color::RED,
            })
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .insert(Monster {})
            .insert(Name {name: format!("{} #{}", name, i)})
            .insert(BlocksTile {})
            .insert(CombatStats {max_hp: 16, hp: 16, defense: 1, power: 4});
            i += 1;
    }
}

//Resizes to support any resolution.
fn size_scaling(
    windows: Res<Windows>,
    mut q: Query<(&TileSize, &mut Transform), Or<(Changed<TileSize>, Changed<Transform>)>>,
) {
    if windows.is_changed() {
        let window = windows.get_primary().unwrap();
        for (sprite_size, mut transform) in q.iter_mut() {
            let scale = Vec3::new(
                sprite_size.size / MAP_WIDTH as f32 * window.width() as f32 / GLYPH_SIZE,
                sprite_size.size / MAP_HEIGHT as f32 * window.height() as f32 / GLYPH_SIZE,
                1.,
            );
            transform.scale = scale;
        }
    }
}

//Helper function for position_translation() system
fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

//Converts map coordinates to bevy window coordinates.
fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&Position, &mut Transform), Or<(Changed<Position>, Changed<Transform>)>>,
) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert_pos(pos.x as f32, window.width() as f32, MAP_WIDTH as f32),
            convert_pos(pos.y as f32, window.height() as f32, MAP_HEIGHT as f32),
            pos.z as f32,
        );
    }
}

//Draws entities on top of the map.
fn renderables(
    mut renderables: Query<
        (
            &Renderable,
            &mut TextureAtlasSprite,
            &Position,
            &mut Visibility,
        ),
        Without<MapTile>,
    >,
    map: Res<Map>,
) {
    for (render, mut sprite, pos, mut vis) in renderables.iter_mut() {
        let idx = map.xy_idx(pos.x, pos.y);
        if map.visible_tiles[idx] {
            *vis = Visibility { is_visible: true };
            sprite.index = render.glyph;
            sprite.color = render.color;
        } else {
            *vis = Visibility { is_visible: false };
        }
    }
}
