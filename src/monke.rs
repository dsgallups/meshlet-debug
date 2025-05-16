use bevy::{pbr::experimental::meshlet::MeshletMesh3d, prelude::*};

use crate::MeshletDebugMaterial;

const ORIGINAL_GLB: Transform = Transform::from_xyz(0., 0., 0.);

const PREPROCESSED_MESHLET: Transform = Transform::from_xyz(-3., 0., 0.);

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

    let meshlet_mesh = assets.load("meshlets/monke.meshlet");
    let debug_material = debug_materials.add(MeshletDebugMaterial::default());

    commands.spawn((
        MeshletMesh3d(meshlet_mesh),
        MeshMaterial3d(debug_material),
        PREPROCESSED_MESHLET,
    ));

    //let
}
