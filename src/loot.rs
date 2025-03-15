use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};

use crate::{
    GameCollisionLayers, INTERACTABLE_INTERACT_LAYERS, LOOT_INTERACT_LAYERS, interactables,
    inventory,
};

#[derive(Debug, Component)]
pub struct GroundLoot(pub inventory::InventoryItem);

#[derive(Debug, Component)]
pub struct GroundLootModel;

pub fn spawn_ground_loot(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    spawn_transform: &GlobalTransform,
) {
    let mut commands = commands.spawn((
        spawn_transform.compute_transform(),
        Name::new("Ground Loot"),
        // TODO: randomize
        GroundLoot(inventory::InventoryItem::Grenade),
    ));

    commands.insert((
        RigidBody::Static,
        Collider::sphere(0.2),
        CollisionLayers::new(GameCollisionLayers::Loot, LOOT_INTERACT_LAYERS),
        //Mass(MASS),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
    ));

    commands.with_children(|parent| {
        parent.spawn((
            Mesh3d(meshes.add(Sphere::new(0.2))),
            MeshMaterial3d(materials.add(Color::from(css::FUCHSIA))),
            // TODO: this is because our temp model is at 1.0 instead of 0.0
            // and rotated 180 degrees around the Y axis
            /*Transform::from_xyz(0.0, -1.0, 0.0)
                .with_rotation(Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians())),
            SceneRoot(model),*/
            Name::new("Model"),
            GroundLootModel,
        ));

        parent.spawn((
            Collider::sphere(0.2),
            CollisionLayers::new(
                GameCollisionLayers::Interactable,
                INTERACTABLE_INTERACT_LAYERS,
            ),
            Sensor,
            Name::new("Interactable"),
            interactables::Interactable::GroundLoot,
        ));
    });
}
