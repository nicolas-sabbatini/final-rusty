use bevy::{prelude::*, window::PresentMode};
use player_plugin::PlayerPlugin;

#[cfg(debug_assertions)]
use debug_plugin::DebugPlugin;
use tilemap_plugin::TilemapPlugin;
#[cfg(debug_assertions)]
mod debug_plugin;

mod common_component;
mod player_plugin;
mod tilemap_plugin;

struct SpriteSheet(Handle<TextureAtlas>);

const WIN_WIDTH: f32 = 200.0;
const WIN_HEIGHT: f32 = 150.0;
const WIN_SCALE: f32 = 4.0;
const TILE_SIZE: f32 = 8.0;

fn main() {
    let mut app = App::new();

    app.insert_resource(Msaa { samples: 1 })
        .insert_resource(WindowDescriptor {
            title: "RustyPoke!".to_string(),
            width: WIN_WIDTH * WIN_SCALE,
            height: WIN_HEIGHT * WIN_SCALE,
            resizable: true,
            present_mode: PresentMode::Fifo,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.5, 0.5, 0.5)));

    app.add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilemapPlugin);

    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin);

    app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);

    app.add_startup_system(setup_camera);

    app.run();
}

fn setup_camera(mut commands: Commands, win_res: Res<Windows>) {
    let win = win_res.get_primary().unwrap();
    let mut new_camera = OrthographicCameraBundle::new_2d();
    new_camera.orthographic_projection.scaling_mode =
        bevy::render::camera::ScalingMode::FixedVertical;
    new_camera.orthographic_projection.scale = win.height() / (WIN_SCALE * 2.0);
    commands.spawn_bundle(new_camera);
}

fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img = assets.load("spritesheet.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(img, Vec2::splat(TILE_SIZE), 11, 1, Vec2::splat(1.0));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(SpriteSheet(atlas_handle));
}
