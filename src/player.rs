use avian3d::prelude::*;
use bevy::{prelude::*, scene::SceneInstance};
use bevy_tnua::prelude::*;

use crate::input;

#[derive(Component)]
pub struct Player;

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
            (
                fix_player_model,
                move_player.after(input::InputSet), //.run_if(in_state(AppState::InGame)),
            )
                .in_set(PlayerSet),
        );
    }
}

fn fix_player_model(
    player_query: Query<&Children, (With<Player>, Added<SceneInstance>)>,
    mut transform_query: Query<&mut Transform>,
) {
    for children in player_query.iter() {
        info!("checking children");
        for child in children {
            info!("checking child");
            if let Ok(mut child_transform) = transform_query.get_mut(*child) {
                info!("fixing child");
                child_transform.translation.y = -1.0;
            }
        }
    }
}

// TODO: rotate the player

pub fn move_player(
    input_state: Res<input::InputState>,
    mut player_query: Query<(&mut TnuaController, &GlobalTransform), With<Player>>,
) {
    if let Ok((mut character_controller, global_transform)) = player_query.get_single_mut() {
        let global_transform = global_transform.compute_transform();

        let direction =
            global_transform.rotation * Vec3::new(input_state.r#move.x, 0.0, input_state.r#move.y);

        character_controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction.normalize_or_zero() * MOVE_SPEED,
            // TODO: this isn't right, but we should probably do this instead of rotate_player()
            //desired_forward: Dir3::new(Vec3::new(last_input.input_state.look.x, 0.0, 0.0)).ok(),
            desired_forward: Some(-Dir3::Z),
            // TODO: this doesn't seem right by the docs, but anything less doesn't work
            float_height: HEIGHT * 0.75,
            ..Default::default()
        });
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    asset_server: &AssetServer,
    _meshes: &mut Assets<Mesh>,
    _materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    let model = asset_server.load(GltfAssetLabel::Scene(0).from_asset("human_1.glb"));

    let mut commands = commands.spawn((
        /*Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.7, 0.5))),*/
        SceneRoot(model),
        Transform::from_translation(position),
        Name::new("Player"),
        Player,
    ));

    commands.insert((
        RigidBody::Dynamic,
        // TODO: can we infer this from the mesh?
        Collider::capsule(HEIGHT * 0.25, HEIGHT),
        Mass(MASS),
        LockedAxes::ROTATION_LOCKED.unlock_rotation_y(),
    ));

    commands.insert((
        TnuaController::default(),
        bevy_tnua_avian3d::TnuaAvian3dSensorShape(Collider::cylinder(0.5, 0.0)),
    ));
}
