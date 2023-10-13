#![allow(dead_code)]
use bevy::prelude::*;
use bevy_atmosphere::prelude::*;
use bevy_voxel_engine::{FlyCameraPlugin, WorldPlugin};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FlyCameraPlugin,
            WorldPlugin,
            AtmospherePlugin,
        ))
        .add_systems(
            Startup,
            (
                bevy_voxel_engine::camera_setup,
                spawn_directional_light,
            ),
        )
        .run();
}

fn spawn_directional_light(mut commands: Commands) {
    let angle_rotation: f32 = 65.;
    let light = (
        DirectionalLightBundle {
            directional_light: DirectionalLight {
                shadows_enabled: true,
                illuminance: 100000.,
                ..default()
            },
            transform: Transform::from_rotation(Quat::from_axis_angle(
                Vec3::NEG_X,
                angle_rotation.to_radians(),
            )),
            ..default()
        },
        Name::new("Directional light"),
    );

    commands.spawn(light);
}