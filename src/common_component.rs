use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;

#[derive(Debug, Component)]
pub struct Speed(pub f32);

#[derive(Debug, Component)]
pub struct Collider;

#[derive(Debug, Component)]
pub struct EncounterSpawn;

#[derive(Debug, Component, Inspectable)]
pub struct CombatStats {
    pub hp: i32,
    pub max_hp: i32,
    pub attack: i32,
    pub defense: i32,
}
