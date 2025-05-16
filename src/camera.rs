use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Startup, setup)
        .add_systems(Update, move_cam);
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Msaa::Off));
}

fn move_cam(cam: Single<&mut Transform, With<Camera3d>>, time: Res<Time>) {
    let now = time.elapsed_secs();

    let mut transform = cam.into_inner();

    let orbit_scale = 8.0 + ops::sin(now) * 40.0;
    *transform = Transform::from_xyz(
        ops::cos(now / 5.0) * orbit_scale,
        12.0 - orbit_scale / 2.0,
        ops::sin(now / 5.0) * orbit_scale,
    )
    .looking_at(Vec3::ZERO, Vec3::Y);
}
