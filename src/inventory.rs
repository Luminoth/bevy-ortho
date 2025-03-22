use bevy::prelude::*;
use rand::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

use crate::{RandomSource, data, weapon};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect, strum::Display, strum::EnumCount)]
pub enum InventoryItem {
    Weapon(data::WeaponType, usize),
    Ammo(data::AmmoType, usize),
    Throwable,
    Consumable,
    // TODO: character mods (abilities, passives, etc)
    // TODO: weapon mods
}

impl InventoryItem {
    pub fn random_loot(
        rng: &mut RandomSource,
        weapon_datum: &data::WeaponDatum,
        ammo_datum: &data::AmmoDatum,
    ) -> Self {
        // TODO: bro this sucks lol
        match rng.random_range(..Self::COUNT) {
            0 => {
                let weapon_type = data::WeaponType::iter().choose(rng).unwrap();
                let weapon_data = weapon_datum.get(&weapon_type).unwrap();
                Self::Weapon(weapon_type, weapon_data.magazine_size)
            }
            1 => {
                let ammo_type = data::AmmoType::iter().choose(rng).unwrap();
                let ammo_data = ammo_datum.get(&ammo_type).unwrap();
                Self::Ammo(ammo_type, ammo_data.loot_size)
            }
            2 => Self::Throwable,
            3 => Self::Consumable,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default, Reflect, strum::Display)]
pub enum WeaponSlot {
    #[default]
    Primary,
    Secondary,
}

#[derive(Debug, Resource, Reflect)]
pub struct Inventory {
    primary: Option<weapon::Weapon>,
    secondary: Option<weapon::Weapon>,
    selected_weapon: WeaponSlot,

    size: usize,
    items: Vec<InventoryItem>,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            primary: None,
            secondary: None,
            selected_weapon: WeaponSlot::default(),
            // TODO: size should come from data
            size: 10,
            items: Vec::with_capacity(10),
        }
    }
}

impl Inventory {
    pub fn has_weapon(&self) -> bool {
        self.primary.is_some() || self.secondary.is_some()
    }

    pub fn get_primary_weapon(&self) -> Option<&weapon::Weapon> {
        self.primary.as_ref()
    }

    pub fn get_secondary_weapon(&self) -> Option<&weapon::Weapon> {
        self.secondary.as_ref()
    }

    pub fn get_items(&self) -> &Vec<InventoryItem> {
        &self.items
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
            InventoryItem::Weapon(weapon_type, ammo_count) => {
                if self.get_selected_weapon_item().is_none() {
                    self.set_selected_weapon_item(weapon::Weapon::new(weapon_type, ammo_count));
                    true
                } else if self.get_unselected_weapon_item().is_none() {
                    self.set_unselected_weapon_item(weapon::Weapon::new(weapon_type, ammo_count));
                    true
                } else {
                    warn!("TODO: hold to weapon swap");
                    false
                }
            }
            InventoryItem::Ammo(_, _) | InventoryItem::Throwable | InventoryItem::Consumable => {
                if self.items.len() >= self.size {
                    return false;
                }

                warn!("TODO: stack inventory items");

                self.items.push(item);

                warn!("TODO: sort inventory items");

                true
            }
        }
    }
}

#[derive(Debug)]
pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Inventory>()
            .register_type::<Inventory>();
    }
}
