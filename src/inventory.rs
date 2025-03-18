use std::collections::HashMap;

use avian3d::prelude::*;
use bevy::prelude::*;
use rand::prelude::*;
use strum::{EnumCount, IntoEnumIterator};

use crate::{GameAssets, RandomSource, data, weapon};

pub const WEAPON_RADIUS: f32 = 0.25;
pub const WEAPON_LENGTH: f32 = 0.5;
pub const AMMO_LENGTH: f32 = 0.5;
pub const THROWABLE_RADIUS: f32 = 0.2;
pub const CONSUMABLE_RADIUS: f32 = 0.2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumCount)]
pub enum InventoryItem {
    Weapon(data::WeaponType),
    Ammo(data::AmmoType),
    Throwable,
    Consumable,
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
        game_assets: &GameAssets,
    ) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        match self {
            Self::Weapon(_) => game_assets.gen_weapon_mesh_components(),
            Self::Ammo(_) => game_assets.gen_ammo_mesh_components(),
            Self::Throwable => game_assets.gen_throwable_mesh_components(),
            Self::Consumable => game_assets.gen_consumable_mesh_components(),
        }
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

    pub fn get_selected_weapon_mut(&mut self) -> Option<&mut weapon::Weapon> {
        match self.selected_weapon {
            SelectedWeapon::Primary => self.primary.as_mut(),
            SelectedWeapon::Secondary => self.secondary.as_mut(),
        }
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
            InventoryItem::Weapon(_) => match self.selected_weapon {
                SelectedWeapon::Primary => {
                    self.primary = Some(weapon::Weapon::new(item));
                    true
                }
                SelectedWeapon::Secondary => {
                    self.secondary = Some(weapon::Weapon::new(item));
                    true
                }
            },
            InventoryItem::Ammo(_) | InventoryItem::Throwable | InventoryItem::Consumable => {
                *self.items.entry(item).or_default() += 1;
                true
            }
        }
    }
}
