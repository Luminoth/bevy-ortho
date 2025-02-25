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
    pub look: Vec2,
    pub r#move: Vec2,
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
    input_state.look = Vec2::ZERO;
    input_state.r#move = Vec2::ZERO;
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

    let mut r#move = Vec2::default();
    if keys.pressed(KeyCode::KeyW) {
        r#move.y -= 1.0;
    }
    if keys.pressed(KeyCode::KeyS) {
        r#move.y += 1.0;
    }
    if keys.pressed(KeyCode::KeyA) {
        r#move.x -= 1.0;
    }
    if keys.pressed(KeyCode::KeyD) {
        r#move.x += 1.0;
    }

    input_state.r#move += r#move;

    let mut look = Vec2::default();
    for evt in evr_motion.read() {
        look += Vec2::new(
            evt.delta.x,
            /*if settings.mnk.invert_look { -1.0 } else { 1.0 } * -evt.delta.y,
            ) * settings.mnk.mouse_sensitivity*/
            evt.delta.y,
        ) * 2.0;
    }

    input_state.look += look;
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

    // left stick (move)
    if let (Some(x), Some(y)) = (
        gamepad.get(GamepadAxis::LeftStickX),
        gamepad.get(GamepadAxis::LeftStickY),
    ) {
        input_state.r#move += Vec2::new(-x, y);
    }

    // right stick (look)
    if let (Some(x), Some(y)) = (
        gamepad.get(GamepadAxis::RightStickX),
        gamepad.get(GamepadAxis::RightStickY),
    ) {
        input_state.look += Vec2::new(
            x,
            /*if settings.gamepad.invert_look {
                    -1.0
                } else {
                    1.0
                } * y,
            ) * settings.gamepad.look_sensitivity*/
            y,
        ) * 4.0;
    }
}
