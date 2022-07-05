use bevy::prelude::*;
use crate::components::{Position, TileSize, BgTile, FgTile};

pub const TERM_WIDTH: usize = 80;
pub const TERM_HEIGHT: usize = 50;
pub const GLYPH_SIZE: f32 = 8.;

pub struct BevyTermPlugin;
impl Plugin for BevyTermPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system(build_term)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
                .with_system(size_scaling),
        );
    }   
}

pub struct Terminal {
    pub bg_tiles: Vec<Entity>,
    pub fg_tiles: Vec<Entity>,
    pub term_width: usize,
    pub term_height: usize
}

#[derive(Clone)]
pub struct Glyphs {
    handle: Handle<TextureAtlas>
}

//Setups the terminal bg & fg sprites, loads glyphs.  
fn build_term(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    //Setting up the spritesheet
    let texture_handle = asset_server.load("glyphs.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(GLYPH_SIZE, GLYPH_SIZE), 16, 16);
    let glyphs = Glyphs {handle: texture_atlases.add(texture_atlas)};
    commands.insert_resource(glyphs.clone());
    
    //Adding camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    //Spawning the tiles. 
    let mut bg_tiles: Vec<Entity> = Vec::new();
    let mut fg_tiles: Vec<Entity> = Vec::new();
    let term_width = TERM_WIDTH;
    let term_height = TERM_HEIGHT;
    for y in 0..term_height {
        for x in 0..term_width {
            let bg = commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::BLACK,
                    index: 32,
                    ..default()
                },
                texture_atlas: glyphs.handle.clone(),
                ..default()
            })
            .insert(Position{x, y, z: 0})
            .insert(TileSize::square(1.))
            .insert(BgTile {})
            .id();

            bg_tiles.push(bg);

            let fg = commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: Color::LIME_GREEN,
                    index: 0,
                    ..default()
                },
                texture_atlas: glyphs.handle.clone(),
                ..default()
            })
            .insert(Position{x, y, z: 1})
            .insert(TileSize::square(1.))
            .insert(FgTile{})
            .id();

            fg_tiles.push(fg);
        }
    }

    //Insert the terminal as a global resource
    commands.insert_resource(Terminal {bg_tiles, fg_tiles, term_width, term_height})
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&TileSize, &mut Transform), Or<(Changed<TileSize>, Changed<Transform>)>>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut transform) in q.iter_mut() {
        let scale = Vec3::new(
            sprite_size.width / TERM_WIDTH as f32 * window.width() as f32 / GLYPH_SIZE,
            sprite_size.height / TERM_HEIGHT as f32 * window.height() as f32 / GLYPH_SIZE,
            1.,
        );
        transform.scale = scale;
    }
}

fn convert_pos(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
    let tile_size = bound_window / bound_game;
    pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform), Or<(Changed<Position>, Changed<Transform>)>>) {
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert_pos(pos.x as f32, window.width() as f32, TERM_WIDTH as f32),
            convert_pos(pos.y as f32, window.height() as f32, TERM_HEIGHT as f32),
            pos.z as f32,
        );
    }
}

pub fn pos_index(x: usize, y: usize) -> usize {
    (y * TERM_WIDTH) + x
}

//Not sure if this function will ever be used, but it's good to have if needed.  
pub fn _index_pos(index: usize) -> (usize, usize) {
    (index % TERM_WIDTH, index / TERM_WIDTH)
}