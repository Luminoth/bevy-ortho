mod camera;
mod input;
mod player;
mod world;

use bevy::prelude::*;

const DEFAULT_RESOLUTION: (f32, f32) = (1280.0, 720.0);

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    camera::spawn_main_camera(&mut commands, 20.0, Vec3::new(0.0, 10.0, 10.0));

    world::spawn_world(
        &mut commands,
        &mut meshes,
        &mut materials,
        Quat::from_axis_angle(Vec3::Y, 45.0_f32.to_radians()),
    );

    player::spawn_player(
        &mut commands,
        &mut meshes,
        &mut materials,
        Vec3::new(0.0, 1.0, 0.0),
    );
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
        // third party plugins
        .add_plugins((
            avian3d::PhysicsPlugins::default(), // TODO: this doesn't work with tnua: .set(PhysicsInterpolationPlugin::interpolate_all()),
            avian3d::debug_render::PhysicsDebugPlugin::default(),
            bevy_tnua::controller::TnuaControllerPlugin::new(avian3d::schedule::PhysicsSchedule),
            bevy_tnua_avian3d::TnuaAvian3dPlugin::new(avian3d::schedule::PhysicsSchedule),
        ))
        // game plugins
        .add_plugins((
            camera::OrthoCameraPlugin,
            input::InputPlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
        ))
        // update continuously even while unfocused (for networking)
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
        });

    app.add_systems(Startup, setup);

    app.run();
}
