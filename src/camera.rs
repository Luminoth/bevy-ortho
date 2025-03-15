use bevy::{prelude::*, render::camera::ScalingMode};

use crate::{AppState, player};

#[derive(Component)]
pub struct MainCamera(Vec3);

#[derive(Debug)]
pub struct OrthoCameraPlugin;

impl Plugin for OrthoCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_camera
                .after(player::PlayerSet)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn update_camera(
    mut camera_query: Query<(&mut Transform, &MainCamera)>,
    player_query: Query<&Transform, (With<player::LocalPlayer>, Without<MainCamera>)>,
) {
    if let Ok((mut camera_transform, camera)) = camera_query.get_single_mut() {
        if let Ok(player_transform) = player_query.get_single() {
            // TODO: this should have a deadzone and all that to feel better
            camera_transform.translation = player_transform.translation + camera.0;
        }
    }
}

pub fn spawn_main_camera(commands: &mut Commands, viewport_height: f32, offset: Vec3) {
    commands.spawn((
        Camera3d::default(),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical { viewport_height },
            ..OrthographicProjection::default_3d()
        }),
        Transform::from_translation(offset).looking_at(Vec3::ZERO, Vec3::Y),
        Name::new("Main camera"),
        MainCamera(offset),
    ));
}
