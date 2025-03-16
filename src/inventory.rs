use std::collections::HashMap;

use bevy::prelude::*;
use rand::prelude::*;
use strum::IntoEnumIterator;

use crate::{RandomSource, weapon};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum InventoryItem {
    Weapon,
    Throwable,
    Consumable,
}

impl InventoryItem {
    pub fn new_random(rng: &mut RandomSource) -> Self {
        InventoryItem::iter().choose(rng).unwrap()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum SelectedWeapon {
    #[default]
    Primary,
    Secondary,
}

#[derive(Debug, Default, Resource)]
pub struct Inventory {
    primary: Option<weapon::Weapon>,
    secondary: Option<weapon::Weapon>,
    selected_weapon: SelectedWeapon,

    items: HashMap<InventoryItem, u8>,
}

impl Inventory {
    pub fn has_weapon(&self) -> bool {
        self.primary.is_some() || self.secondary.is_some()
    }

    pub fn select_weapon(&mut self, weapon: SelectedWeapon) {
        self.selected_weapon = weapon;
    }

    pub fn toggle_weapon(&mut self) {
        match self.selected_weapon {
            SelectedWeapon::Primary => self.select_weapon(SelectedWeapon::Secondary),
            SelectedWeapon::Secondary => self.select_weapon(SelectedWeapon::Primary),
        }
    }

    pub fn add_item(&mut self, item: InventoryItem) -> bool {
        match item {
            InventoryItem::Weapon => match self.selected_weapon {
                SelectedWeapon::Primary => {
                    self.primary = Some(weapon::Weapon::new(item));
                    true
                }
                SelectedWeapon::Secondary => {
                    self.secondary = Some(weapon::Weapon::new(item));
                    true
                }
            },
            InventoryItem::Throwable | InventoryItem::Consumable => {
                *self.items.entry(item).or_default() += 1;
                true
            }
        }
    }
}
