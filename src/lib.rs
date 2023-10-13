mod fly_camera;
mod world;

use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
// fly_camera
use fly_camera::FlyCamera;
pub use fly_camera::FlyCameraPlugin;
// world
pub use world::WorldPlugin;

pub fn camera_setup(mut comands: Commands) {
    comands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        AtmosphereCamera::default(),
        FlyCamera::default(),
    ));
}
