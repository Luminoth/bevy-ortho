use bevy::prelude::*;

#[derive(Debug, Component)]
#[require(Transform)]
pub struct PlayerSpawn;

#[derive(Debug, Component)]
#[require(Transform)]
pub struct GroundLootSpawn;
