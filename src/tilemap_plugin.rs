use bevy::prelude::*;

use crate::{
    common_component::{Collider, EncounterSpawn},
    AppState, SpriteSheet, TILE_SIZE,
};

#[derive(Bundle)]
struct MapBundle {
    name: Name,
    transform: Transform,
    g_transform: GlobalTransform,
}

pub struct TilemapPlugin;
impl Plugin for TilemapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::OverWorld).with_system(creat_simple_map));
    }
}

fn creat_simple_map(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    // TODO:  Change to a custom load
    let map = include_str!("../assets/map.txt");

    let mut spawn_tile = |i: usize, x: usize, y: usize, collider: bool, spawn: bool| {
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
        if collider {
            commands.entity(tile).insert(Collider);
        }
        if spawn {
            commands.entity(tile).insert(EncounterSpawn);
        }
        tile
    };

    let mut tiles = Vec::new();
    for (y, line) in map.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            let tile = match ch {
                '#' => spawn_tile(6, x, y, true, false),
                '%' => spawn_tile(8, x, y, true, false),
                'g' => spawn_tile(7, x, y, false, false),
                ',' => spawn_tile(9, x, y, false, false),
                '~' => spawn_tile(10, x, y, false, false),
                _ => spawn_tile(3, x, y, false, false),
            };
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
