use bevy::prelude::*;
mod camera;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, camera::plugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(SceneRoot(assets.load("monke.glb#Scene0")));
}
