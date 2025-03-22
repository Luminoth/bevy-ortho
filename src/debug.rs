use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    window::PrimaryWindow,
};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui};

use crate::{camera, cursor, player};

#[derive(Debug, Default, Reflect, Resource)]
pub struct DebugSettings {
    pub show_debug_ui: bool,
    pub show_world_inspector: bool,
}

fn show_debug_ui(debug_settings: Res<DebugSettings>) -> bool {
    debug_settings.show_debug_ui
}

fn show_world_inspector(debug_settings: Res<DebugSettings>) -> bool {
    debug_settings.show_debug_ui && debug_settings.show_world_inspector
}

#[derive(Debug, Default)]
pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            // diagnostics
            //bevy::diagnostic::LogDiagnosticsPlugin::default(),
            bevy::diagnostic::FrameTimeDiagnosticsPlugin,
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            //bevy::render::diagnostic::RenderDiagnosticsPlugin,
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
            // inspectors
            // TODO: why does the world inspector not pick up custom resource types?
            // using register_type() on them doesn't seem to fix it
            // TODO: might have outgrown the quick plugins: https://docs.rs/bevy-inspector-egui/0.25.2/bevy_inspector_egui/#use-case-2-manual-ui
            bevy_inspector_egui::quick::WorldInspectorPlugin::default()
                .run_if(show_world_inspector),
        ))
        .add_systems(
            Update,
            (
                listen_input,
                (debug_ui, game_debug_ui).chain().run_if(show_debug_ui),
            ),
        )
        .init_resource::<DebugSettings>();
    }
}

fn listen_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut debug_settings: ResMut<DebugSettings>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    if keys.just_pressed(KeyCode::Backquote) {
        debug_settings.show_debug_ui = !debug_settings.show_debug_ui;
        crate::show_cursor(&mut window_query.single_mut(), debug_settings.show_debug_ui);
    }
}

fn debug_ui(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut debug_settings: ResMut<DebugSettings>,
    mut contexts: EguiContexts,
    gamepads: Query<(&Name, &Gamepad)>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label(format!(
                "{:.1} avg fps, {:.3} avg ms/frame",
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FPS)
                    .and_then(|fps| fps.smoothed())
                    .unwrap_or_default(),
                diagnostics
                    .get(&FrameTimeDiagnosticsPlugin::FRAME_TIME)
                    .and_then(|frame_time| frame_time.smoothed())
                    .unwrap_or_else(|| time.delta_secs_f64())
            ));
            ui.label(format!(
                "{:.2}% avg cpu, {:.2}% memory",
                diagnostics
                    .get(&SystemInformationDiagnosticsPlugin::CPU_USAGE)
                    .and_then(|cpu| cpu.smoothed())
                    .unwrap_or_default(),
                diagnostics
                    .get(&SystemInformationDiagnosticsPlugin::MEM_USAGE)
                    .and_then(|memory| memory.value())
                    .unwrap_or_default()
            ));

            ui.label("Gamepads:");
            for (name, gamepad) in &gamepads {
                ui.label(format!(
                    "  {}:{} {}",
                    gamepad.vendor_id().unwrap_or_default(),
                    gamepad.product_id().unwrap_or_default(),
                    name
                ));
            }

            ui.label(format!(
                "{} entities",
                diagnostics
                    .get(&EntityCountDiagnosticsPlugin::ENTITY_COUNT)
                    .and_then(|count| count.value())
                    .unwrap_or_default()
            ));

            if ui.button("World Inspector").clicked() {
                debug_settings.show_world_inspector = !debug_settings.show_world_inspector;
            }
        });
    });
}

fn game_debug_ui(
    mut contexts: EguiContexts,
    player_query: Query<&GlobalTransform, With<player::LocalPlayer>>,
    cursor_query: Query<&Node, With<cursor::Cursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<camera::MainCamera>>,
) {
    let player_global_transform = player_query.single();
    let cursor_node = cursor_query.single();
    let (camera, camera_global_transform) = camera_query.single();

    let cursor_viewport_position =
        cursor::get_cursor_viewport_position(cursor_node).unwrap_or_default();
    let cursor_world_position = cursor::get_cursor_world_position(
        cursor_node,
        camera,
        camera_global_transform,
        player_global_transform,
    )
    .unwrap_or_default();
    let player_global_translation = player_global_transform.translation();

    egui::Window::new("Game Debug").show(contexts.ctx_mut(), |ui| {
        ui.vertical(|ui| {
            ui.label(format!(
                "Cursor viewport position: {}",
                cursor_viewport_position
            ));
            ui.label(format!("Cursor world position: {}", cursor_world_position));

            ui.label(format!("Player position: {}", player_global_translation));
        });
    });
}
