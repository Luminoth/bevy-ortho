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
    let mut commands = commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(10.0, 10.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
        Transform::from_xyz(0.0, 0.0, 0.0).with_rotation(rotation),
    ));

    commands.insert((
        RigidBody::Static,
        // TODO: can we infer this from the mesh?
        Collider::cuboid(10.0, 0.1, 10.0),
    ));
}

fn spawn_box(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    color: Color,
    position: Vec3,
    rotation: Quat,
) {
    let mut commands = commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(color)),
        Transform::from_translation(position).with_rotation(rotation),
    ));

    commands.insert((
        RigidBody::Static,
        // TODO: can we infer this from the mesh?
        Collider::cuboid(1.0, 1.0, 1.0),
    ));
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
            shadows_enabled: true,
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
        Vec3::new(1.5, 0.5, 1.5),
        rotation,
    );

    spawn_box(
        commands,
        meshes,
        materials,
        Color::srgb(0.8, 0.7, 0.6),
        Vec3::new(1.5, 0.5, -1.5),
        rotation,
    );

    spawn_box(
        commands,
        meshes,
        materials,
        Color::srgb(0.8, 0.7, 0.6),
        Vec3::new(-1.5, 0.5, 1.5),
        rotation,
    );

    spawn_box(
        commands,
        meshes,
        materials,
        Color::srgb(0.8, 0.7, 0.6),
        Vec3::new(-1.5, 0.5, -1.5),
        rotation,
    );
}
