use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::Rng;

use crate::{AppState, SpriteSheet};

// Plugin struct definitions
#[derive(Debug, Component, Inspectable)]
pub struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    tag: Enemy,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Combat).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_pause(AppState::Combat))
            .add_system_set(SystemSet::on_resume(AppState::Combat))
            .add_system_set(SystemSet::on_exit(AppState::Combat).with_system(despawn_enemy));
    }
}

fn spawn_enemy(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    // TODO change to a file
    let enemy_type = [("Rat", 11), ("Snake", 12), ("Wolf", 13)];

    let selected_enemy = rand::thread_rng().gen_range(0..enemy_type.len());
    commands.spawn_bundle(EnemyBundle {
        tag: Enemy,
        name: Name::new(enemy_type[selected_enemy].0),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(enemy_type[selected_enemy].1),
            texture_atlas: sprite_sheet.0.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        },
    });
}

fn despawn_enemy(mut commands: Commands, enemy_query: Query<Entity, With<Enemy>>) {
    for ent in enemy_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
