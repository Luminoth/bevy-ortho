use bevy::{prelude::*, render::camera::ScalingMode};

use crate::input;

#[derive(Component)]
pub struct MainCamera;

const MOVE_SPEED: f32 = 5.0;

#[derive(Debug)]
pub struct OrthoCameraPlugin;

impl Plugin for OrthoCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_camera.after(input::InputSet), //.run_if(in_state(AppState::InGame)),
        );
    }
}

fn update_camera(
    time: Res<Time>,
    input_state: Res<input::InputState>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(mut camera_transform) = camera_query.get_single_mut() {
        // TODO: temp stuff, what we wanna do is move the player character
        // and have the camera follow them

        let movement =
            camera_transform.rotation * Vec3::new(input_state.r#move.x, 0.0, input_state.r#move.y);

        let y = camera_transform.translation.y;
        camera_transform.translation += movement * time.delta_secs() * MOVE_SPEED;
        camera_transform.translation.y = y;
    }
}

pub fn spawn_main_camera(commands: &mut Commands, position: Vec3, look_at: Vec3) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(position).looking_at(look_at, Vec3::Y),
        MainCamera,
    ));
}
