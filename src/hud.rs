use bevy::prelude::*;

use crate::{AppState, inventory, ui};

#[derive(Debug, Component)]
pub struct Hud;

#[derive(Debug, Component)]
struct PrimaryWeaponLabel;

#[derive(Debug, Component)]
struct SecondaryWeaponLabel;

#[derive(Debug, Component)]
struct InventoryLabel;

#[derive(Debug)]
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_hud.run_if(in_state(AppState::InGame)));
    }
}

// TODO: updating this stuff every frame isn't great
// especially since we're allocating every frame
#[allow(clippy::type_complexity)]
fn update_hud(
    inventory: Res<inventory::Inventory>,
    mut label_set: ParamSet<(
        Query<&mut Text, With<PrimaryWeaponLabel>>,
        Query<&mut Text, With<SecondaryWeaponLabel>>,
        Query<&mut Text, With<InventoryLabel>>,
    )>,
) {
    label_set.p0().single_mut().0 = format!(
        "Primary Weapon: {:?}",
        inventory
            .get_primary_weapon()
            .map(|weapon| (weapon.r#type, weapon.ammo_count))
    );

    label_set.p1().single_mut().0 = format!(
        "Secondary Weapon: {:?}",
        inventory
            .get_secondary_weapon()
            .map(|weapon| (weapon.r#type, weapon.ammo_count))
    );

    let mut inventory_text = String::new();
    for item in inventory.get_items() {
        inventory_text.push_str(
            match item {
                inventory::InventoryItem::Ammo(ammo_type, ammo_count) => {
                    format!("{}: {}\n", ammo_type, ammo_count)
                }
                _ => format!("{}\n", item),
            }
            .as_ref(),
        )
    }
    label_set.p2().single_mut().0 = inventory_text;
}

pub fn spawn_hud(commands: &mut Commands) {
    ui::spawn_canvas(commands, "HUD", false)
        .insert(Hud)
        .with_children(|parent| {
            ui::spawn_vbox_at(parent, (Val::Px(0.0), Val::Px(0.0))).with_children(|parent| {
                ui::spawn_label(parent, "Primary Weapon: None").insert(PrimaryWeaponLabel);
                ui::spawn_label(parent, "Secondary Weapon: None").insert(SecondaryWeaponLabel);

                ui::spawn_label(parent, "Inventory:");
                ui::spawn_label(parent, "").insert(InventoryLabel);
            });
        });
}
