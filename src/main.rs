use bevy::{prelude::*, window::PresentMode};
use camera_plugin::CameraPlugin;
use combat_plugin::CombatPlugin;
use player_plugin::PlayerPlugin;
use tilemap_plugin::TilemapPlugin;

// Load and use this module on debug
#[cfg(debug_assertions)]
use debug_plugin::DebugPlugin;
#[cfg(debug_assertions)]
mod debug_plugin;

mod camera_plugin;
mod combat_plugin;
mod common_component;
mod player_plugin;
mod tilemap_plugin;

struct SpriteSheet(Handle<TextureAtlas>);

const WIN_WIDTH: f32 = 200.0;
const WIN_HEIGHT: f32 = 150.0;
const WIN_SCALE: f32 = 4.0;
const TILE_SIZE: f32 = 8.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    OverWorld,
    Combat,
}

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
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));

    app.add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(TilemapPlugin)
        .add_plugin(CombatPlugin)
        .add_plugin(CameraPlugin);

    // Add this plugins and system on debug
    #[cfg(debug_assertions)]
    app.add_plugin(DebugPlugin)
        .add_system(bevy::input::system::exit_on_esc_system);

    app.add_startup_system_to_stage(StartupStage::PreStartup, load_assets);

    app.add_state(AppState::OverWorld);

    app.run();
}

fn load_assets(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let img = assets.load("spritesheet.png");
    let atlas =
        TextureAtlas::from_grid_with_padding(img, Vec2::splat(TILE_SIZE), 11, 2, Vec2::splat(1.0));

    let atlas_handle = texture_atlases.add(atlas);

    commands.insert_resource(SpriteSheet(atlas_handle));
}
