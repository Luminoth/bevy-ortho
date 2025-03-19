mod bullet;
mod camera;
mod cursor;
mod data;
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
    color::palettes::css,
    prelude::*,
    window::{CursorGrabMode, PrimaryWindow},
};
use rand::prelude::*;

const DEFAULT_RESOLUTION: (f32, f32) = (1280.0, 720.0);
const VIEWPORT_HEIGHT: f32 = 20.0;
const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, 5.0, 5.0);
const WORLD_ROTATION: f32 = 45.0_f32.to_radians();

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

#[derive(Debug, Default)]
struct MeshMaterial {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

impl MeshMaterial {
    fn gen_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        (
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
        )
    }
}

#[derive(Debug, Default, Resource)]
pub struct GameAssets {
    pub player_model: Handle<Scene>,
    pub player_animation_graph: Handle<AnimationGraph>,

    weapon_mesh: MeshMaterial,
    ammo_mesh: MeshMaterial,
    throwable_mesh: MeshMaterial,
    consumable_mesh: MeshMaterial,
    bullet_mesh: MeshMaterial,

    floor_mesh: MeshMaterial,
    wall_mesh: MeshMaterial,
    box_mesh: MeshMaterial,
    crate_mesh: MeshMaterial,
}

impl GameAssets {
    fn load(
        &mut self,
        commands: &mut Commands,
        asset_server: &AssetServer,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        animation_graphs: &mut Assets<AnimationGraph>,
    ) {
        let (graph, node_indices) = AnimationGraph::from_clips([
            asset_server.load(GltfAssetLabel::Animation(0).from_asset(player::MODEL_PATH))
        ]);

        let graph_handle = animation_graphs.add(graph);
        commands.insert_resource(player::Animations {
            animations: node_indices,
            graph: graph_handle,
        });

        self.player_model =
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(player::MODEL_PATH));

        self.weapon_mesh.mesh = meshes.add(Capsule3d::new(
            inventory::WEAPON_RADIUS,
            inventory::WEAPON_LENGTH,
        ));
        self.weapon_mesh.material = materials.add(Color::from(css::DARK_RED));

        self.ammo_mesh.mesh = meshes.add(Cuboid::new(
            inventory::AMMO_LENGTH,
            inventory::AMMO_LENGTH,
            inventory::AMMO_LENGTH,
        ));
        self.ammo_mesh.material = materials.add(Color::from(css::GREEN_YELLOW));

        self.throwable_mesh.mesh = meshes.add(Sphere::new(inventory::THROWABLE_RADIUS));
        self.throwable_mesh.material = materials.add(Color::from(css::GREY));

        self.consumable_mesh.mesh = meshes.add(Sphere::new(inventory::CONSUMABLE_RADIUS));
        self.consumable_mesh.material = materials.add(Color::from(css::WHITE));

        self.bullet_mesh.mesh = meshes.add(Sphere::new(bullet::RADIUS));
        self.bullet_mesh.material = materials.add(Color::from(css::BLACK));

        self.floor_mesh.mesh = meshes.add(
            Plane3d::default()
                .mesh()
                .size(world::FLOOR_X_LENGTH, world::FLOOR_Z_LENGTH),
        );
        self.floor_mesh.material = materials.add(Color::srgb(0.3, 0.5, 0.3));

        self.wall_mesh.mesh = meshes.add(Cuboid::new(
            world::WALL_X_LENGTH,
            world::WALL_Y_LENGTH,
            world::WALL_Z_LENGTH,
        ));
        self.wall_mesh.material = materials.add(Color::srgb(0.4, 0.7, 0.3));

        self.box_mesh.mesh = meshes.add(Cuboid::new(
            world::BOX_X_LENGTH,
            world::BOX_Y_LENGTH,
            world::BOX_Z_LENGTH,
        ));
        self.box_mesh.material = materials.add(Color::srgb(0.8, 0.7, 0.6));

        self.crate_mesh.mesh = meshes.add(Cuboid::new(
            world::CRATE_X_LENGTH,
            world::CRATE_Y_LENGTH,
            world::CRATE_Z_LENGTH,
        ));
        self.crate_mesh.material = materials.add(Color::srgb(0.8, 0.7, 0.6));
    }

    pub fn gen_floor_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.floor_mesh.gen_components()
    }

    pub fn gen_wall_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.wall_mesh.gen_components()
    }

    pub fn gen_box_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.box_mesh.gen_components()
    }

    pub fn gen_crate_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.crate_mesh.gen_components()
    }

    pub fn gen_weapon_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.weapon_mesh.gen_components()
    }

    pub fn gen_ammo_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.ammo_mesh.gen_components()
    }

    pub fn gen_throwable_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.throwable_mesh.gen_components()
    }

    pub fn gen_consumable_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.consumable_mesh.gen_components()
    }

    pub fn gen_bullet_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.bullet_mesh.gen_components()
    }
}

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

fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
    mut app_state: ResMut<NextState<AppState>>,
) {
    let mut assets = GameAssets::default();
    assets.load(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &mut animation_graphs,
    );

    camera::spawn_main_camera(&mut commands, VIEWPORT_HEIGHT, CAMERA_OFFSET);

    world::spawn_world(
        &mut commands,
        &assets,
        Quat::from_axis_angle(Vec3::Y, WORLD_ROTATION),
    );

    commands.insert_resource(assets);

    app_state.set(AppState::InGame);
}

// TODO: this could be split up like
// spawn_player/s, spawn_loot, init_ui, etc
#[allow(clippy::too_many_arguments)]
fn enter_game(
    mut commands: Commands,
    game_assets: Res<GameAssets>,
    mut random: ResMut<RandomSource>,
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
    player::spawn_player(&mut commands, &game_assets, player_spawn);

    for loot_spawn in loot_spawn_query.iter() {
        loot::spawn_ground_loot(&mut commands, &game_assets, &mut random, loot_spawn);
    }
}

// TODO: put this in the debug plugin
fn save_scene(world: &World) {
    use std::io::Write;

    let scene = DynamicSceneBuilder::from_world(world)
        .deny_resource::<Time<Real>>()
        .deny_resource::<GameAssets>()
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
            std::fs::File::create(format!("assets/saved_scene.scn.ron"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .unwrap();
        })
        .detach();
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
            loot::GroundLootPlugin,
            player::PlayerPlugin,
            weapon::WeaponPlugin,
            bullet::BulletPlugin,
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
        .add_systems(Update, wait_for_window.run_if(in_state(AppState::Init)))
        .add_systems(OnEnter(AppState::LoadAssets), load_assets)
        .add_systems(OnEnter(AppState::InGame), enter_game)
        .add_systems(
            Update,
            (
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
