use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    GameCollisionLayers, INTERACTABLE_INTERACT_LAYERS, LOOT_INTERACT_LAYERS, RandomSource,
    interactables, inventory,
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

#[derive(Debug, Deref, Component)]
pub struct GroundLoot(inventory::InventoryItem);

#[derive(Debug, Component)]
#[require(Bobber)]
pub struct GroundLootModel;

#[derive(Debug)]
pub struct GroundLootPlugin;

impl Plugin for GroundLootPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, animate_bobbers);
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

pub fn spawn_ground_loot(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    random: &mut RandomSource,
    spawn_transform: &GlobalTransform,
) {
    let item = inventory::InventoryItem::new_random(random);

    let mut commands = commands.spawn((
        spawn_transform.compute_transform(),
        Name::new("Ground Loot"),
        GroundLoot(item),
    ));

    commands.insert((
        RigidBody::Static,
        item.gen_collider(),
        CollisionLayers::new(GameCollisionLayers::Loot, LOOT_INTERACT_LAYERS),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
    ));

    commands.with_children(|parent| {
        parent.spawn((
            item.gen_model(meshes, materials),
            // TODO: this is because our temp model is at 1.0 instead of 0.0
            // and rotated 180 degrees around the Y axis
            /*Transform::from_xyz(0.0, -1.0, 0.0)
                .with_rotation(Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians())),
            SceneRoot(model),*/
            Name::new("Model"),
            GroundLootModel,
        ));

        // TODO: move to interactables module
        parent.spawn((
            Collider::sphere(0.5),
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
