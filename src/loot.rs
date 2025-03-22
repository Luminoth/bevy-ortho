use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};

use crate::{
    GameCollisionLayers, LOOT_INTERACT_LAYERS, RandomSource, assets, data, interactables, inventory,
};

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Bobber {
    bob_amp: f32,
    bob_speed: f32,
    rot_speed: f32,
}

impl Default for Bobber {
    fn default() -> Self {
        Self {
            bob_amp: 0.5,
            bob_speed: 1.25,
            rot_speed: 0.25,
        }
    }
}

const WEAPON_RADIUS: f32 = 0.25;
const WEAPON_LENGTH: f32 = 0.5;
const AMMO_LENGTH: f32 = 0.5;
const THROWABLE_RADIUS: f32 = 0.2;
const CONSUMABLE_RADIUS: f32 = 0.2;

#[derive(Debug, Deref, Component, Reflect)]
pub struct GroundLoot(inventory::InventoryItem);

#[derive(Debug, Component)]
#[require(Bobber)]
pub struct GroundLootModel;

#[derive(Debug)]
pub struct GroundLootPlugin;

impl Plugin for GroundLootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_bobbers)
            .register_type::<GroundLoot>();
    }
}

fn animate_bobbers(time: Res<Time>, mut bobber_query: Query<(&Bobber, &mut Transform)>) {
    for (bobber, mut transform) in bobber_query.iter_mut() {
        transform.translation.y = bobber.bob_amp
            + bobber.bob_amp
                * (time.elapsed_secs() * bobber.bob_speed * std::f32::consts::FRAC_PI_2).sin();

        transform.rotate_y(std::f32::consts::TAU * time.delta_secs() * bobber.rot_speed);
    }
}

pub fn load_weapon_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> assets::MeshMaterial {
    assets::MeshMaterial {
        mesh: meshes.add(Capsule3d::new(WEAPON_RADIUS, WEAPON_LENGTH)),
        material: materials.add(Color::from(css::DARK_RED)),
    }
}

pub fn load_ammo_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> assets::MeshMaterial {
    assets::MeshMaterial {
        mesh: meshes.add(Cuboid::new(AMMO_LENGTH, AMMO_LENGTH, AMMO_LENGTH)),
        material: materials.add(Color::from(css::GREEN_YELLOW)),
    }
}

pub fn load_throwable_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> assets::MeshMaterial {
    assets::MeshMaterial {
        mesh: meshes.add(Sphere::new(THROWABLE_RADIUS)),
        material: materials.add(Color::from(css::GREY)),
    }
}

pub fn load_consumable_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> assets::MeshMaterial {
    assets::MeshMaterial {
        mesh: meshes.add(Sphere::new(CONSUMABLE_RADIUS)),
        material: materials.add(Color::from(css::WHITE)),
    }
}

pub fn spawn_ground_loot(
    commands: &mut Commands,
    game_assets: &assets::GameAssets,
    weapon_datum: &data::WeaponDatum,
    ammo_datum: &data::AmmoDatum,
    random: &mut RandomSource,
    spawn_transform: &GlobalTransform,
) {
    let item = inventory::InventoryItem::random_loot(random, weapon_datum, ammo_datum);

    let (model, collider) = match item {
        inventory::InventoryItem::Weapon(_, _) => (
            game_assets.gen_weapon_mesh_components(),
            Collider::capsule(WEAPON_RADIUS, WEAPON_LENGTH),
        ),
        inventory::InventoryItem::Ammo(_, _) => (
            game_assets.gen_ammo_mesh_components(),
            Collider::cuboid(AMMO_LENGTH, AMMO_LENGTH, AMMO_LENGTH),
        ),
        inventory::InventoryItem::Throwable => (
            game_assets.gen_throwable_mesh_components(),
            Collider::sphere(THROWABLE_RADIUS),
        ),
        inventory::InventoryItem::Consumable => (
            game_assets.gen_consumable_mesh_components(),
            Collider::sphere(CONSUMABLE_RADIUS),
        ),
    };

    let mut commands = commands.spawn((
        spawn_transform.compute_transform(),
        Visibility::default(),
        Name::new("Ground Loot"),
        GroundLoot(item),
    ));

    commands.insert((
        RigidBody::Static,
        collider,
        CollisionLayers::new(GameCollisionLayers::Loot, LOOT_INTERACT_LAYERS),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
    ));

    commands.with_children(|parent| {
        parent.spawn((model, Name::new("Model"), GroundLootModel));

        interactables::spawn_interactable(parent, interactables::InteractableType::GroundLoot);
    });
}
