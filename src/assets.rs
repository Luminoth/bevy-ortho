use bevy::{color::palettes::css, prelude::*};

use crate::{camera, inventory, player, projectile, world};

const VIEWPORT_HEIGHT: f32 = 20.0;
const CAMERA_OFFSET: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT * 0.5, VIEWPORT_HEIGHT * 0.5);

#[derive(Debug, Default)]
struct MeshMaterial {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
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
        let (graph, node_indices) = AnimationGraph::from_clips([
            asset_server.load(GltfAssetLabel::Animation(0).from_asset(player::MODEL_PATH))
        ]);

        let graph_handle = animation_graphs.add(graph);
        commands.insert_resource(player::Animations {
            animations: node_indices,
            graph: graph_handle,
        });

        self.player_model =
            asset_server.load(GltfAssetLabel::Scene(0).from_asset(player::MODEL_PATH));

        self.weapon_mesh.mesh = meshes.add(Capsule3d::new(
            inventory::WEAPON_RADIUS,
            inventory::WEAPON_LENGTH,
        ));
        self.weapon_mesh.material = materials.add(Color::from(css::DARK_RED));

        self.ammo_mesh.mesh = meshes.add(Cuboid::new(
            inventory::AMMO_LENGTH,
            inventory::AMMO_LENGTH,
            inventory::AMMO_LENGTH,
        ));
        self.ammo_mesh.material = materials.add(Color::from(css::GREEN_YELLOW));

        self.throwable_mesh.mesh = meshes.add(Sphere::new(inventory::THROWABLE_RADIUS));
        self.throwable_mesh.material = materials.add(Color::from(css::GREY));

        self.consumable_mesh.mesh = meshes.add(Sphere::new(inventory::CONSUMABLE_RADIUS));
        self.consumable_mesh.material = materials.add(Color::from(css::WHITE));

        self.bullet_mesh.mesh = meshes.add(Sphere::new(projectile::BULLET_RADIUS));
        self.bullet_mesh.material = materials.add(Color::from(css::BLACK));

        self.floor_mesh.mesh = meshes.add(
            Plane3d::default()
                .mesh()
                .size(world::FLOOR_X_LENGTH, world::FLOOR_Z_LENGTH),
        );
        self.floor_mesh.material = materials.add(Color::srgb(0.3, 0.5, 0.3));

        self.wall_mesh.mesh = meshes.add(Cuboid::new(
            world::WALL_X_LENGTH,
            world::WALL_Y_LENGTH,
            world::WALL_Z_LENGTH,
        ));
        self.wall_mesh.material = materials.add(Color::srgb(0.4, 0.7, 0.3));

        self.box_mesh.mesh = meshes.add(Cuboid::new(
            world::BOX_X_LENGTH,
            world::BOX_Y_LENGTH,
            world::BOX_Z_LENGTH,
        ));
        self.box_mesh.material = materials.add(Color::srgb(0.8, 0.7, 0.6));

        self.crate_mesh.mesh = meshes.add(Cuboid::new(
            world::CRATE_X_LENGTH,
            world::CRATE_Y_LENGTH,
            world::CRATE_Z_LENGTH,
        ));
        self.crate_mesh.material = materials.add(Color::srgb(0.8, 0.7, 0.6));
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
