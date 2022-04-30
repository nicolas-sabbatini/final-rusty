use bevy::prelude::*;

use crate::{AppState, WIN_HEIGHT, WIN_WIDTH};

// Plugin struct definitions
#[derive(Debug, Component)]
struct FadeoutStatus {
    alpha: f32,
    sent: bool,
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

pub struct FadeoutPlugin;
impl Plugin for FadeoutPlugin {
    fn build(&self, app: &mut App) {}
}

pub fn create_fadeout(commands: &mut Commands, next_state: AppState, fadeout_duration: f32) {
    commands.spawn_bundle(FadeoutBundle {
        tag: Name::new("Fadeout"),
        status: FadeoutStatus {
            alpha: 1.0,
            sent: false,
            next_state,
            timer: Timer::from_seconds(fadeout_duration, false),
        },
        sprite: SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.0, 0.0, 0.0, 1.0),
                custom_size: Some(Vec2::new(WIN_WIDTH, WIN_HEIGHT)),
                ..Default::default()
            },
            transform: Transform::from_xyz(0.0, 0.0, 999.9),
            ..Default::default()
        },
    });
}
