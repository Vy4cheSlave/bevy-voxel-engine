#![allow(dead_code)]
use super::logic_of_marching_cubes::{self, VoxelGrid};

use bevy::prelude::*;
use bevy::render::mesh::{self, PrimitiveTopology};

//размер воксельного чанка в абстрактной системе счисления
const SIZE_CHUNK: u16 = 32;

pub struct ResolutionOfTheGrid {
    value: u64,
}

impl ResolutionOfTheGrid {
    pub fn new(value: u64) -> Self {
        if value == 0 {
            panic!("ResolutionOfTheGrid should take a NON NEGATIVE value")
        } else {
            ResolutionOfTheGrid { value }
        }
    }

    pub fn value(&self) -> usize {
        self.value as usize
    }
}

#[derive(Component)]
pub struct VoxelChunk {
    coordinates: [i128; 3],
}

impl VoxelChunk {
    pub fn new(coordinates: [i128; 3]) -> Self {
        VoxelChunk { coordinates }
    }

    pub fn get_chunk_coordinates_from_global(value: [f32; 3]) -> [i128; 3] {
        [
            Self::round_chunk_coordinates(value[0] / SIZE_CHUNK as f32) as i128,
            Self::round_chunk_coordinates(value[1] / SIZE_CHUNK as f32) as i128,
            Self::round_chunk_coordinates(value[2] / SIZE_CHUNK as f32) as i128,
        ]
    }

    pub fn get_chunk_coordinates_from_global_as_vec3(value: [f32; 3]) -> Vec3 {
        Vec3::new(
            Self::round_chunk_coordinates(value[0] / SIZE_CHUNK as f32),
            Self::round_chunk_coordinates(value[1] / SIZE_CHUNK as f32),
            Self::round_chunk_coordinates(value[2] / SIZE_CHUNK as f32),
        )
    }

    pub fn size_chunk() -> u16 {
        SIZE_CHUNK
    }

    pub fn coordinates(&self) -> &[i128; 3] {
        &self.coordinates
    }

    pub fn coordinates_as_vec3(&self) -> Vec3 {
        Vec3::new(
            self.coordinates[0] as f32,
            self.coordinates[1] as f32,
            self.coordinates[2] as f32,
        )
    }

    pub fn coordinates_set(&mut self, coordinates: [i128; 3]) {
        self.coordinates = coordinates;
    }

    pub fn return_chunk_mesh(
        &mut self,
        generation_rules_for_the_grid: impl Fn([f64; 3]) -> f64,
        resolution: ResolutionOfTheGrid,
    ) -> mesh::Mesh {
        // разрешение воксельной сетки
        let resolution = resolution.value();
        let resolution_size = resolution - 1;
        let scale_of_the_step_coordinates: f64 = SIZE_CHUNK as f64 / resolution_size as f64;

        let mut voxel_grid = VoxelGrid::new(resolution);

        for z in 0..resolution {
            for y in 0..resolution {
                for x in 0..resolution {
                    // вот здесь заменить
                    voxel_grid.push(generation_rules_for_the_grid([
                        (x as f64 + self.coordinates[0] as f64 * resolution_size as f64)
                            * scale_of_the_step_coordinates
                            // делает неазвисимым от размера чанка
                            / SIZE_CHUNK as f64,
                        (y as f64 + self.coordinates[1] as f64 * resolution_size as f64)
                            * scale_of_the_step_coordinates
                            / SIZE_CHUNK as f64,
                        (z as f64 + self.coordinates[2] as f64 * resolution_size as f64)
                            * scale_of_the_step_coordinates
                            / SIZE_CHUNK as f64,
                    ]) as f32);
                }
            }
        }

        let mut positions: Vec<[f32; 3]> = Vec::new();
        for z in 0..resolution - 1 {
            for y in 0..resolution - 1 {
                for x in 0..resolution - 1 {
                    logic_of_marching_cubes::march_cube((x, y, z), &voxel_grid, &mut positions);
                }
            }
        }
        self.get_transform_vertex(&mut positions, resolution_size);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        // indices for uv
        let indices_for_uv: Vec<u32> = (0..positions.len()).map(|v| v as u32).collect();
        // Normals of the vertices
        let mut normals: Vec<[f32; 3]> = Vec::new();
        for index in (0..indices_for_uv.len()).step_by(3) {
            let value = logic_of_marching_cubes::polygon_normal(
                &positions[index],
                &positions[index + 1],
                &positions[index + 2],
            );
            normals.push(value.clone());
            normals.push(value.clone());
            normals.push(value);
        }

        // Positions of the vertices
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        // normals
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        // uv
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0., 0.]; indices_for_uv.len()]);
        // A triangle using vertices
        mesh.set_indices(Some(mesh::Indices::U32(indices_for_uv)));
        // return
        mesh
    }

    fn get_transform_vertex(&self, positions: &mut Vec<[f32; 3]>, resolution_size: usize) {
        positions.iter_mut().for_each(|vertex| {
            *vertex = [
                // вершина / разрешение * размер чанка = преобразование в новые координаты, относительно нулевых глобальных
                // преобразование в новые координаты + координаты чанка * на размер чанка = смещение к координатам чанка
                vertex[0] / resolution_size as f32 * SIZE_CHUNK as f32 //- SIZE_CHUNK as f32 / 2.
                    + self.coordinates[0] as f32 * SIZE_CHUNK as f32,
                vertex[1] / resolution_size as f32 * SIZE_CHUNK as f32 //- SIZE_CHUNK as f32 / 2.
                    + self.coordinates[1] as f32 * SIZE_CHUNK as f32,
                vertex[2] / resolution_size as f32 * SIZE_CHUNK as f32 //- SIZE_CHUNK as f32 / 2.
                    + self.coordinates[2] as f32 * SIZE_CHUNK as f32,
            ]
        });
    }

    fn round_chunk_coordinates(value: f32) -> f32 {
        if value >= 0. {
            value.floor()
        } else {
            value.floor()
        }
    }
}

impl Default for VoxelChunk {
    fn default() -> Self {
        VoxelChunk {
            coordinates: [0, 0, 0],
        }
    }
}
