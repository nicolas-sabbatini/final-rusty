use bevy::{prelude::*, render::camera::Camera2d};

use crate::{AppState, WIN_HEIGHT, WIN_SCALE, WIN_WIDTH};

// Plugin struct definitions
#[derive(Debug, Component)]
pub struct LetterBox(f32);

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_camera)
            .add_startup_system(setup_letterboxing)
            .add_system_to_stage(CoreStage::PostUpdate, update_letterboxing);

        app.add_system_set(
            SystemSet::on_enter(AppState::OverWorld).with_system(set_overworld_camera),
        )
        .add_system_set(SystemSet::on_resume(AppState::OverWorld).with_system(set_overworld_camera))
        .add_system_set(SystemSet::on_enter(AppState::Combat).with_system(set_combat_camera))
        .add_system_set(SystemSet::on_resume(AppState::Combat).with_system(set_combat_camera));
    }
}

fn setup_camera(mut commands: Commands, win_res: Res<Windows>) {
    let win = win_res.get_primary().unwrap();
    let mut new_camera = OrthographicCameraBundle::new_2d();
    new_camera.orthographic_projection.scaling_mode =
        bevy::render::camera::ScalingMode::FixedVertical;
    new_camera.orthographic_projection.scale = win.height() / (WIN_SCALE * 2.0);
    commands.spawn_bundle(new_camera);
}

fn setup_letterboxing(mut commands: Commands) {
    let mut spawn_letterboxing = |x_mul: f32, name: String| {
        commands
            .spawn_bundle(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.1, 0.1, 0.1),
                    custom_size: Some(Vec2::new(WIN_WIDTH, WIN_HEIGHT)),
                    ..Default::default()
                },
                transform: Transform::from_xyz(WIN_WIDTH * x_mul, 0.0, 99.0),
                ..Default::default()
            })
            .insert(Name::new(name))
            .insert(LetterBox(x_mul));
    };
    spawn_letterboxing(1.0, String::from("Left"));
    spawn_letterboxing(-1.0, String::from("Right"));
}

fn update_letterboxing(
    mut letterbox_query: Query<(&mut Transform, &LetterBox), Without<Camera2d>>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    let camera_transform = camera_query
        .get_single()
        .expect("No camera found 'CameraPlugin 59'");
    for (mut letterbox, x_mul) in letterbox_query.iter_mut() {
        letterbox.translation = camera_transform.translation;
        let x_offset_mul = if camera_transform.scale.x == 1.0 {
            0.0
        } else {
            0.25
        };
        letterbox.translation.x += (WIN_WIDTH - (x_offset_mul * WIN_WIDTH)) * x_mul.0;
    }
}

fn set_combat_camera(mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("No camera found 'CameraPlugin 27'");
    camera_transform.translation = Vec3::new(0.0, 0.0, 999.9);
    camera_transform.scale = Vec3::new(0.5, 0.5, 1.0);
}

fn set_overworld_camera(mut camera_query: Query<&mut Transform, With<Camera2d>>) {
    let mut camera_transform = camera_query
        .get_single_mut()
        .expect("No camera found 'CameraPlugin 35'");
    camera_transform.translation = Vec3::new(0.0, 0.0, 999.9);
    camera_transform.scale = Vec3::new(1.0, 1.0, 1.0);
}
