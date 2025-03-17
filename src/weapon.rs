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
    pub fn can_fire(&self, datum: &data::WeaponDatum, time: &Time) -> bool {
        // TODO: handle semi-auto
        // TODO: verify ammo

        let data = datum.get(&self.r#type).unwrap();
        self.last_fire_ts + data.fire_rate <= time.elapsed_secs()
    }

    pub fn fire(&mut self, datum: &data::WeaponDatum, time: &Time) {
        if !self.can_fire(datum, time) {
            return;
        }

        // TODO:
        info!("firing weapon");
        self.last_fire_ts = time.elapsed_secs();

        // TODO: consume ammo
    }
}
