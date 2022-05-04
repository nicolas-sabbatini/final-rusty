use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::Rng;

use crate::{common_component::CombatStats, AppState, SpriteSheet};

// TODO move structs to a create enemy plugin
// Plugin struct definitions
#[derive(Debug, Component, Inspectable)]
pub struct Enemy;

#[derive(Bundle)]
struct EnemyBundle {
    name: Name,
    tag: Enemy,
    combat_stats: CombatStats,
    #[bundle]
    sprite: SpriteSheetBundle,
}
// end TODO

pub struct CombatEvent {
    pub target: Entity,
    pub damage: i32,
}

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatEvent>();

        app.add_system_set(SystemSet::on_enter(AppState::Combat).with_system(spawn_enemy))
            .add_system_set(SystemSet::on_update(AppState::Combat).with_system(combat_input))
            .add_system_set(SystemSet::on_exit(AppState::Combat).with_system(despawn_enemy));
    }
}

fn spawn_enemy(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    // TODO change to a file
    let enemy_type = [("Rat", 16), ("Snake", 17), ("Wolf", 18)];

    let selected_enemy = rand::thread_rng().gen_range(0..enemy_type.len());
    commands.spawn_bundle(EnemyBundle {
        tag: Enemy,
        name: Name::new(enemy_type[selected_enemy].0),
        // TODO diferent enemys diferent stats
        combat_stats: CombatStats {
            hp: 5,
            max_hp: 5,
            attack: 1,
            defense: 1,
        },
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

fn combat_input(mut keyboar: ResMut<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if keyboar.just_pressed(KeyCode::Space) {
        keyboar.reset(KeyCode::Space);
        state.pop().expect("Error poping state 'Combat plugin' 72");
    }
}
