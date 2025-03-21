use std::ops::Deref;

use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;

use crate::{
    AppState, GameCollisionLayers, PLAYER_INTERACT_LAYERS, assets, camera, cursor, data, input,
    interactables, inventory,
};

#[derive(Debug, Resource)]
#[allow(dead_code)]
pub struct Animations {
    pub animations: Vec<AnimationNodeIndex>,
    pub graph: Handle<AnimationGraph>,
}

#[derive(Debug, Component)]
pub struct Player {
    toggle_select_timer: Timer,
    weapon_select_timer: (inventory::WeaponSlot, Timer),
}

impl Player {
    fn new() -> Self {
        let mut this = Self {
            toggle_select_timer: Timer::from_seconds(0.5, TimerMode::Once),
            weapon_select_timer: (
                inventory::WeaponSlot::default(),
                Timer::from_seconds(0.5, TimerMode::Once),
            ),
        };

        this.toggle_select_timer.pause();
        this.weapon_select_timer.1.pause();

        this
    }
}

#[derive(Debug, Component)]
pub struct LocalPlayer;

#[derive(Debug, Component)]
pub struct PlayerModel;

// TODO: move to player data
pub const MODEL_PATH: &str = "human_1.glb";
const MOVE_SPEED: f32 = 8.0;
const HEIGHT: f32 = 2.0;
const MASS: f32 = 75.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct PlayerSet;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (move_player, (handle_weapon_select_input, handle_firing))
                .chain()
                .after(input::InputSet)
                .run_if(in_state(AppState::InGame))
                .in_set(PlayerSet),
        )
        .add_systems(
            Update,
            update_player
                .run_if(in_state(AppState::InGame))
                .in_set(PlayerSet),
        )
        .add_systems(PostProcessCollisions, handle_interact_input);
    }
}

fn update_player(
    time: Res<Time>,
    mut inventory: ResMut<inventory::Inventory>,
    mut player_query: Query<&mut Player>,
) {
    for mut player in player_query.iter_mut() {
        player.toggle_select_timer.tick(time.delta());
        if player.toggle_select_timer.just_finished() {
            info!("toggle weapon");
            inventory.toggle_selected_weapon();
        }

        player.weapon_select_timer.1.tick(time.delta());
        if player.weapon_select_timer.1.just_finished() {
            info!("select weapon {}", player.weapon_select_timer.0);
            inventory.set_selected_weapon(player.weapon_select_timer.0);
        }
    }
}

fn move_player(
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

fn handle_interact_input(
    mut commands: Commands,
    mut evr_interact: EventReader<input::InteractInputEvent>,
    player_query: Query<&CollidingEntities, With<LocalPlayer>>,
    interactable_query: Query<(&interactables::InteractableType, &Parent)>,
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
            commands.trigger(interactables::InteractEvent {
                target: parent,
                target_type: *interactable,
            });
            break;
        }
    }

    evr_interact.clear();
}

fn handle_weapon_select_input(
    inventory: Res<inventory::Inventory>,
    mut evr_toggle_weapon: EventReader<input::ToggleWeaponInputEvent>,
    mut evr_select_weapon: EventReader<input::SelectWeaponInputEvent>,
    mut player_query: Query<&mut Player, With<LocalPlayer>>,
) {
    // TODO: we can't select / toggle empty weapon slots
    if inventory.has_weapon() {
        let mut player = player_query.single_mut();
        if (!player.toggle_select_timer.paused() && !player.toggle_select_timer.finished())
            || (!player.weapon_select_timer.1.paused() && !player.weapon_select_timer.1.finished())
        {
            return;
        }

        if evr_select_weapon.is_empty() {
            if !evr_toggle_weapon.is_empty() {
                // TODO: this would come from the animation
                player.toggle_select_timer.reset();
                player.toggle_select_timer.unpause();
            }
        } else {
            let selected = evr_select_weapon.read().next().unwrap();
            let weapon_slot = *selected.deref();

            player.weapon_select_timer.0 = weapon_slot;

            // TODO: this would come from the animation
            player.weapon_select_timer.1.reset();
            player.weapon_select_timer.1.unpause();
        }
    }

    evr_toggle_weapon.clear();
    evr_select_weapon.clear();
}

fn handle_firing(
    mut commands: Commands,
    input_state: Res<input::InputState>,
    mut inventory: ResMut<inventory::Inventory>,
    datum: Res<data::WeaponDataSource>,
    time: Res<Time>,
    player_query: Query<(Entity, &GlobalTransform), With<LocalPlayer>>,
) {
    if !input_state.firing {
        return;
    }

    let weapon = inventory.get_selected_weapon_item_mut();
    if let Some(weapon) = weapon {
        let (entity, global_transform) = player_query.single();

        let mut origin = global_transform.compute_transform();
        origin.translation.y = 1.5;
        // TODO: we might want to spawn this in front of the player as well
        // (currently it spawns inside the player and we filter the collision)

        // TODO: if we can do this with an event / trigger
        // it might be cleaner than calling a function
        weapon.fire(&mut commands, entity, &datum, &time, &origin);
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    game_assets: &assets::GameAssets,
    spawn_transform: &GlobalTransform,
) {
    let mut commands = commands.spawn((
        spawn_transform.compute_transform(),
        Visibility::default(),
        CollidingEntities::default(),
        Name::new("Player"),
        Player::new(),
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
            SceneRoot(game_assets.player_model.clone()),
            Name::new("Model"),
            PlayerModel,
        ));
    });
}
