use std::borrow::Cow;

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{GameAssets, GameCollisionLayers, PROJECTILE_INTERACT_LAYERS};

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Projectile {
    owner: Entity,
}

impl Projectile {
    fn new(owner: Entity) -> Self {
        Self { owner }
    }
}

#[derive(Debug, Component)]
pub struct ProjectileModel;

#[derive(Debug, Event)]
pub struct ProjectileCollisionEvent {
    pub target: Entity,
}

pub const BULLET_RADIUS: f32 = 0.1;
const MASS: f32 = 0.005;

#[derive(Debug)]
pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostProcessCollisions,
            (filter_collisions, handle_collisions).chain(),
        );
    }
}

// TODO: if we stop spawning projectiles inside players
// we might be able to remove this
fn filter_collisions(
    mut collisions: ResMut<Collisions>,
    projectile_query: Query<(Entity, &Projectile)>,
) {
    for (entity, projectile) in projectile_query.iter() {
        collisions.remove_collision_pair(entity, projectile.owner);
    }
}

fn handle_collisions(
    mut commands: Commands,
    projectile_query: Query<(Entity, &CollidingEntities), With<Projectile>>,
) {
    for (entity, colliding_entities) in projectile_query.iter() {
        for colliding_entity in colliding_entities.iter() {
            debug!("projectile {} collides with {}", entity, colliding_entity);
            commands.trigger_targets(
                ProjectileCollisionEvent {
                    target: *colliding_entity,
                },
                entity,
            );
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn spawn_projectile<'a>(
    commands: &'a mut Commands,
    model: (Mesh3d, MeshMaterial3d<StandardMaterial>),
    name: impl Into<Cow<'static, str>>,
    radius: f32,
    owner: Entity,
    origin: Vec3,
    direction: Dir3,
    speed: f32,
) -> EntityCommands<'a> {
    let mut commands = commands.spawn((
        Transform::from_translation(origin).looking_to(direction, Vec3::Y),
        Visibility::default(),
        CollidingEntities::default(),
        Name::new(name),
        Projectile::new(owner),
    ));

    commands.insert((
        RigidBody::Dynamic,
        Collider::sphere(radius),
        CollisionLayers::new(GameCollisionLayers::Projectile, PROJECTILE_INTERACT_LAYERS),
        Mass(MASS),
        LinearVelocity(speed * direction),
        LockedAxes::ROTATION_LOCKED,
        //SweptCcd::default(),
    ));

    commands.with_children(|parent| {
        parent.spawn((model, Name::new("Model"), ProjectileModel));
    });

    commands
}

pub fn spawn_bullet<'a>(
    commands: &'a mut Commands,
    game_assets: &GameAssets,
    owner: Entity,
    origin: Vec3,
    direction: Dir3,
    speed: f32,
) -> EntityCommands<'a> {
    spawn_projectile(
        commands,
        game_assets.gen_bullet_mesh_components(),
        "Bullet",
        BULLET_RADIUS,
        owner,
        origin,
        direction,
        speed,
    )
}
