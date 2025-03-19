use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{GameAssets, GameCollisionLayers, PROJECTILE_INTERACT_LAYERS};

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Bullet {
    owner: Entity,
    origin: Vec3,
    max_distance: f32,
}

#[derive(Debug, Component)]
pub struct BulletModel;

pub const RADIUS: f32 = 0.1;
const MASS: f32 = 0.005;

impl Bullet {
    fn new(owner: Entity, origin: Vec3, max_distance: f32) -> Self {
        Self {
            owner,
            origin,
            max_distance,
        }
    }
}

#[derive(Debug)]
pub struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostProcessCollisions,
            (filter_collisions, handle_collisions).chain(),
        )
        .add_systems(PostUpdate, check_bullet_despawn);
    }
}

fn check_bullet_despawn(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Bullet, &Transform)>,
) {
    for (entity, bullet, transform) in bullet_query.iter() {
        if bullet.origin.distance(transform.translation) > bullet.max_distance {
            debug!("despawning stray bullet");
            commands.entity(entity).despawn_recursive();
        }
    }
}

// TODO: if we stop spawning bullets inside players
// we might be able to remove this
fn filter_collisions(mut collisions: ResMut<Collisions>, bullet_query: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in bullet_query.iter() {
        collisions.remove_collision_pair(entity, bullet.owner);
    }
}

fn handle_collisions(
    mut commands: Commands,
    bullet_query: Query<(Entity, &CollidingEntities), With<Bullet>>,
) {
    for (entity, colliding_entities) in bullet_query.iter() {
        for colliding_entity in colliding_entities.iter() {
            debug!("bullet {} collides with {}", entity, colliding_entity);
            warn!("TODO: signal bullet collision");
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    game_assets: &GameAssets,
    owner: Entity,
    origin: Vec3,
    direction: Dir3,
    speed: f32,
    max_distance: f32,
) {
    let mut commands = commands.spawn((
        Transform::from_translation(origin).looking_to(direction, Vec3::Y),
        Visibility::default(),
        CollidingEntities::default(),
        Name::new("Bullet"),
        Bullet::new(owner, origin, max_distance),
    ));

    commands.insert((
        RigidBody::Dynamic,
        Collider::sphere(RADIUS),
        CollisionLayers::new(GameCollisionLayers::Projectile, PROJECTILE_INTERACT_LAYERS),
        Mass(MASS),
        LinearVelocity(speed * direction),
        LockedAxes::ROTATION_LOCKED,
        //SweptCcd::default(),
    ));

    commands.with_children(|parent| {
        parent.spawn((
            game_assets.gen_bullet_mesh_components(),
            Name::new("Model"),
            BulletModel,
        ));
    });
}
