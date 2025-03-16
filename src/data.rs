#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::LazyLock;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AmmoType {
    Light,
}

#[derive(Debug)]
pub struct AmmoData {
    pub name: String,
}

pub static AMMO_DATA: LazyLock<HashMap<AmmoType, AmmoData>> = LazyLock::new(|| {
    HashMap::from([(
        AmmoType::Light,
        AmmoData {
            name: "Light".to_owned(),
        },
    )])
});

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponType {
    Pistol,
}

#[derive(Debug)]
pub struct WeaponData {
    pub name: String,
    pub ammo_type: AmmoType,
    pub cooldown: f32,
    pub damage: usize,
}

pub static WEAPON_DATA: LazyLock<HashMap<WeaponType, WeaponData>> = LazyLock::new(|| {
    HashMap::from([(
        WeaponType::Pistol,
        WeaponData {
            name: "Pistol".to_owned(),
            ammo_type: AmmoType::Light,
            cooldown: 0.5,
            damage: 10,
        },
    )])
});
