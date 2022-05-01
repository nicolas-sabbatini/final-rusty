use crate::{
    common_component::{Collider, EncounterSpawn, Speed},
    fadeout_plugin::FadeoutConfigResource,
    AppState, SpriteSheet, TILE_SIZE,
};
use bevy::{prelude::*, render::camera::Camera2d, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;
use rand::Rng;

// Plugin struct definitions
#[derive(Debug, Component, Inspectable)]
pub struct Player;

#[derive(Debug, Component, Inspectable)]
pub struct CombatTimer {
    remaining: f32,
    max_range: f32,
    min_range: f32,
}
impl CombatTimer {
    fn new(min_range: f32, max_range: f32) -> Self {
        Self {
            max_range,
            min_range,
            remaining: rand::thread_rng().gen_range(min_range..max_range),
        }
    }

    fn tick(&mut self, tick: f32) {
        self.remaining -= tick;
    }

    fn is_done(&self) -> bool {
        self.remaining < 0.0
    }

    fn reset(&mut self) {
        self.remaining = rand::thread_rng().gen_range(self.min_range..self.max_range);
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    tag: Player,
    speed: Speed,
    until_combat: CombatTimer,
    #[bundle]
    sprite: SpriteSheetBundle,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::OverWorld).with_system(spawn_player))
            .add_system_set(
                SystemSet::on_update(AppState::OverWorld)
                    .with_system(move_player)
                    .with_system(camera_follow.after(move_player))
                    .with_system(check_encunter.after(move_player)),
            )
            // On combat enter
            .add_system_set(SystemSet::on_enter(AppState::Combat).with_system(hide_player))
            // Always that the Overworld start show the player
            .add_system_set(SystemSet::on_resume(AppState::OverWorld).with_system(show_player));
    }
}

fn spawn_player(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    commands.spawn_bundle(PlayerBundle {
        tag: Player,
        name: Name::new("Player"),
        speed: Speed(32.0),
        until_combat: CombatTimer::new(20.0, 50.0),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(8),
            texture_atlas: sprite_sheet.0.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        },
    });
}

fn move_player(
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    mut encounter_query: Query<&mut CombatTimer, With<Player>>,
    collition_query: Query<&Transform, (With<Collider>, Without<Player>)>,
    encounter_collition_query: Query<&Transform, (With<EncounterSpawn>, Without<Player>)>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let (mut player_transform, speed) = player_query
        .get_single_mut()
        .expect("No player found 'PlayerPlugin (move_player 55)'");

    let mut vel = Vec3::new(0.0, 0.0, 0.0);
    if keyboard.pressed(KeyCode::Up) {
        vel.y += 1.0
    }
    if keyboard.pressed(KeyCode::Down) {
        vel.y -= 1.0
    }
    if keyboard.pressed(KeyCode::Left) {
        vel.x -= 1.0
    }
    if keyboard.pressed(KeyCode::Right) {
        vel.x += 1.0
    }
    vel = vel.normalize_or_zero() * speed.0 * time.delta_seconds();

    let mut vel_x = vel;
    vel_x.y = 0.0;
    let mut vel_y = vel;
    vel_y.x = 0.0;

    if !collition_query.is_empty()
        && !collition_query
            .iter()
            .any(|w| check_colition(player_transform.translation + vel_x, w.translation))
    {
        player_transform.translation += vel_x;
    }
    if !collition_query.is_empty()
        && !collition_query
            .iter()
            .any(|w| check_colition(player_transform.translation + vel_y, w.translation))
    {
        player_transform.translation += vel_y;
    }
    if !encounter_collition_query.is_empty()
        && encounter_collition_query
            .iter()
            .any(|w| check_colition(player_transform.translation, w.translation))
    {
        let mut encounter_timer = encounter_query
            .get_single_mut()
            .expect("No encounter timer found 'PlayerPlugin (move_player 127)'");
        encounter_timer.tick(vel.x.abs() + vel.y.abs());
    }
}

fn check_colition(player_position: Vec3, collition_target: Vec3) -> bool {
    let collition = collide(
        player_position,
        Vec2::splat(TILE_SIZE * 0.6),
        collition_target,
        Vec2::splat(TILE_SIZE),
    );
    collition.is_some()
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("No player found 'PlayerPlugin (camera_follow 108)'");
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("No camera found 'PlayerPlugin (camera_follow 108)'");
    camera_transform.translation.x = player_transform.translation.x;
    camera_transform.translation.y = player_transform.translation.y;
}

/// Look for trouble
fn check_encunter(
    mut commands: Commands,
    mut encounter_query: Query<(&mut CombatTimer, &Transform), With<Player>>,
    mut state: ResMut<State<AppState>>,
) {
    let (mut encounter_timer, player_transform) = encounter_query
        .get_single_mut()
        .expect("No encounter timer found 'Player plugin' 163");
    if encounter_timer.is_done() {
        encounter_timer.reset();
        commands.insert_resource(FadeoutConfigResource {
            fadeout_duration: 0.5,
            next_state: AppState::Combat,
            position: player_transform.translation,
        });
        state
            .push(AppState::Fadeout)
            .expect("Error pushing state to App::Fadeout 'Player plugin' 184");
    }
}

fn hide_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = false;

    for children in children_query.iter() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = false;
            }
        }
    }
}

fn show_player(
    mut player_query: Query<&mut Visibility, With<Player>>,
    children_query: Query<&Children, With<Player>>,
    mut child_visibility_query: Query<&mut Visibility, Without<Player>>,
) {
    let mut player_vis = player_query.single_mut();
    player_vis.is_visible = true;

    for children in children_query.iter() {
        for child in children.iter() {
            if let Ok(mut child_vis) = child_visibility_query.get_mut(*child) {
                child_vis.is_visible = true;
            }
        }
    }
}
