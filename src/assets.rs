use bevy::prelude::*;

use crate::{camera, loot, player, projectile, world};

const VIEWPORT_HEIGHT: f32 = 20.0;
const CAMERA_OFFSET: Vec3 = Vec3::new(
    // TODO: this would be cool but it does mean rotating all the player input
    0.0, //-VIEWPORT_HEIGHT * 0.5,
    VIEWPORT_HEIGHT * 0.5,
    VIEWPORT_HEIGHT * 0.5,
);

#[derive(Debug, Default)]
pub struct MeshMaterial {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
}

impl MeshMaterial {
    fn gen_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        (
            Mesh3d(self.mesh.clone()),
            MeshMaterial3d(self.material.clone()),
        )
    }
}

#[derive(Debug, Default, Resource)]
pub struct GameAssets {
    pub player_model: Handle<Scene>,
    pub _player_animation_graph: Handle<AnimationGraph>,

    weapon_mesh: MeshMaterial,
    ammo_mesh: MeshMaterial,
    throwable_mesh: MeshMaterial,
    consumable_mesh: MeshMaterial,
    bullet_mesh: MeshMaterial,

    floor_mesh: MeshMaterial,
    wall_mesh: MeshMaterial,
    box_mesh: MeshMaterial,
    crate_mesh: MeshMaterial,
}

impl GameAssets {
    fn load(
        &mut self,
        commands: &mut Commands,
        asset_server: &AssetServer,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<StandardMaterial>,
        animation_graphs: &mut Assets<AnimationGraph>,
    ) {
        // world
        self.floor_mesh = world::load_floor_assets(meshes, materials);
        self.wall_mesh = world::load_wall_assets(meshes, materials);
        self.box_mesh = world::load_box_assets(meshes, materials);
        self.crate_mesh = world::load_crate_assets(meshes, materials);

        // player
        self.player_model = player::load_player_assets(commands, asset_server, animation_graphs);

        // loot
        self.weapon_mesh = loot::load_weapon_assets(meshes, materials);
        self.ammo_mesh = loot::load_ammo_assets(meshes, materials);
        self.throwable_mesh = loot::load_throwable_assets(meshes, materials);
        self.consumable_mesh = loot::load_consumable_assets(meshes, materials);

        // projectiles
        self.bullet_mesh = projectile::load_bullet_assets(meshes, materials);
    }

    pub fn gen_floor_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.floor_mesh.gen_components()
    }

    pub fn gen_wall_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.wall_mesh.gen_components()
    }

    pub fn gen_box_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.box_mesh.gen_components()
    }

    pub fn gen_crate_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.crate_mesh.gen_components()
    }

    pub fn gen_weapon_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.weapon_mesh.gen_components()
    }

    pub fn gen_ammo_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.ammo_mesh.gen_components()
    }

    pub fn gen_throwable_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.throwable_mesh.gen_components()
    }

    pub fn gen_consumable_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.consumable_mesh.gen_components()
    }

    pub fn gen_bullet_mesh_components(&self) -> (Mesh3d, MeshMaterial3d<StandardMaterial>) {
        self.bullet_mesh.gen_components()
    }
}

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut animation_graphs: ResMut<Assets<AnimationGraph>>,
) {
    let mut assets = GameAssets::default();
    assets.load(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        &mut animation_graphs,
    );

    // these would be part of the scene asset
    {
        camera::spawn_main_camera(&mut commands, VIEWPORT_HEIGHT, CAMERA_OFFSET);

        world::spawn_world(&mut commands, &assets);
    }

    commands.insert_resource(assets);
}
