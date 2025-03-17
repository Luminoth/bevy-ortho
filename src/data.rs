#![allow(dead_code)]

use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum AmmoType {
    Light,
}

#[derive(Debug)]
pub struct AmmoData {
    pub name: String,
}

#[derive(Debug, Deref, Resource)]
pub struct AmmoDataSource(HashMap<AmmoType, AmmoData>);

fn register_ammo_data(commands: &mut Commands) {
    commands.insert_resource(AmmoDataSource(HashMap::from([(
        AmmoType::Light,
        AmmoData {
            name: "Light".to_owned(),
        },
    )])));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::EnumIter)]
pub enum WeaponType {
    Pistol,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WeaponFireMode {
    SemiAuto,
    Burst(u8),
    FullAuto,
}

#[derive(Debug)]
pub struct WeaponData {
    pub name: String,
    pub ammo_type: AmmoType,
    pub fire_mode: WeaponFireMode,
    pub fire_rate: f32,
    pub damage: usize,
}

#[derive(Debug, Deref, Resource)]
pub struct WeaponDataSource(HashMap<WeaponType, WeaponData>);

fn register_weapon_data(commands: &mut Commands) {
    commands.insert_resource(WeaponDataSource(HashMap::from([(
        WeaponType::Pistol,
        WeaponData {
            name: "Pistol".to_owned(),
            ammo_type: AmmoType::Light,
            fire_mode: WeaponFireMode::SemiAuto,
            fire_rate: 0.5,
            damage: 10,
        },
    )])));
}

pub fn register_data(commands: &mut Commands) {
    register_ammo_data(commands);
    register_weapon_data(commands);
}
