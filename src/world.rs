use avian3d::prelude::*;
use bevy::{color::palettes::css, prelude::*};

#[derive(Debug)]
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, _app: &mut App) {}
}

fn spawn_floor(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    rotation: Quat,
) {
    let x_len = 30.0;
    let z_len = 30.0;

    let mut commands = commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(x_len, z_len))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(rotation),
        Name::new("Floor"),
    ));

    commands.insert((RigidBody::Static, Collider::cuboid(x_len, 0.1, z_len)));
}

fn spawn_box(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    color: Color,
    position: Vec3,
    rotation: Quat,
) {
    let x_len = 1.0;
    let y_len = 1.0;
    let z_len = 1.0;

    let mut commands = commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(x_len, y_len, z_len))),
        MeshMaterial3d(materials.add(color)),
        Transform::from_translation(position).with_rotation(rotation),
        Name::new("Box"),
    ));

    commands.insert((RigidBody::Static, Collider::cuboid(x_len, y_len, z_len)));
}

fn spawn_crate(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    color: Color,
    position: Vec3,
    rotation: Quat,
) {
    let x_len = 2.0;
    let y_len = 1.0;
    let z_len = 1.0;

    let mut commands = commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(x_len, y_len, z_len))),
        MeshMaterial3d(materials.add(color)),
        Transform::from_translation(position).with_rotation(rotation),
        Name::new("Box"),
    ));

    commands.insert((RigidBody::Static, Collider::cuboid(x_len, y_len, z_len)));
}

pub fn spawn_world(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    rotation: Quat,
) {
    commands.insert_resource(AmbientLight {
        color: css::WHITE.into(),
        brightness: 80.0,
    });

    commands.spawn((
        DirectionalLight {
            color: css::ORANGE_RED.into(),
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 8.0, 0.0),
        Name::new("Sun"),
    ));

    spawn_floor(commands, meshes, materials, rotation);

    // boxes
    spawn_box(
        commands,
        meshes,
        materials,
        Color::srgb(0.8, 0.7, 0.6),
        Vec3::new(5.0, 0.5, 5.0),
        rotation,
    );

    spawn_box(
        commands,
        meshes,
        materials,
        Color::srgb(0.8, 0.7, 0.6),
        Vec3::new(5.0, 0.5, -5.0),
        rotation,
    );

    spawn_crate(
        commands,
        meshes,
        materials,
        Color::srgb(0.8, 0.7, 0.6),
        Vec3::new(-5.0, 0.5, 5.0),
        rotation,
    );

    spawn_crate(
        commands,
        meshes,
        materials,
        Color::srgb(0.8, 0.7, 0.6),
        Vec3::new(-5.0, 0.5, -5.0),
        rotation,
    );
}
