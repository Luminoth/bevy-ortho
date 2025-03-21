use bevy::{color::palettes::css, prelude::*, window::PrimaryWindow};

use crate::{AppState, input, player, ui};

#[derive(Debug, Component)]
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

    if let Ok(window) = window_query.get_single() {
        let movement = Vec2::new(input_state.secondary.x, input_state.secondary.y);

        if let Val::Px(left) = &mut cursor_node.left {
            *left = (*left + movement.x).clamp(0.0, window.width() - 1.0);
        }

        if let Val::Px(top) = &mut cursor_node.top {
            *top = (*top + movement.y).clamp(0.0, window.height() - 1.0);
        }
    }
}

pub fn spawn_cursor(commands: &mut Commands, position: Vec2) {
    ui::spawn_panel_at(
        commands,
        (Val::Px(position.x), Val::Px(position.y)),
        (Val::Auto, Val::Auto),
        "Cursor",
    )
    .insert(Cursor)
    .with_children(|parent| {
        // top
        ui::spawn_image_at(
            parent,
            (Val::Auto, Val::Px(-15.0)),
            (Val::Px(1.0), Val::Px(20.0)),
            ImageNode::solid_color(css::RED.into()),
        );

        // bottom
        ui::spawn_image_at(
            parent,
            (Val::Auto, Val::Px(15.0)),
            (Val::Px(1.0), Val::Px(20.0)),
            ImageNode::solid_color(css::GREEN.into()),
        );

        // left
        ui::spawn_image_at(
            parent,
            (Val::Px(-15.0), Val::Auto),
            (Val::Px(20.0), Val::Px(1.0)),
            ImageNode::solid_color(css::BLUE.into()),
        );

        // right
        ui::spawn_image_at(
            parent,
            (Val::Px(15.0), Val::Auto),
            (Val::Px(20.0), Val::Px(1.0)),
            ImageNode::solid_color(css::YELLOW.into()),
        );
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
