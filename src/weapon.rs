use bevy::prelude::*;

use crate::{assets, data, projectile};

#[derive(Debug, Reflect)]
pub struct Weapon {
    pub r#type: data::WeaponType,
    pub ammo_count: usize,

    pub last_fire_ts: f32,
}

/*
TODO:

semi-auto fires on "start fire" only
full-auto fires on "start fire" and then on a timer every "cooldown" seconds
    ignore new "start fire" while on cooldown
burst fires on "start fire" and then on a timer every "cooldown" seconds until the burst is over
    this ignores "start fire" while it's bursting ?
*/

impl Weapon {
    pub fn new(weapon_type: data::WeaponType, ammo_count: usize) -> Self {
        Self {
            r#type: weapon_type,
            ammo_count,
            last_fire_ts: 0.0,
        }
    }

    pub fn can_fire(&self, datum: &data::WeaponDatum, time: &Time) -> bool {
        if self.ammo_count < 1 {
            return false;
        }

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
    ) -> bool {
        if !self.can_fire(datum, time) {
            return false;
        }

        commands.trigger(FireWeaponEvent::new(owner, self.r#type, origin));
        self.last_fire_ts = time.elapsed_secs();

        self.ammo_count -= 1;

        true
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
    game_assets: Res<assets::GameAssets>,
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
