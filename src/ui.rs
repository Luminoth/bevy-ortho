use bevy::prelude::*;

pub const PICKING_BEHAVIOR_BLOCKING: PickingBehavior = PickingBehavior {
    should_block_lower: true,
    is_hoverable: false,
};

#[derive(Debug)]
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn spawn_panel_at<'a>(
    commands: &'a mut Commands,
    position: (Val, Val),
    size: (Val, Val),
    name: impl AsRef<str>,
) -> EntityCommands<'a> {
    commands.spawn((
        Node {
            left: position.0,
            top: position.1,
            width: size.0,
            height: size.1,
            ..default()
        },
        Name::new(format!("Ui Panel - {}", name.as_ref())),
    ))
}

pub fn spawn_canvas<'a>(
    commands: &'a mut Commands,
    name: impl AsRef<str>,
    blocking: bool,
) -> EntityCommands<'a> {
    let mut commands = commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        Name::new(format!("Ui Canvas - {}", name.as_ref())),
    ));

    if blocking {
        commands.insert(PICKING_BEHAVIOR_BLOCKING);
    }

    commands
}

pub fn spawn_label_at<'a>(
    parent: &'a mut ChildBuilder,
    position: (Val, Val),
    text: impl Into<String>,
) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            left: position.0,
            top: position.1,
            position_type: PositionType::Absolute,
            ..default()
        },
        Text::new(text),
        PickingBehavior::IGNORE,
    ))
}

pub fn spawn_image_at<'a>(
    parent: &'a mut ChildBuilder,
    position: (Val, Val),
    size: (Val, Val),
    image: ImageNode,
) -> EntityCommands<'a> {
    parent.spawn((
        Node {
            left: position.0,
            top: position.1,
            width: size.0,
            height: size.1,
            position_type: PositionType::Absolute,
            ..default()
        },
        image,
    ))
}
