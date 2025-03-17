use std::ops::Deref;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;

use crate::{
    AppState, GameCollisionLayers, PLAYER_INTERACT_LAYERS, camera, cursor, input, interactables,
    inventory,
};

#[derive(Debug, Resource)]
#[allow(dead_code)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(Debug, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct LocalPlayer;

#[derive(Debug, Component)]
pub struct PlayerModel;

// TODO: move to player data
const MOVE_SPEED: f32 = 8.0;
const HEIGHT: f32 = 2.0;
const MASS: f32 = 75.0;
const MODEL_PATH: &str = "human_1.glb";

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PlayerSet;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_player, (listen_interact, listen_weapon_select))
                .chain()
                .after(input::InputSet)
                .run_if(in_state(AppState::InGame))
                .in_set(PlayerSet),
        );
    }
}

fn update_player(
    input_state: Res<input::InputState>,
    mut player_query: Query<(&mut TnuaController, &GlobalTransform), With<LocalPlayer>>,
    cursor_query: Query<&Node, With<cursor::Cursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
) {
    if let Ok((mut character_controller, player_global_transform)) = player_query.get_single_mut() {
        let cursor_node = cursor_query.single();
        let (camera, camera_global_transform) = camera_query.single();

        let move_direction = Vec3::new(input_state.primary.x, 0.0, input_state.primary.y);

        let look_at = cursor::get_cursor_world_position(
            cursor_node,
            camera,
            camera_global_transform,
            player_global_transform,
        )
        .unwrap_or_default();

        let player_global_position = player_global_transform.translation();

        character_controller.basis(TnuaBuiltinWalk {
            desired_velocity: move_direction.normalize_or_zero() * MOVE_SPEED,
            desired_forward: Dir3::new(look_at - player_global_position).ok(),
            // TODO: this doesn't seem right by the docs / examples?
            float_height: HEIGHT * 0.75,
            ..Default::default()
        });
    }
}

fn listen_interact(
    mut evr_interact: EventReader<input::InteractInputEvent>,
    mut evw_interact: EventWriter<interactables::InteractEvent>,
    player_query: Query<&CollidingEntities, With<LocalPlayer>>,
    interactable_query: Query<(&interactables::Interactable, &Parent)>,
) {
    if evr_interact.is_empty() {
        return;
    }

    let colliding_entities = player_query.single();

    for entity in colliding_entities.iter() {
        let interactable = interactable_query
            .get(*entity)
            .or_else(|_| interactable_query.get(*entity));

        if let Ok((interactable, parent)) = interactable {
            let parent = parent.get();
            evw_interact.send(interactables::InteractEvent(parent, *interactable));
            break;
        }
    }

    evr_interact.clear();
}

fn listen_weapon_select(
    mut inventory: ResMut<inventory::Inventory>,
    mut evr_toggle_weapon: EventReader<input::ToggleWeaponInputEvent>,
    mut evr_select_weapon: EventReader<input::SelectWeaponInputEvent>,
) {
    if inventory.has_weapon() {
        if evr_select_weapon.is_empty() {
            if !evr_toggle_weapon.is_empty() {
                inventory.toggle_weapon();
            }
        } else {
            let selected = evr_select_weapon.read().next().unwrap();
            inventory.select_weapon(*selected.deref());
        }
    }

    evr_toggle_weapon.clear();
    evr_select_weapon.clear();
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    graphs: &mut Assets<AnimationGraph>,
    spawn_transform: &GlobalTransform,
) {
    let (graph, node_indices) = AnimationGraph::from_clips([
        asset_server.load(GltfAssetLabel::Animation(0).from_asset(MODEL_PATH))
    ]);

    let graph_handle = graphs.add(graph);
    commands.insert_resource(Animations {
        animations: node_indices,
        graph: graph_handle,
    });

    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset(MODEL_PATH));

    let mut commands = commands.spawn((
        spawn_transform.compute_transform(),
        CollidingEntities::default(),
        Name::new("Player"),
        Player,
        LocalPlayer,
    ));

    commands.insert((
        RigidBody::Dynamic,
        // TODO: why is the radius so small?
        Collider::capsule(HEIGHT * 0.25, HEIGHT),
        CollisionLayers::new(GameCollisionLayers::Player, PLAYER_INTERACT_LAYERS),
        Mass(MASS),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
    ));

    commands.insert((
        TnuaController::default(),
        bevy_tnua_avian3d::TnuaAvian3dSensorShape(Collider::cylinder(0.5, 0.0)),
    ));

    commands.with_children(|parent| {
        parent.spawn((
            // TODO: this is because our temp model is at 1.0 instead of 0.0
            // and rotated 180 degrees around the Y axis
            Transform::from_xyz(0.0, -1.0, 0.0)
                .with_rotation(Quat::from_axis_angle(Vec3::Y, 180.0_f32.to_radians())),
            SceneRoot(model),
            Name::new("Model"),
            PlayerModel,
        ));
    });
}
