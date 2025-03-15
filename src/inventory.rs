use std::collections::HashMap;

use bevy::prelude::*;

use crate::weapon;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum InventoryItem {
    Grenade,
}

#[derive(Debug, Default, Resource)]
pub struct Inventory {
    _primary: Option<weapon::Weapon>,
    _secondary: Option<weapon::Weapon>,

    items: HashMap<InventoryItem, u8>,
}

impl Inventory {
    pub fn add_item(&mut self, item: InventoryItem) -> bool {
        *self.items.entry(item).or_default() += 1;

        true
    }
}
