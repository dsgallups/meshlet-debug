use bevy::{gltf::GltfMesh, prelude::*};

use crate::scene_viewer_plugin::SceneHandle;

pub fn plugin(app: &mut App) {
    app.add_systems(PreUpdate, log_scene);
}

fn log_scene(
    scene_handle: Res<SceneHandle>,
    mut run: Local<bool>,
    gltfs: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
) {
    if *run || !scene_handle.is_loaded {
        return;
    }

    let gltf = gltfs.get(&scene_handle.gltf_handle).unwrap();

    let mesh_handles = &gltf.meshes;

    info!("Number of meshes: {}", mesh_handles.len());
    let first = mesh_handles.first().unwrap();

    let gltf_mesh = gltf_meshes.get(first).unwrap();
    let primitives = &gltf_mesh.primitives;

    info!("Mesh primitives len: {}", primitives.len());
    let mesh_handle = &primitives.first().unwrap().mesh;

    let mesh = meshes.get(mesh_handle).unwrap();

    let primitive_topology = mesh.primitive_topology();
    let indices = mesh.indices();

    info!(
        "Mesh:\nprimitive_topology: {:?}\nindices: {:?}",
        primitive_topology, indices
    );

    *run = true;
}
