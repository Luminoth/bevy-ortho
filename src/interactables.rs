use std::ops::Deref;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{AppState, GameCollisionLayers, INTERACTABLE_INTERACT_LAYERS, inventory, loot, player};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Component, strum::Display)]
pub enum InteractableType {
    GroundLoot,
}

#[derive(Debug, Event)]
pub struct InteractEvent(pub Entity, pub InteractableType);

#[derive(Debug)]
pub struct InteractablesPlugin;

impl Plugin for InteractablesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            listen_interact
                .run_if(in_state(AppState::InGame))
                .after(player::PlayerSet),
        )
        .add_event::<InteractEvent>();
    }
}

fn listen_interact(
    mut commands: Commands,
    mut inventory: ResMut<inventory::Inventory>,
    mut evr_interact: EventReader<InteractEvent>,
    ground_loot_query: Query<&loot::GroundLoot>,
) {
    if evr_interact.is_empty() {
        return;
    }

    let evt = evr_interact.read().next().unwrap();
    match evt.1 {
        InteractableType::GroundLoot => {
            let loot = ground_loot_query.get(evt.0).unwrap();

            if inventory.add_item(*loot.deref()) {
                info!("picked up ground loot {:?}", loot.deref());
                commands.entity(evt.0).despawn_recursive();
            }
        }
    }

    evr_interact.clear();
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
