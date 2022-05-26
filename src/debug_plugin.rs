use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_inspector_egui::{RegisterInspectable, WorldInspectorPlugin};

use crate::combat_plugin::Enemy;
use crate::common_component::CombatStats;
use crate::player_plugin::{CombatTimer, Player};

pub struct DebugPlugin;
impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(WorldInspectorPlugin::new())
            .add_plugin(LogDiagnosticsPlugin::default())
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .register_inspectable::<Player>()
            .register_inspectable::<CombatTimer>()
            .register_inspectable::<CombatStats>()
            .register_inspectable::<Enemy>();
    }
}
