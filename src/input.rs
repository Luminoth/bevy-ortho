use bevy::{
    input::{
        gamepad::{GamepadConnection, GamepadConnectionEvent},
        mouse::MouseMotion,
    },
    prelude::*,
};

#[derive(Debug, Resource)]
struct ConnectedGamepad(Entity);

#[derive(Debug, Default, Copy, Clone, Resource, Reflect)]
pub struct InputState {
    pub primary: Vec2,
    pub secondary: Vec2,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemSet)]
pub struct InputSet;

#[derive(Debug)]
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_gamepad_events,
                (update_mnk, (update_gamepad.after(handle_gamepad_events))),
            )
                .in_set(InputSet),
        )
        .add_systems(PostUpdate, clear_input)
        .init_resource::<InputState>();
    }
}

fn clear_input(mut input_state: ResMut<InputState>) {
    input_state.primary = Vec2::ZERO;
    input_state.secondary = Vec2::ZERO;
}

fn handle_gamepad_events(
    mut commands: Commands,
    gamepad: Option<Res<ConnectedGamepad>>,
    mut evr_connections: EventReader<GamepadConnectionEvent>,
) {
    for evt_conn in evr_connections.read() {
        match &evt_conn.connection {
            GamepadConnection::Connected { name, .. } => {
                info!("gamepad connected: {:?}, name: {}", evt_conn.gamepad, name,);

                if gamepad.is_none() {
                    info!("using gamepad {:?}", evt_conn.gamepad);
                    commands.insert_resource(ConnectedGamepad(evt_conn.gamepad));
                }
            }
            GamepadConnection::Disconnected => {
                warn!("gamepad disconnected: {:?}", evt_conn.gamepad);

                if let Some(ConnectedGamepad(gamepad)) = gamepad.as_deref() {
                    if *gamepad == evt_conn.gamepad {
                        commands.remove_resource::<ConnectedGamepad>();
                    }
                }
            }
        }
    }
}

fn update_mnk(
    keys: Res<ButtonInput<KeyCode>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut input_state: ResMut<InputState>,
    //settings: Res<Settings>,
) {
    /*if !settings.mnk.enabled {
        return;
    }*/

    let mut primary = Vec2::default();
    if keys.pressed(KeyCode::KeyW) {
        primary.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        primary.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        primary.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        primary.x += 1.0;
    }

    input_state.primary += primary;

    let mut secondary = Vec2::default();
    for evt in evr_motion.read() {
        secondary += Vec2::new(
            evt.delta.x,
            evt.delta.y, //if settings.mnk.invert_look { -1.0 } else { 1.0 } * evt.delta.y,
        );
    }

    input_state.secondary += secondary; // * settings.mnk.mouse_sensitivity;
}

fn update_gamepad(
    //settings: Res<Settings>,
    gamepad: Option<Res<ConnectedGamepad>>,
    mut input_state: ResMut<InputState>,
    gamepads: Query<&Gamepad>,
) {
    /*if !settings.gamepad.enabled {
        return;
    }*/

    let Some(&ConnectedGamepad(gamepad)) = gamepad.as_deref() else {
        return;
    };

    let gamepad = gamepads.get(gamepad).unwrap();

    // left stick (primary, usually move)
    if let (Some(x), Some(y)) = (
        gamepad.get(GamepadAxis::LeftStickX),
        gamepad.get(GamepadAxis::LeftStickY),
    ) {
        input_state.primary += Vec2::new(x, -y);
    }

    // right stick (secondary, usually look / cursor)
    if let (Some(x), Some(y)) = (
        gamepad.get(GamepadAxis::RightStickX),
        gamepad.get(GamepadAxis::RightStickY),
    ) {
        input_state.secondary += Vec2::new(
            x, y, //if settings.gamepad.invert_look { -1.0 } else { 1.0 } * y,
        ) * 4.0; // * settings.gamepad.look_sensitivity
    }
}
