use bevy::prelude::*;

use crate::{data, inventory};

#[derive(Debug)]
pub struct Weapon {
    #[allow(dead_code)]
    pub r#type: data::WeaponType,
    pub last_fire_ts: f32,
}

impl Weapon {
    pub fn new(item: inventory::InventoryItem) -> Self {
        match item {
            inventory::InventoryItem::Weapon(weapon_type) => Self {
                r#type: weapon_type,
                last_fire_ts: 0.0,
            },
            _ => unreachable!(),
        }
    }

    #[allow(dead_code)]
    pub fn can_fire(&self, data: &data::WeaponData, time: &Time) -> bool {
        self.last_fire_ts + data.fire_rate <= time.elapsed_secs()
    }
}
