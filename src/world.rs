use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};

use crate::{GameCollisionLayers, WORLD_INTERACT_LAYERS, assets, spawn};

pub const CEILING_HEIGHT: f32 = 10.0;

pub const FLOOR_X_LENGTH: f32 = 50.0;
pub const FLOOR_Z_LENGTH: f32 = 50.0;

pub const BORDER_X_LENGTH: f32 = FLOOR_X_LENGTH;
pub const BORDER_Y_LENGTH: f32 = CEILING_HEIGHT;
pub const BORDER_Z_LENGTH: f32 = 1.0;

pub const WALL_X_LENGTH: f32 = 2.0;
pub const WALL_Y_LENGTH: f32 = 2.0;
pub const WALL_Z_LENGTH: f32 = 1.0;

pub const BOX_X_LENGTH: f32 = 1.0;
pub const BOX_Y_LENGTH: f32 = 1.0;
pub const BOX_Z_LENGTH: f32 = 1.0;

pub const CRATE_X_LENGTH: f32 = 2.0;
pub const CRATE_Y_LENGTH: f32 = 1.0;
pub const CRATE_Z_LENGTH: f32 = 1.0;

#[derive(Debug, Component)]
pub struct WorldBorder;

#[derive(Debug)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {}
}

fn spawn_ceiling(commands: &mut Commands) {
    let mut commands = commands.spawn((
        Transform::from_xyz(0.0, CEILING_HEIGHT, 0.0),
        Name::new("Ceiling"),
        WorldBorder,
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(FLOOR_X_LENGTH, 0.1, FLOOR_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

fn spawn_floor(commands: &mut Commands, game_assets: &assets::GameAssets) {
    let mut commands = commands.spawn((
        game_assets.gen_floor_mesh_components(),
        Transform::from_xyz(0.0, 0.0, 0.0),
        Name::new("Floor"),
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(FLOOR_X_LENGTH, 0.1, FLOOR_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

fn spawn_border(commands: &mut Commands, position: Vec3, rotation: Quat) {
    let mut commands = commands.spawn((
        Transform::from_translation(position).with_rotation(rotation),
        Name::new("World Border"),
        WorldBorder,
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(BORDER_X_LENGTH, BORDER_Y_LENGTH, BORDER_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

fn spawn_wall(
    commands: &mut Commands,
    game_assets: &assets::GameAssets,
    position: Vec3,
    rotation: Quat,
) {
    let mut commands = commands.spawn((
        game_assets.gen_wall_mesh_components(),
        Transform::from_translation(position).with_rotation(rotation),
        Name::new("Wall"),
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(WALL_X_LENGTH, WALL_Y_LENGTH, WALL_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

fn spawn_box(
    commands: &mut Commands,
    game_assets: &assets::GameAssets,
    position: Vec3,
    rotation: Quat,
) {
    let mut commands = commands.spawn((
        game_assets.gen_box_mesh_components(),
        Transform::from_translation(position).with_rotation(rotation),
        Name::new("Box"),
    ));

    commands.insert((
        RigidBody::Static,
        Collider::cuboid(BOX_X_LENGTH, BOX_Y_LENGTH, BOX_Z_LENGTH),
        CollisionLayers::new(GameCollisionLayers::World, WORLD_INTERACT_LAYERS),
    ));
}

fn spawn_crate(
    commands: &mut Commands,
    game_assets: &assets::GameAssets,
    position: Vec3,
    rotation: Quat,
) {
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

pub fn spawn_world(commands: &mut Commands, game_assets: &assets::GameAssets) {
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

    spawn_ceiling(commands);
    spawn_floor(commands, game_assets);

    let half_floor_x = FLOOR_X_LENGTH * 0.5;
    let half_floor_z = FLOOR_Z_LENGTH * 0.5;
    let half_border_y = BORDER_Y_LENGTH * 0.5;
    spawn_border(
        commands,
        Vec3::new(-half_floor_x, half_border_y, 0.0),
        Quat::from_rotation_y(90.0_f32.to_radians()),
    );
    spawn_border(
        commands,
        Vec3::new(half_floor_x, half_border_y, 0.0),
        Quat::from_rotation_y(90.0_f32.to_radians()),
    );
    spawn_border(
        commands,
        Vec3::new(0.00, half_border_y, -half_floor_z),
        Quat::default(),
    );
    spawn_border(
        commands,
        Vec3::new(0.0, half_border_y, half_floor_z),
        Quat::default(),
    );

    let half_wall_y = WALL_Y_LENGTH * 0.5;
    spawn_wall(
        commands,
        game_assets,
        Vec3::new(0.0, half_wall_y, -10.0),
        Quat::default(),
    );

    let half_box_y = BOX_Y_LENGTH * 0.5;
    spawn_box(
        commands,
        game_assets,
        Vec3::new(5.0, half_box_y, 5.0),
        Quat::from_rotation_y(45.0_f32.to_radians()),
    );
    spawn_box(
        commands,
        game_assets,
        Vec3::new(5.0, half_box_y, -5.0),
        Quat::from_rotation_y(45.0_f32.to_radians()),
    );

    let half_crate_y = CRATE_Y_LENGTH * 0.5;
    spawn_crate(
        commands,
        game_assets,
        Vec3::new(-5.0, half_crate_y, 5.0),
        Quat::from_rotation_y(45.0_f32.to_radians()),
    );
    spawn_crate(
        commands,
        game_assets,
        Vec3::new(-5.0, half_crate_y, -5.0),
        Quat::from_rotation_y(45.0_f32.to_radians()),
    );

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
