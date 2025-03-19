use bevy::prelude::*;

use crate::{GameAssets, bullet, data, inventory};

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

    pub fn fire(
        &mut self,
        commands: &mut Commands,
        datum: &data::WeaponDatum,
        time: &Time,
        origin: &Transform,
    ) {
        if !self.can_fire(datum, time) {
            return;
        }

        // TODO:
        commands.trigger(FireWeaponEvent::from_transform(origin));
        self.last_fire_ts = time.elapsed_secs();

        // TODO: consume ammo
    }
}

#[derive(Event)]
struct FireWeaponEvent {
    origin: Vec3,
    direction: Dir3,
}

impl FireWeaponEvent {
    fn from_transform(transform: &Transform) -> Self {
        Self {
            origin: transform.translation,
            direction: transform.forward(),
        }
    }
}

#[derive(Debug)]
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(fire_weapon_handler);
    }
}

fn fire_weapon_handler(
    trigger: Trigger<FireWeaponEvent>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    bullet::spawn_bullet(
        &mut commands,
        &game_assets,
        trigger.origin,
        trigger.direction,
        // TODO: these from weapon / ammo data ?
        0.25,
        25.0,
    );
}
