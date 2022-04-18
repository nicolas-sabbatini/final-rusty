use crate::{
    common_component::{Collider, Speed},
    AppState, SpriteSheet, TILE_SIZE,
};
use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_inspector_egui::Inspectable;

// Plugin struct definitions
#[derive(Debug, Component, Inspectable)]
pub struct Player;

#[derive(Bundle)]
struct PlayerBundle {
    name: Name,
    tag: Player,
    speed: Speed,
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
                    .with_system(camera_follow),
            );
    }
}

fn spawn_player(mut commands: Commands, sprite_sheet: Res<SpriteSheet>) {
    //    let mut sprite = ;
    //    sprite.custom_size = Some(Vec2::new(50.0, 50.0));

    commands.spawn_bundle(PlayerBundle {
        tag: Player,
        name: Name::new("Player"),
        speed: Speed(32.0),
        sprite: SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(1),
            texture_atlas: sprite_sheet.0.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 10.0),
            ..Default::default()
        },
    });
}

fn move_player(
    mut player_query: Query<(&mut Transform, &Speed), With<Player>>,
    collition_query: Query<&Transform, (With<Collider>, Without<Player>)>,
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

    if check_colition(player_transform.translation + vel_x, &collition_query) {
        player_transform.translation += vel_x;
    }
    if check_colition(player_transform.translation + vel_y, &collition_query) {
        player_transform.translation += vel_y;
    }
}

fn check_colition(
    player_position: Vec3,
    collition_query: &Query<&Transform, (With<Collider>, Without<Player>)>,
) -> bool {
    for collider_position in collition_query.iter() {
        let collition = collide(
            player_position,
            Vec2::splat(TILE_SIZE * 0.7),
            collider_position.translation,
            Vec2::splat(TILE_SIZE),
        );
        if collition.is_some() {
            return false;
        }
    }
    true
}

fn camera_follow(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
    let player_transform = player_query
        .get_single()
        .expect("No player found 'PlayerPlugin (camera_follow 108)'");
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("No camera found 'PlayerPlugin (camera_follow 108)'");
    camera_transform.translation = player_transform.translation;
}
