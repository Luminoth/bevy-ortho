use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{GameAssets, GameCollisionLayers, PROJECTILE_INTERACT_LAYERS, player};

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Bullet {
    max_distance: f32,
    distance_traveled: f32,

    speed: f32,
}

#[derive(Debug, Component)]
pub struct BulletModel;

pub const RADIUS: f32 = 0.1;
const MASS: f32 = 0.005;

impl Bullet {
    fn new(speed: f32, max_distance: f32) -> Self {
        Self {
            max_distance,
            distance_traveled: 0.0,
            speed,
        }
    }
}

#[derive(Debug)]
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_bullets.after(player::PlayerSet));
    }
}

fn update_bullets(
    mut commands: Commands,
    time: Res<Time<Fixed>>,
    mut bullet_query: Query<(Entity, &mut Bullet, &mut Transform)>,
) {
    for (entity, mut bullet, mut transform) in bullet_query.iter_mut() {
        if bullet.distance_traveled > bullet.max_distance {
            commands.entity(entity).despawn_recursive();
            continue;
        }

        let direction = transform.forward();
        let distance = bullet.speed * time.elapsed_secs();
        transform.translation += direction * distance;

        bullet.distance_traveled += distance;
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    game_assets: &GameAssets,
    spawn_position: Vec3,
    facing: Dir3,
    speed: f32,
    max_distance: f32,
) {
    let mut commands = commands.spawn((
        Transform::from_translation(spawn_position).looking_to(facing, Vec3::Y),
        Name::new("Bullet"),
        Bullet::new(speed, max_distance),
    ));

    commands.insert((
        RigidBody::Dynamic,
        Collider::sphere(RADIUS),
        CollisionLayers::new(GameCollisionLayers::Projectile, PROJECTILE_INTERACT_LAYERS),
        Mass(MASS),
        LockedAxes::ROTATION_LOCKED,
    ));

    commands.with_children(|parent| {
        parent.spawn((
            game_assets.gen_bullet_mesh_components(),
            Name::new("Model"),
            BulletModel,
        ));
    });
}
