use bevy::prelude::*;

use crate::weapon;

#[derive(Debug)]
pub enum InventoryItem {}

#[allow(dead_code)]
#[derive(Debug, Default, Resource)]
pub struct Inventory {
    primary: Option<weapon::Weapon>,
    secondary: Option<weapon::Weapon>,

    items: Vec<InventoryItem>,
}
