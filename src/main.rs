mod assets;
mod camera;
mod cursor;
mod data;
mod debug;
mod hud;
mod input;
mod interactables;
mod inventory;
mod loot;
mod player;
mod projectile;
mod spawn;
mod ui;
mod weapon;
mod world;

use avian3d::prelude::*;
use bevy::{
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use rand::prelude::*;

const DEFAULT_RESOLUTION: (f32, f32) = (1280.0, 720.0);

#[derive(Debug, Deref, DerefMut, Resource)]
pub struct RandomSource(StdRng);

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
    Projectile,
    Interactable,
}

pub const WORLD_INTERACT_LAYERS: [GameCollisionLayers; 4] = [
    GameCollisionLayers::Default,
    GameCollisionLayers::Player,
    GameCollisionLayers::Loot,
    GameCollisionLayers::Projectile,
];
pub const PLAYER_INTERACT_LAYERS: [GameCollisionLayers; 4] = [
    GameCollisionLayers::Default,
    GameCollisionLayers::World,
    GameCollisionLayers::Projectile,
    GameCollisionLayers::Interactable,
];
pub const LOOT_INTERACT_LAYERS: [GameCollisionLayers; 2] =
    [GameCollisionLayers::Default, GameCollisionLayers::World];
pub const PROJECTILE_INTERACT_LAYERS: [GameCollisionLayers; 3] = [
    GameCollisionLayers::Default,
    GameCollisionLayers::World,
    GameCollisionLayers::Player,
];
pub const INTERACTABLE_INTERACT_LAYERS: [GameCollisionLayers; 1] = [GameCollisionLayers::Player];

pub fn show_cursor(window: &mut Window, show: bool) {
    window.cursor_options.grab_mode = if show {
        CursorGrabMode::None
    } else {
        CursorGrabMode::Locked
    };

    window.cursor_options.visible = show;
}

fn setup(mut commands: Commands) {
    let rng = StdRng::from_rng(&mut rand::rng());
    commands.insert_resource(RandomSource(rng));

    data::register_data(&mut commands);
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

fn wait_for_assets(mut app_state: ResMut<NextState<AppState>>) {
    warn!("TODO: wait for assets to load");

    app_state.set(AppState::InGame);
}

fn init_ui(mut commands: Commands, mut window_query: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = window_query.single_mut();
    show_cursor(&mut window, false);

    hud::spawn_hud(&mut commands);

    cursor::spawn_cursor(
        &mut commands,
        Vec2::new(window.width() * 0.5, window.height() * 0.5),
    );
}

fn spawn_loot(
    mut commands: Commands,
    game_assets: Res<assets::GameAssets>,
    weapon_datum: Res<data::WeaponDataSource>,
    ammo_datum: Res<data::AmmoDataSource>,
    mut random: ResMut<RandomSource>,
    loot_spawn_query: Query<&GlobalTransform, With<spawn::GroundLootSpawn>>,
) {
    for loot_spawn in loot_spawn_query.iter() {
        loot::spawn_ground_loot(
            &mut commands,
            &game_assets,
            &weapon_datum,
            &ammo_datum,
            &mut random,
            loot_spawn,
        );
    }
}

fn spawn_player(
    mut commands: Commands,
    mut random: ResMut<RandomSource>,
    game_assets: Res<assets::GameAssets>,
    player_spawn_query: Query<&GlobalTransform, With<spawn::PlayerSpawn>>,
) {
    let mut player_spawns = player_spawn_query.iter().collect::<Vec<_>>();

    // TODO: this would be done for each player / party
    let idx = (0..player_spawns.len()).choose(&mut random).unwrap();
    let player_spawn = player_spawns.swap_remove(idx);
    player::spawn_player(&mut commands, &game_assets, player_spawn);
}

// TODO: put this in the debug plugin
fn save_scene(world: &World) {
    use std::io::Write;

    let scene = DynamicSceneBuilder::from_world(world)
        .deny_resource::<Time<Real>>()
        .deny_resource::<assets::GameAssets>()
        .deny_component::<bevy::render::camera::CameraRenderGraph>()
        .deny_component::<bevy::render::camera::Exposure>()
        .deny_component::<bevy::render::camera::CameraMainTextureUsages>()
        .deny_component::<bevy::render::view::ColorGrading>()
        .deny_component::<bevy::render::mesh::skinning::SkinnedMesh>()
        .deny_component::<Mesh3d>()
        .deny_component::<MeshMaterial3d<StandardMaterial>>()
        .deny_component::<SceneRoot>()
        .extract_entities(world.iter_entities().map(|entity| entity.id()))
        .extract_resources()
        .build();

    let type_registry = world.resource::<AppTypeRegistry>();
    let type_registry = type_registry.read();
    let serialized_scene = scene.serialize(&type_registry).unwrap();

    #[cfg(not(target_arch = "wasm32"))]
    bevy::tasks::IoTaskPool::get()
        .spawn(async move {
            std::fs::File::create("assets/saved_scene.scn.ron")
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .unwrap();
        })
        .detach();
}

pub fn quit_game(mut exit: EventWriter<AppExit>) {
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
            ui::UiPlugin,
            hud::HudPlugin,
            cursor::CursorPlugin,
            world::WorldPlugin,
            loot::GroundLootPlugin,
            inventory::InventoryPlugin,
            player::PlayerPlugin,
            weapon::WeaponPlugin,
            projectile::ProjectilePlugin,
            interactables::InteractablesPlugin,
            debug::DebugPlugin,
        ))
        // update continuously even while unfocused (for networking)
        .insert_resource(bevy::winit::WinitSettings {
            focused_mode: bevy::winit::UpdateMode::Continuous,
            unfocused_mode: bevy::winit::UpdateMode::Continuous,
        })
        .init_state::<AppState>();

    app.add_systems(Startup, setup)
        .add_systems(OnEnter(AppState::LoadAssets), assets::load_assets)
        .add_systems(
            OnEnter(AppState::InGame),
            (init_ui, spawn_loot, spawn_player),
        )
        .add_systems(
            Update,
            (
                wait_for_window.run_if(in_state(AppState::Init)),
                wait_for_assets.run_if(in_state(AppState::LoadAssets)),
                save_scene.run_if(in_state(AppState::InGame)).run_if(
                    bevy::input::common_conditions::input_just_pressed(KeyCode::KeyF),
                ),
                quit_game.run_if(in_state(AppState::InGame)).run_if(
                    bevy::input::common_conditions::input_just_pressed(KeyCode::Escape),
                ),
            ),
        );

    app.run();
}
