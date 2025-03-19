use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};

use crate::{GameAssets, GameCollisionLayers, WORLD_INTERACT_LAYERS, spawn};

pub const FLOOR_X_LENGTH: f32 = 30.0;
pub const FLOOR_Z_LENGTH: f32 = 30.0;

pub const BOX_X_LENGTH: f32 = 1.0;
pub const BOX_Y_LENGTH: f32 = 1.0;
pub const BOX_Z_LENGTH: f32 = 1.0;

pub const CRATE_X_LENGTH: f32 = 2.0;
pub const CRATE_Y_LENGTH: f32 = 1.0;
pub const CRATE_Z_LENGTH: f32 = 1.0;

#[derive(Debug)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {}
}

fn spawn_floor(commands: &mut Commands, game_assets: &GameAssets, rotation: Quat) {
    let mut commands = commands.spawn((
        game_assets.gen_floor_mesh_components(),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(rotation),
        Name::new("Floor"),
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(FLOOR_X_LENGTH, 0.1, FLOOR_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

fn spawn_box(commands: &mut Commands, game_assets: &GameAssets, position: Vec3, rotation: Quat) {
    let mut commands = commands.spawn((
        game_assets.gen_box_meshh_components(),
        Transform::from_translation(position).with_rotation(rotation),
        Name::new("Box"),
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(BOX_X_LENGTH, BOX_Y_LENGTH, BOX_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

fn spawn_crate(commands: &mut Commands, game_assets: &GameAssets, position: Vec3, rotation: Quat) {
    let mut commands = commands.spawn((
        game_assets.gen_crate_mesh_components(),
        Transform::from_translation(position).with_rotation(rotation),
        Name::new("Box"),
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(CRATE_X_LENGTH, CRATE_Y_LENGTH, CRATE_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

pub fn spawn_world(commands: &mut Commands, game_assets: &GameAssets, rotation: Quat) {
    commands.insert_resource(AmbientLight {
        color: css::WHITE.into(),
        brightness: 80.0,
    });

    commands.spawn((
        DirectionalLight {
            color: css::ORANGE_RED.into(),
            illuminance: 10000.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 0.0),
        Name::new("Sun"),
    ));

    spawn_floor(commands, game_assets, rotation);

    spawn_box(commands, game_assets, Vec3::new(5.0, 0.5, 5.0), rotation);
    spawn_box(commands, game_assets, Vec3::new(5.0, 0.5, -5.0), rotation);
    spawn_crate(commands, game_assets, Vec3::new(-5.0, 0.5, 5.0), rotation);
    spawn_crate(commands, game_assets, Vec3::new(-5.0, 0.5, -5.0), rotation);

    commands.spawn((
        Transform::from_translation(Vec3::new(0.0, 1.0, 0.0)),
        Name::new("Player Spawn"),
        spawn::PlayerSpawn,
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(-3.5, 1.0, -2.0)),
        Name::new("Ground Loot Spawn"),
        spawn::GroundLootSpawn,
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(3.5, 1.0, -2.0)),
        Name::new("Ground Loot Spawn"),
        spawn::GroundLootSpawn,
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(3.5, 1.0, 2.0)),
        Name::new("Ground Loot Spawn"),
        spawn::GroundLootSpawn,
    ));

    commands.spawn((
        Transform::from_translation(Vec3::new(-3.5, 1.0, 2.0)),
        Name::new("Ground Loot Spawn"),
        spawn::GroundLootSpawn,
    ));
}
