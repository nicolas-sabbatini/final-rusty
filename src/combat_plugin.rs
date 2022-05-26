use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use rand::Rng;

use crate::{common_component::CombatStats, player_plugin::Player, AppState, SpriteSheet};

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
    pub emitter: Entity,
}

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CombatEvent>();

        app.add_system_set(SystemSet::on_enter(AppState::Combat).with_system(spawn_enemy))
            .add_system_set(
                SystemSet::on_update(AppState::Combat)
                    .with_system(force_end_combat)
                    .with_system(combat_input)
                    .with_system(process_combat.after(combat_input))
                    .with_system(end_combat.after(process_combat)),
            )
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

fn combat_input(
    keyboard: Res<Input<KeyCode>>,
    mut combat_event: EventWriter<CombatEvent>,
    enemy_query: Query<Entity, With<Enemy>>,
    player_query: Query<Entity, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        // TODO Handle multiple enemys
        let target = enemy_query
            .get_single()
            .expect("Can not get a target entity");
        // TODO Handle multiple player entities
        let emitter = player_query
            .get_single()
            .expect("Can not get Player entity");
        combat_event.send(CombatEvent { target, emitter });
        println!("Combat");
    }
}

fn process_combat(
    mut combat_event: EventReader<CombatEvent>,
    mut combat_stats_query: Query<&mut CombatStats>,
) {
    for event in combat_event.iter() {
        let [emitter, mut target] = combat_stats_query
            .get_many_mut([event.emitter, event.target])
            .expect("Can not get any CombatStats");
        println!("{emitter:#?}, {target:#?}");
        target.hp -= i32::max(emitter.attack - target.defense, 0);
    }
}

fn end_combat(
    mut state: ResMut<State<AppState>>,
    enemy_stats_query: Query<&CombatStats, With<Enemy>>,
) {
    // TODO Handle multiple enemys and player losing
    for combat_stats in enemy_stats_query.iter() {
        if combat_stats.hp <= 0 {
            state.pop().expect("Error poping Combat state");
        }
    }
}

fn force_end_combat(mut keyboard: ResMut<Input<KeyCode>>, mut state: ResMut<State<AppState>>) {
    if keyboard.just_pressed(KeyCode::Q) {
        keyboard.reset(KeyCode::Q);
        state.pop().expect("Error poping Combat state");
    }
}
