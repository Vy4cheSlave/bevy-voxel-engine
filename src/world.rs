#![allow(dead_code)]
mod chunk_from_marching_cubes;
mod data_for_marching_cubes;
mod logic_of_marching_cubes;

use bevy::{prelude::*, utils::hashbrown::HashMap};
use noise::{BasicMulti, NoiseFn, Seedable, SuperSimplex};

use chunk_from_marching_cubes::{ResolutionOfTheGrid, VoxelChunk};
pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GreetTimer(Timer::from_seconds(
            25. / 60.,
            TimerMode::Repeating,
        )))
        .insert_resource(InitChunkTimer(Timer::from_seconds(
            1.,
            TimerMode::Repeating,
        )))
        .add_systems(Startup, (init_chunk_creation))
        .add_systems(
            Update,
            (
                generate_chunk_mesh,
                delete_chunk_mesh,
            ),
        );
    }
}

#[derive(Component)]
struct ChunkGenerated;

#[derive(Component)]
struct ChunkNotGenerated;

#[derive(Resource)]
struct GreetTimer(Timer);

#[derive(Resource)]
struct InitChunkTimer(Timer);

const MAX_VIEW_DISTANCE: i64 = 3000;

// потребуется распаралеливание
fn init_chunk_creation(mut commands: Commands) {
    for z in -100..=100 {
        for y in -5..=5 {
            for x in -100..=100 {
                commands.spawn((VoxelChunk::new([x, y, z]), ChunkNotGenerated));
            }
        }
    }
}

fn generate_chunk_mesh(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_q: Query<&GlobalTransform, With<Camera3d>>,
    mut entity_q: Query<(Entity, &mut VoxelChunk), With<ChunkNotGenerated>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut mesh: Mesh;
        let super_simplex = SuperSimplex::new(0);

        let rules_of_generation = |value: [f64; 3]| {
            #[allow(dead_code)]
            let surface_y = 0. + super_simplex.get([value[0], value[2]]) / 1.2;
            if surface_y > value[1] / 1. {
                0.3
            } else {
                -0.3
            }
        };

        let chunk_visible_in_view_distance =
            (MAX_VIEW_DISTANCE as f32 / VoxelChunk::size_chunk() as f32).floor();
        let camera_translation = camera_q.single().translation().to_array();
        let chunk_position_with_camera =
            VoxelChunk::get_chunk_coordinates_from_global_as_vec3(camera_translation);

        for (entity, mut voxel_chunk_q) in entity_q.iter_mut()
        {
            match Vec3::distance_squared(
                chunk_position_with_camera,
                voxel_chunk_q.coordinates_as_vec3(),
            ) {
                val if val <= chunk_visible_in_view_distance / 4. => {
                    mesh = voxel_chunk_q
                        .return_chunk_mesh(rules_of_generation, ResolutionOfTheGrid::new(32));
                    commands.entity(entity).remove::<ChunkNotGenerated>();
                    commands.entity(entity).insert((
                        PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(Color::BLUE.into()),
                            ..default()
                        },
                        ChunkGenerated,
                    ));
                }
                val if val > chunk_visible_in_view_distance / 4.
                    && val <= chunk_visible_in_view_distance / 4. * 3. =>
                {
                    mesh = voxel_chunk_q
                        .return_chunk_mesh(rules_of_generation, ResolutionOfTheGrid::new(16));
                    commands.entity(entity).remove::<ChunkNotGenerated>();
                    commands.entity(entity).insert((
                        PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(Color::BLUE.into()),
                            ..default()
                        },
                        ChunkGenerated,
                    ));
                }
                val if val > chunk_visible_in_view_distance / 4. * 3.
                    && val <= chunk_visible_in_view_distance =>
                {
                    mesh = voxel_chunk_q
                        .return_chunk_mesh(rules_of_generation, ResolutionOfTheGrid::new(8));
                    commands.entity(entity).remove::<ChunkNotGenerated>();
                    commands.entity(entity).insert((
                        PbrBundle {
                            mesh: meshes.add(mesh),
                            material: materials.add(Color::BLUE.into()),
                            ..default()
                        },
                        ChunkGenerated,
                    ));
                }
                _ => (),
            }
        }
    }
}

fn delete_chunk_mesh(
    time: Res<Time>,
    mut timer: ResMut<GreetTimer>,
    mut commands: Commands,
    camera_q: Query<&GlobalTransform, With<Camera3d>>,
    entity_q: Query<(Entity, &VoxelChunk), With<ChunkGenerated>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        let chunk_visible_in_view_distance =
            (MAX_VIEW_DISTANCE as f32 / VoxelChunk::size_chunk() as f32).floor();
        let camera_translation = camera_q.single().translation().to_array();
        let chunk_position_with_camera =
            VoxelChunk::get_chunk_coordinates_from_global_as_vec3(camera_translation);

        for (entity, voxel_chunk) in entity_q.iter().filter(|(_, c)| {
            Vec3::distance_squared(chunk_position_with_camera, c.coordinates_as_vec3())
                > chunk_visible_in_view_distance
        }) {
            commands
                .entity(entity)
                .remove::<(PbrBundle, ChunkGenerated)>();
            commands.entity(entity).insert(ChunkNotGenerated);
        }
    }
}
