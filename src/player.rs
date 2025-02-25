use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Debug)]
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, _app: &mut App) {
        /*app.add_systems(
            Update,
            update_camera.after(input::InputSet), //.run_if(in_state(AppState::InGame)),
        );*/
    }
}

pub fn spawn_player(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
    _look_at: Vec3,
) {
    commands.spawn((
        Mesh3d(meshes.add(Capsule3d::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.7, 0.5))),
        Transform::from_translation(position),
        Name::new("Player"),
        Player,
    ));
}
