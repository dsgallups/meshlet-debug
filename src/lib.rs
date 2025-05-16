#![allow(dead_code)]

use bevy::{
    pbr::experimental::meshlet::MeshletPlugin, prelude::*, render::render_resource::AsBindGroup,
};
mod camera;
mod monke;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Window {
                    position: WindowPosition::Centered(MonitorSelection::Index(1)),
                    focused: false,
                    ..default()
                }
                .into(),
                ..default()
            }),
            MeshletPlugin {
                cluster_buffer_slots: 8192,
            },
            MaterialPlugin::<MeshletDebugMaterial>::default(),
            camera::plugin,
            monke::plugin,
        ));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Clone, Default)]
struct MeshletDebugMaterial {
    _dummy: (),
}
impl Material for MeshletDebugMaterial {}
