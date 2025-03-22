#![allow(dead_code)]

use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect, strum::Display, strum::EnumIter)]
pub enum AmmoType {
    Light,
}

#[derive(Debug)]
pub struct AmmoData {
    pub name: String,
    pub loot_size: usize,
    pub stack_size: usize,
}

pub type AmmoDatum = HashMap<AmmoType, AmmoData>;

#[derive(Debug, Deref, Resource)]
pub struct AmmoDataSource(AmmoDatum);

fn register_ammo_data(commands: &mut Commands) {
    commands.insert_resource(AmmoDataSource(AmmoDatum::from([(
        AmmoType::Light,
        AmmoData {
            name: "Light".to_owned(),
            loot_size: 20,
            stack_size: 50,
        },
    )])));
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Reflect, strum::Display, strum::EnumIter)]
pub enum WeaponType {
    Pistol,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, strum::Display)]
pub enum WeaponFireMode {
    SemiAuto,
    Burst(u8),
    FullAuto,
}

#[derive(Debug)]
pub struct WeaponData {
    pub name: String,
    pub ammo_type: AmmoType,
    pub magazine_size: usize,
    pub fire_mode: WeaponFireMode,
    pub fire_rate: f32,
    pub projectile_speed: f32,
    pub damage: usize,
}

pub type WeaponDatum = HashMap<WeaponType, WeaponData>;

#[derive(Debug, Deref, Resource)]
pub struct WeaponDataSource(WeaponDatum);

fn register_weapon_data(commands: &mut Commands) {
    commands.insert_resource(WeaponDataSource(WeaponDatum::from([(
        WeaponType::Pistol,
        WeaponData {
            name: "Pistol".to_owned(),
            ammo_type: AmmoType::Light,
            magazine_size: 10,
            fire_mode: WeaponFireMode::SemiAuto,
            fire_rate: 0.25,
            projectile_speed: 200.0,
            damage: 10,
        },
    )])));
}

pub fn register_data(commands: &mut Commands) {
    register_ammo_data(commands);
    register_weapon_data(commands);
}
