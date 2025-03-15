use bevy::{prelude::*, window::PrimaryWindow};

use crate::{AppState, input, player};

#[derive(Component)]
pub struct Cursor;

#[derive(Debug)]
pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_cursor
                .after(player::PlayerSet)
                .run_if(in_state(AppState::InGame)),
        );
    }
}

fn update_cursor(
    input_state: Res<input::InputState>,
    mut cursor_query: Query<&mut Node, With<Cursor>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let mut cursor_node = cursor_query.single_mut();
    let window = window_query.single();

    let movement = Vec2::new(input_state.secondary.x, input_state.secondary.y);

    if let Val::Px(left) = &mut cursor_node.left {
        *left = (*left + movement.x).clamp(0.0, window.width() - 1.0);
    }

    if let Val::Px(top) = &mut cursor_node.top {
        *top = (*top + movement.y).clamp(0.0, window.height() - 1.0);
    }
}

pub fn spawn_cursor(commands: &mut Commands, position: Vec2) {
    let mut commands = commands.spawn((
        Node {
            left: Val::Px(position.x),
            top: Val::Px(position.y),
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            ..default()
        },
        Name::new("Cursor"),
        Cursor,
    ));

    commands.with_children(|parent| {
        // top
        parent.spawn((
            Node {
                top: Val::Px(-15.0),
                width: Val::Px(1.0),
                height: Val::Px(20.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode::solid_color(Color::srgb(1.0, 0.0, 0.0)),
        ));

        // bottom
        parent.spawn((
            Node {
                top: Val::Px(15.0),
                width: Val::Px(1.0),
                height: Val::Px(20.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode::solid_color(Color::srgb(0.0, 1.0, 0.0)),
        ));

        // left
        parent.spawn((
            Node {
                left: Val::Px(-15.0),
                width: Val::Px(20.0),
                height: Val::Px(1.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode::solid_color(Color::srgb(0.0, 0.0, 1.0)),
        ));

        // right
        parent.spawn((
            Node {
                left: Val::Px(15.0),
                width: Val::Px(20.0),
                height: Val::Px(1.0),
                position_type: PositionType::Absolute,
                ..default()
            },
            ImageNode::solid_color(Color::srgb(0.0, 1.0, 1.0)),
        ));
    });
}

pub fn get_cursor_viewport_position(cursor_node: &Node) -> Option<Vec2> {
    let mut cursor_viewport_position = Vec2::default();

    if let Val::Px(left) = cursor_node.left {
        cursor_viewport_position.x = left;
    } else {
        return None;
    }

    if let Val::Px(top) = cursor_node.top {
        cursor_viewport_position.y = top;
    } else {
        return None;
    }

    Some(cursor_viewport_position)
}

pub fn get_cursor_world_position(
    cursor_node: &Node,
    camera: &Camera,
    camera_global_transform: &GlobalTransform,
    player_global_transform: &GlobalTransform,
) -> Option<Vec3> {
    let plane_origin = player_global_transform.translation();
    let plane = InfinitePlane3d::new(player_global_transform.up());

    let cursor_viewport_position = get_cursor_viewport_position(cursor_node).unwrap_or_default();

    let ray = camera
        .viewport_to_world(camera_global_transform, cursor_viewport_position)
        .ok()?;

    let distance = ray.intersect_plane(plane_origin, plane)?;

    Some(ray.get_point(distance))
}
