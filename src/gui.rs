use crate::prelude::*;

//Events
pub struct UiBox {
    pub fg: Color,
    pub bg: Color,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

pub struct ClearUi;

//Components
#[derive(Component)]
pub struct UiTile;

//Systems
pub fn draw_ui(mut event: EventWriter<UiBox>) {
    event.send(UiBox {fg: Color::WHITE, bg: Color::BLACK, x: 0, y: 43, w: 79, h: 6});
    //Left off adding HP Bar.
}

pub fn clear_ui(mut commands: Commands, query: Query<Entity, With<UiTile>>, mut evt: EventReader<ClearUi>) {
    for _event in evt.iter() {
        for ent in query.iter() {
            commands.entity(ent).despawn();
        }
    }
}

pub fn draw_ui_box(mut ui_box: EventReader<UiBox>, mut commands: Commands, glyphs: Res<Glyphs>) {
    for bx in ui_box.iter() {
        for fx in bx.x..=bx.w {
            //Top
            let glyph = if fx == bx.x {130} else if fx == bx.w + bx.x {131} else {128};
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: bx.fg,
                    index: glyph,
                    ..default()
                },
                texture_atlas: glyphs.handle.clone(),
                ..default()
            }).insert(TileSize{size: 1.}).insert(Position {x: fx, y: bx.y, z: 2}).insert(Renderable{glyph, color: bx.fg}).insert(UiTile {});

            //Bottom
            let glyph = if fx == bx.x {132} else if fx == bx.w + bx.x {133} else {128};
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: bx.fg,
                    index: glyph,
                    ..default()
                },
                texture_atlas: glyphs.handle.clone(),
                ..default()
            }).insert(TileSize{size: 1.}).insert(Position {x: fx, y: bx.y + bx.h, z: 2}).insert(Renderable{glyph, color: bx.fg}).insert(UiTile {});
        }

        for fy in bx.y..=bx.h + bx.y {
            //Left
            if fy == bx.y || fy == bx.y + bx.h {
                //Skip first and last iterations.  
                continue
            }
            let glyph = 129;
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: bx.fg,
                    index: glyph,
                    ..default()
                },
                texture_atlas: glyphs.handle.clone(),
                ..default()
            }).insert(TileSize{size: 1.}).insert(Position {x: bx.x, y: fy, z: 2}).insert(Renderable{glyph, color: bx.fg}).insert(UiTile {});

            //Right
            commands.spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    color: bx.fg,
                    index: glyph,
                    ..default()
                },
                texture_atlas: glyphs.handle.clone(),
                ..default()
            }).insert(TileSize{size: 1.}).insert(Position {x: bx.x + bx.w, y: fy, z: 2}).insert(Renderable{glyph, color: bx.fg}).insert(UiTile {});
        }
    }
}