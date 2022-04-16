use bevy::prelude::*;

use crate::{common_component::Collider, SpriteSheet, TILE_SIZE};

#[derive(Bundle)]
struct MapBundle {
    name: Name,
    transform: Transform,
    g_transform: GlobalTransform,
}

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(creat_simple_map);
    }
}

fn creat_simple_map(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    // TODO:  Change to a custom load
    let map = include_str!("../assets/map.txt");

    let mut tiles = Vec::new();
    for (y, line) in map.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let (i, c) = match ch {
                '#' => (6, true),
                '%' => (8, true),
                'g' => (7, false),
                ',' => (9, false),
                '~' => (10, false),
                _ => (3, false),
            };
            let tile = commands
                .spawn_bundle(SpriteSheetBundle {
                    sprite: TextureAtlasSprite::new(i),
                    texture_atlas: sprite_sheet.0.clone(),
                    transform: Transform::from_xyz(
                        (x as f32 - 1.0) * TILE_SIZE,
                        (1.0 - (y as f32)) * TILE_SIZE,
                        9.0,
                    ),
                    ..Default::default()
                })
                .id();
            if c {
                commands.entity(tile).insert(Collider);
            }
            tiles.push(tile);
        }
    }
    commands
        .spawn_bundle(MapBundle {
            name: Name::new("map.txt"),
            transform: Transform::default(),
            g_transform: GlobalTransform::default(),
        })
        .push_children(&tiles);
}
