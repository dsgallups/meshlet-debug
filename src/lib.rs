use bevy::{pbr::experimental::meshlet::MeshletPlugin, prelude::*};
mod camera;
mod monke;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins,
            MeshletPlugin {
                cluster_buffer_slots: 8192,
            },
            camera::plugin,
            monke::plugin,
        ));
    }
}
