use bevy::{prelude::*, render::camera::ScalingMode};

#[derive(Component)]
pub struct MainCamera;

#[derive(Debug)]
pub struct OrthoCameraPlugin;

impl Plugin for OrthoCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_camera, //.after(input::InputSet)
                           //.run_if(in_state(AppState::InGame)),
        );
    }
}

fn update_camera(
    _time: Res<Time>,
    //input_state: Res<InputState>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(mut _camera_transform) = camera_query.get_single_mut() {
        // TODO: move the camera
    }
}

pub fn spawn_camera(commands: &mut Commands, position: Vec3, look_at: Vec3) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.0,
            },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(position).looking_at(look_at, Vec3::Y),
    ));
}
