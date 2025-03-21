use std::collections::HashMap;

use avian3d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

use crate::{RandomSource, assets, data, weapon};

pub const WEAPON_RADIUS: f32 = 0.25;
pub const WEAPON_LENGTH: f32 = 0.5;
pub const AMMO_LENGTH: f32 = 0.5;
pub const THROWABLE_RADIUS: f32 = 0.2;
pub const CONSUMABLE_RADIUS: f32 = 0.2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::Display, strum::EnumCount)]
pub enum InventoryItem {
    // TODO: weapon loot should have an ammo count
    Weapon(data::WeaponType),
    // TODO: ammo loot should have an ammo count
    Ammo(data::AmmoType),
    Throwable,
    Consumable,
    // TODO: character mods (abilities, passives, etc)
    // TODO: weapon mods
}

impl InventoryItem {
    pub fn new_random(rng: &mut RandomSource) -> Self {
        // TODO: bro this sucks lol
        match rng.random_range(..Self::COUNT) {
            0 => Self::Weapon(data::WeaponType::iter().choose(rng).unwrap()),
            1 => Self::Ammo(data::AmmoType::iter().choose(rng).unwrap()),
            2 => Self::Throwable,
            3 => Self::Consumable,
            _ => unreachable!(),
        }
    }

    pub fn gen_collider(&self) -> Collider {
        Collider::sphere(0.2);

        match self {
            Self::Weapon(_) => Collider::capsule(WEAPON_RADIUS, WEAPON_LENGTH),
            Self::Ammo(_) => Collider::cuboid(AMMO_LENGTH, AMMO_LENGTH, AMMO_LENGTH),
            Self::Throwable => Collider::sphere(THROWABLE_RADIUS),
            Self::Consumable => Collider::sphere(CONSUMABLE_RADIUS),
        }
    }

    pub fn gen_model(
        &self,
        game_assets: &assets::GameAssets,
    ) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        match self {
            Self::Weapon(_) => game_assets.gen_weapon_mesh_components(),
            Self::Ammo(_) => game_assets.gen_ammo_mesh_components(),
            Self::Throwable => game_assets.gen_throwable_mesh_components(),
            Self::Consumable => game_assets.gen_consumable_mesh_components(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, strum::Display)]
pub enum WeaponSlot {
    #[default]
    Primary,
    Secondary,
}

#[derive(Debug, Default, Resource)]
pub struct Inventory {
    primary: Option<weapon::Weapon>,
    secondary: Option<weapon::Weapon>,
    selected_weapon: WeaponSlot,

    items: HashMap<InventoryItem, u8>,
}

impl Inventory {
    pub fn has_weapon(&self) -> bool {
        self.primary.is_some() || self.secondary.is_some()
    }

    fn get_weapon_item(&self, weapon_slot: WeaponSlot) -> Option<&weapon::Weapon> {
        match weapon_slot {
            WeaponSlot::Primary => self.primary.as_ref(),
            WeaponSlot::Secondary => self.secondary.as_ref(),
        }
    }

    fn get_weapon_item_mut(&mut self, weapon_slot: WeaponSlot) -> Option<&mut weapon::Weapon> {
        match weapon_slot {
            WeaponSlot::Primary => self.primary.as_mut(),
            WeaponSlot::Secondary => self.secondary.as_mut(),
        }
    }

    pub fn get_selected_weapon_item(&mut self) -> Option<&weapon::Weapon> {
        self.get_weapon_item(self.selected_weapon)
    }

    fn get_unselected_weapon_item(&mut self) -> Option<&weapon::Weapon> {
        match self.selected_weapon {
            WeaponSlot::Primary => self.get_weapon_item(WeaponSlot::Secondary),
            WeaponSlot::Secondary => self.get_weapon_item(WeaponSlot::Primary),
        }
    }

    pub fn get_selected_weapon_item_mut(&mut self) -> Option<&mut weapon::Weapon> {
        self.get_weapon_item_mut(self.selected_weapon)
    }

    fn set_weapon_item(&mut self, weapon_slot: WeaponSlot, weapon: weapon::Weapon) {
        match weapon_slot {
            WeaponSlot::Primary => {
                info!("setting primary weapon {:?}", weapon);
                self.primary = Some(weapon);
            }
            WeaponSlot::Secondary => {
                info!("setting secondary weapon {:?}", weapon);
                self.secondary = Some(weapon);
            }
        }
        warn!("TODO: handle replace weapon");
    }

    fn set_selected_weapon_item(&mut self, weapon: weapon::Weapon) {
        self.set_weapon_item(self.selected_weapon, weapon);
    }

    fn set_unselected_weapon_item(&mut self, weapon: weapon::Weapon) {
        match self.selected_weapon {
            WeaponSlot::Primary => self.set_weapon_item(WeaponSlot::Secondary, weapon),
            WeaponSlot::Secondary => self.set_weapon_item(WeaponSlot::Primary, weapon),
        }
    }

    pub fn set_selected_weapon(&mut self, weapon_slot: WeaponSlot) {
        info!(
            "select weapon {}: {}",
            weapon_slot,
            self.get_weapon_item(weapon_slot).is_some()
        );
        self.selected_weapon = weapon_slot;
    }

    pub fn toggle_selected_weapon(&mut self) {
        match self.selected_weapon {
            WeaponSlot::Primary => self.set_selected_weapon(WeaponSlot::Secondary),
            WeaponSlot::Secondary => self.set_selected_weapon(WeaponSlot::Primary),
        }
    }

    pub fn add_item(&mut self, item: InventoryItem) -> bool {
        match item {
            InventoryItem::Weapon(_) => {
                if self.get_selected_weapon_item().is_none() {
                    self.set_selected_weapon_item(weapon::Weapon::new(item));
                    true
                } else if self.get_unselected_weapon_item().is_none() {
                    self.set_unselected_weapon_item(weapon::Weapon::new(item));
                    true
                } else {
                    false
                }
            }
            InventoryItem::Ammo(_) | InventoryItem::Throwable | InventoryItem::Consumable => {
                info!("adding item {}", item);
                warn!("TODO: verify space available");
                *self.items.entry(item).or_default() += 1;
                true
            }
        }
    }
}
