use bevy::prelude::*;

use crate::ui;

#[derive(Debug, Component)]
pub struct Hud;

#[derive(Debug)]
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, _app: &mut App) {}
}

pub fn spawn_hud(commands: &mut Commands) {
    ui::spawn_canvas(commands, "HUD", false)
        .insert(Hud)
        .with_children(|parent| {
            ui::spawn_label_at(parent, (Val::Px(0.0), Val::Px(0.0)), "Test HUD Text");
        });
}
