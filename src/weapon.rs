use bevy::prelude::*;

use crate::{GameAssets, data, inventory, projectile};

#[derive(Debug)]
pub struct Weapon {
    pub r#type: data::WeaponType,
    pub last_fire_ts: f32,
    // TODO: magazine capacity
    // TODO: current ammo count
}

/*
TODO:

need an event for "start fire" and "end fire"
semi-auto fires on "start fire" only
full-auto fires on "start fire" and then on a timer every "cooldown" seconds
    ignore new "start fire" while on cooldown
burst fires on "start fire" and then on a timer every "cooldown" seconds until the burst is over
    this ignores "start fire" while it's bursting ?
*/

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

    pub fn can_fire(&self, datum: &data::WeaponDatum, time: &Time) -> bool {
        info!("TODO: verify ammo available");

        let data = datum.get(&self.r#type).unwrap();
        info!("TODO: handle fire mode {}", data.fire_mode);
        self.last_fire_ts + data.fire_rate <= time.elapsed_secs()
    }

    pub fn fire(
        &mut self,
        commands: &mut Commands,
        owner: Entity,
        datum: &data::WeaponDatum,
        time: &Time,
        origin: &Transform,
    ) {
        if !self.can_fire(datum, time) {
            return;
        }

        commands.trigger(FireWeaponEvent::new(owner, self.r#type, origin));
        self.last_fire_ts = time.elapsed_secs();

        info!("TODO: consume ammo");
    }
}

#[derive(Debug, Event)]
struct FireWeaponEvent {
    owner: Entity,
    weapon_type: data::WeaponType,
    origin: Vec3,
    direction: Dir3,
}

impl FireWeaponEvent {
    fn new(owner: Entity, weapon_type: data::WeaponType, transform: &Transform) -> Self {
        Self {
            owner,
            weapon_type,
            origin: transform.translation,
            direction: transform.forward(),
        }
    }
}

#[derive(Debug)]
pub struct WeaponPlugin;

impl Plugin for WeaponPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_fire_weapon);
    }
}

fn on_fire_weapon(
    trigger: Trigger<FireWeaponEvent>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    datum: Res<data::WeaponDataSource>,
) {
    let data = datum.get(&trigger.weapon_type).unwrap();

    // TODO: weapon data determines the projectile to spawn here
    projectile::spawn_bullet(
        &mut commands,
        &game_assets,
        trigger.owner,
        trigger.origin,
        trigger.direction,
        data.projectile_speed,
    )
    .observe(on_bullet_collision);
}

fn on_bullet_collision(trigger: Trigger<projectile::ProjectileCollisionEvent>) {
    info!(
        "bullet collision for {}: {}",
        trigger.entity(),
        trigger.target
    );
}
