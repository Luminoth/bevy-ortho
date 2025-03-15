mod camera;
mod cursor;
mod debug;
mod input;
mod interactables;
mod inventory;
mod loot;
mod player;
mod spawn;
mod weapon;
mod world;

use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};

const DEFAULT_RESOLUTION: (f32, f32) = (1280.0, 720.0);

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, Hash, States, Reflect)]
pub enum AppState {
    #[default]
    Init,
    LoadAssets,
    InGame,
}

#[derive(Debug, PhysicsLayer, Default)]
pub enum GameCollisionLayers {
    #[default]
    Default,
    World,
    Player,
    Loot,
    Interactable,
}

pub const WORLD_INTERACT_LAYERS: [GameCollisionLayers; 3] = [
    GameCollisionLayers::Default,
    GameCollisionLayers::Player,
    GameCollisionLayers::Loot,
];
pub const PLAYER_INTERACT_LAYERS: [GameCollisionLayers; 3] = [
    GameCollisionLayers::Default,
    GameCollisionLayers::World,
    GameCollisionLayers::Interactable,
];
pub const LOOT_INTERACT_LAYERS: [GameCollisionLayers; 2] =
    [GameCollisionLayers::Default, GameCollisionLayers::World];
pub const INTERACTABLE_INTERACT_LAYERS: [GameCollisionLayers; 1] = [GameCollisionLayers::Player];

pub fn show_cursor(window: &mut Window, show: bool) {
    window.cursor_options.grab_mode = if show {
        CursorGrabMode::None
    } else {
        CursorGrabMode::Locked
    };

    window.cursor_options.visible = show;
}

fn wait_for_window(
    frames: Res<bevy::core::FrameCount>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    // this is insane but we need to wait for the window
    // to show before we can grab the cursor
    // and there's no way to tell when that happens beyond waiting
    // for an arbitrary number of frames to pass by lol
    if frames.0 > 5 {
        app_state.set(AppState::LoadAssets);
    }
}

fn load_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    info!("load assets");

    camera::spawn_main_camera(&mut commands, 20.0, Vec3::new(0.0, 5.0, 5.0));

    world::spawn_world(
        &mut commands,
        &mut meshes,
        &mut materials,
        Quat::from_axis_angle(Vec3::Y, 45.0_f32.to_radians()),
    );

    app_state.set(AppState::InGame);
}

// TODO: this could be split up like
// spawn_player/s, spawn_loot, init_ui, etc
#[allow(clippy::too_many_arguments)]
fn enter_game(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut graphs: ResMut<Assets<AnimationGraph>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    player_spawn_query: Query<&GlobalTransform, With<spawn::PlayerSpawn>>,
    loot_spawn_query: Query<&GlobalTransform, With<spawn::GroundLootSpawn>>,
) {
    info!("enter game");

    let mut window = window_query.single_mut();
    show_cursor(&mut window, false);

    // UI
    cursor::spawn_cursor(
        &mut commands,
        Vec2::new(window.width() * 0.5, window.height() * 0.5),
    );

    commands.init_resource::<inventory::Inventory>();

    // player
    let player_spawn = player_spawn_query.single();
    player::spawn_player(&mut commands, &asset_server, &mut graphs, player_spawn);

    for loot_spawn in loot_spawn_query.iter() {
        loot::spawn_ground_loot(&mut commands, &mut meshes, &mut materials, loot_spawn);
    }
}

fn quit_game(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit::Success);
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
            cursor::CursorPlugin,
            world::WorldPlugin,
            player::PlayerPlugin,
            interactables::InteractablesPlugin,
            debug::DebugPlugin,
        ))
        // update continuously even while unfocused (for networking)
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
        })
        .init_state::<AppState>();

    app.add_systems(Update, wait_for_window.run_if(in_state(AppState::Init)))
        .add_systems(OnEnter(AppState::LoadAssets), load_assets)
        .add_systems(OnEnter(AppState::InGame), enter_game)
        .add_systems(
            Update,
            quit_game.run_if(in_state(AppState::InGame)).run_if(
                bevy::input::common_conditions::input_just_pressed(KeyCode::Escape),
            ),
        );

    app.run();
}
