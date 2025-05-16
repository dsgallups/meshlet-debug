use bevy::{pbr::experimental::meshlet::MeshletMesh3d, prelude::*};

use crate::MeshletDebugMaterial;

const FLOOR: f32 = -5.;

const ORIGINAL_GLB: Transform = Transform::from_xyz(0., FLOOR, 3.);

const MONKEY_MESHLET: Transform = Transform::from_xyz(-3., FLOOR, 3.);
const HIGH_POLY_MONKE_MESHLET: Transform = Transform::from_xyz(-6., FLOOR, 3.);
const BUNNY_MESHLET: Transform = Transform::from_xyz(3., FLOOR, 3.);

const BIRCH: Transform = Transform::from_xyz(5., FLOOR, 0.);
const BIRCH_MESHLET: Transform = Transform::from_xyz(-5., FLOOR, 0.);

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup);
}

fn setup(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut debug_materials: ResMut<Assets<MeshletDebugMaterial>>,
) {
    let original_glb = assets.load("models/monke.glb#Scene0");
    commands.spawn((SceneRoot(original_glb), ORIGINAL_GLB));

    let monkey_meshlet = assets.load("meshlets/monke.meshlet");
    let debug_material = debug_materials.add(MeshletDebugMaterial::default());

    commands.spawn((
        MeshletMesh3d(monkey_meshlet),
        MeshMaterial3d(debug_material.clone()),
        MONKEY_MESHLET,
    ));
    let bunny_meshlet = assets.load("meshlets/bunny.meshlet_mesh");
    commands.spawn((
        MeshletMesh3d(bunny_meshlet),
        MeshMaterial3d(debug_material.clone()),
        BUNNY_MESHLET,
    ));

    let big_monke_meshlet = assets.load("meshlets/bigger_monkey.meshlet");
    commands.spawn((
        MeshletMesh3d(big_monke_meshlet),
        MeshMaterial3d(debug_material.clone()),
        HIGH_POLY_MONKE_MESHLET,
    ));

    let birch = assets.load("models/birch.glb#Scene0");
    commands.spawn((SceneRoot(birch), BIRCH));

    let birch_meshlet = assets.load("meshlets/birch.meshlet");
    commands.spawn((
        MeshletMesh3d(birch_meshlet),
        MeshMaterial3d(debug_material.clone()),
        BIRCH_MESHLET,
    ));
    //let
}
