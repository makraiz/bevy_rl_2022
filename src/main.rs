use bevy::prelude::*;

const PLAYER_COLOR: Color = Color::rgb(0., 1., 0.);
const TERM_WIDTH: usize = 80;
const TERM_HEIGHT: usize = 50;

#[derive(Component)]
struct Player;

#[derive(Component, Clone, Copy, PartialEq, Eq)]
struct Position {
    x: usize,
    y: usize
}

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
        .add_system(movement)
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(position_translation)
        )
        .run();
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, TERM_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, TERM_HEIGHT as f32),
            0.
        );
    }
}

fn movement(keys: Res<Input<KeyCode>>, mut positions: Query<&mut Position, With<Player>>) {
    for mut pos in positions.iter_mut() {
        if keys.just_pressed(KeyCode::Left) {
            pos.x -= 1;
        }
        if keys.just_pressed(KeyCode::Right) {
            pos.x += 1;
        }
        if keys.just_pressed(KeyCode::Down) {
            pos.y -= 1;
        }
        if keys.just_pressed(KeyCode::Up) {
            pos.y += 1;
        }
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
    let texture_handle = asset_server.load("glyphs.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8., 8.), 16, 16);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        sprite: TextureAtlasSprite {
            color: PLAYER_COLOR,
            index: 64,
            ..default()
        },
        ..default()
    })
    .insert(Player)
    .insert(Position {x: TERM_WIDTH / 2, y: TERM_HEIGHT / 2});
}