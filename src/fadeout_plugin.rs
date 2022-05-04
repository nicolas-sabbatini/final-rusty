use bevy::prelude::*;

use crate::{AppState, WIN_HEIGHT, WIN_WIDTH};

// Plugin struct definitions
#[derive(Debug, Component)]
struct FadeoutStatus {
    next_state: AppState,
    timer: Timer,
}

#[derive(Bundle)]
struct FadeoutBundle {
    tag: Name,
    status: FadeoutStatus,
    #[bundle]
    sprite: SpriteBundle,
}

#[derive(Debug, Clone, Copy)]
pub struct FadeoutConfigResource {
    pub fadeout_duration: f32,
    pub next_state: AppState,
    pub position: Vec3,
}

pub struct FadeoutPlugin;
impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(AppState::Fadeout).with_system(create_fadeout))
            .add_system_set(SystemSet::on_update(AppState::Fadeout).with_system(update_fadeout));
    }
}

// TODO: Change fadeout_config to be an option
fn create_fadeout(mut commands: Commands, fadeout_config: ResMut<FadeoutConfigResource>) {
    commands.spawn_bundle(FadeoutBundle {
        tag: Name::new("Fadeout"),
        status: FadeoutStatus {
            next_state: fadeout_config.next_state,
            timer: Timer::from_seconds(fadeout_config.fadeout_duration, false),
        },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 0.0),
                custom_size: Some(Vec2::new(WIN_WIDTH, WIN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(
                fadeout_config.position.x,
                fadeout_config.position.y,
                999.9,
            ),
            ..Default::default()
        },
    });
    commands.remove_resource::<FadeoutConfigResource>();
}

fn update_fadeout(
    mut commands: Commands,
    mut fadeout_query: Query<(Entity, &mut FadeoutStatus, &mut Sprite)>,
    mut state: ResMut<State<AppState>>,
    time: Res<Time>,
) {
    for (entity, mut status, mut sprite) in fadeout_query.iter_mut() {
        status.timer.tick(time.delta());
        sprite.color.set_a(status.timer.percent() + 0.3);
        if status.timer.finished() {
            commands.entity(entity).despawn_recursive();
            state.set(status.next_state).unwrap();
        }
    }
}
