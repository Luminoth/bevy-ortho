mod camera;
mod input;

use bevy::prelude::*;

const DEFAULT_RESOLUTION: (f32, f32) = (1280.0, 720.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    camera::spawn_main_camera(&mut commands, Vec3::new(5.0, 5.0, 5.0), Vec3::ZERO);

    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(100.0, 100.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.3, 0.5, 0.3))),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.5, 0.5, 1.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(1.5, 0.5, -1.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(-1.5, 0.5, 1.5),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.8, 0.7, 0.6))),
        Transform::from_xyz(-1.5, 0.5, -1.5),
    ));

    commands.spawn((PointLight::default(), Transform::from_xyz(3.0, 8.0, 5.0)));
}

fn main() {
    let mut app = App::new();
    app
        // bevy plugins
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Bevy Ortho Jam".into(),
                        resolution: DEFAULT_RESOLUTION.into(),
                        ..default()
                    }),
                    ..default()
                })
                .set(bevy::log::LogPlugin {
                    // default bevy filter plus silence some spammy 3rd party crates
                    filter: format!(
                        "{},symphonia_core=error,symphonia_bundle_mp3=error",
                        bevy::log::DEFAULT_FILTER
                    ),
                    ..default()
                }),
        )
        // game plugins
        .add_plugins((camera::OrthoCameraPlugin, input::InputPlugin))
        // update continuously even while unfocused (for networking)
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
        });

    app.add_systems(Startup, setup);

    app.run();
}
