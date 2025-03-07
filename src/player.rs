use avian3d::prelude::*;
use bevy::prelude::*;
use bevy_tnua::prelude::*;

use crate::input;

#[derive(Resource)]
#[allow(dead_code)]
struct Animations {
    animations: Vec<AnimationNodeIndex>,
    graph: Handle<AnimationGraph>,
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerModel;

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
            (
                move_player.after(input::InputSet), //.run_if(in_state(AppState::InGame)),
            )
                .in_set(PlayerSet),
        );
    }
}

pub fn move_player(
    input_state: Res<input::InputState>,
    mut player_query: Query<&mut TnuaController, With<Player>>,
) {
    if let Ok(mut character_controller) = player_query.get_single_mut() {
        let direction = Vec3::new(input_state.primary.x, 0.0, input_state.primary.y);

        character_controller.basis(TnuaBuiltinWalk {
            desired_velocity: direction.normalize_or_zero() * MOVE_SPEED,
            desired_forward: Dir3::new(Vec3::new(
                -input_state.secondary.x,
                0.0,
                input_state.secondary.y,
            ))
            .ok(),
            // TODO: this doesn't seem right by the docs / examples?
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
    graphs: &mut Assets<AnimationGraph>,
    position: Vec3,
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
        Transform::from_translation(position),
        Name::new("Player"),
        Player,
    ));

    commands.insert((
        RigidBody::Dynamic,
        // TODO: why is the radius so small?
        Collider::capsule(HEIGHT * 0.25, HEIGHT),
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
            Transform::from_xyz(0.0, -1.0, 0.0),
            SceneRoot(model),
            Name::new("Model"),
            PlayerModel,
        ));
    });
}
