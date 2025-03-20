use std::ops::Deref;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{GameCollisionLayers, INTERACTABLE_INTERACT_LAYERS, inventory, loot};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Component, strum::Display)]
pub enum InteractableType {
    GroundLoot,
}

#[derive(Debug, Event)]
pub struct InteractEvent {
    pub target: Entity,
    pub target_type: InteractableType,
}

#[derive(Debug)]
pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_interact);
    }
}

fn on_interact(
    trigger: Trigger<InteractEvent>,
    mut commands: Commands,
    mut inventory: ResMut<inventory::Inventory>,
    ground_loot_query: Query<&loot::GroundLoot>,
) {
    match trigger.target_type {
        InteractableType::GroundLoot => {
            let loot = ground_loot_query.get(trigger.target).unwrap();

            if inventory.add_item(*loot.deref()) {
                info!("picked up ground loot {:?}", loot.deref());
                commands.entity(trigger.target).despawn_recursive();
            }
        }
    }
}

pub fn spawn_interactable(parent: &mut ChildBuilder, r#type: InteractableType) {
    parent.spawn((
        Collider::sphere(0.5),
        CollisionLayers::new(
            GameCollisionLayers::Interactable,
            INTERACTABLE_INTERACT_LAYERS,
        ),
        Sensor,
        Name::new("Interactable"),
        r#type,
    ));
}
