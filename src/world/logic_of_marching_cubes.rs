use bevy::prelude::*;

use super::data_for_marching_cubes::{
    EDGES_FOR_MARCHING_CUBES, POINTS_FOR_MARCHING_CUBES, TRIANGULATIONS_FOR_MARCHING_CUBES,
};

pub struct VoxelGrid {
    data: Vec<f32>,
    // число вокселей у сетки (сторона куба/чанка)
    resolution: usize,
}

impl VoxelGrid {
    pub fn new(resolution: usize) -> Self {
        Self {
            data: Vec::with_capacity(resolution * resolution * resolution),
            resolution,
        }
    }

    pub fn read(&self, x: usize, y: usize, z: usize) -> f32 {
        self.data[x + y * self.resolution + z * self.resolution * self.resolution]
    }

    pub fn push(&mut self, value: f32) {
        self.data.push(value);
    }
}

fn get_triangulation(voxel_grid: &VoxelGrid, (x, y, z): (usize, usize, usize)) -> [i8; 15] {
    let mut config_idx = 0b00000000;

    config_idx |= (voxel_grid.read(x, y, z).is_sign_negative() as u8) << 0;
    config_idx |= (voxel_grid.read(x, y, z + 1).is_sign_negative() as u8) << 1;
    config_idx |= (voxel_grid.read(x + 1, y, z + 1).is_sign_negative() as u8) << 2;
    config_idx |= (voxel_grid.read(x + 1, y, z).is_sign_negative() as u8) << 3;
    config_idx |= (voxel_grid.read(x, y + 1, z).is_sign_negative() as u8) << 4;
    config_idx |= (voxel_grid.read(x, y + 1, z + 1).is_sign_negative() as u8) << 5;
    config_idx |= (voxel_grid.read(x + 1, y + 1, z + 1).is_sign_negative() as u8) << 6;
    config_idx |= (voxel_grid.read(x + 1, y + 1, z).is_sign_negative() as u8) << 7;

    return TRIANGULATIONS_FOR_MARCHING_CUBES[config_idx as usize];
}

pub fn march_cube(
    (x, y, z): (usize, usize, usize),
    voxel_grid: &VoxelGrid,
    positions: &mut Vec<[f32; 3]>,
) {
    let triangulation = get_triangulation(voxel_grid, (x, y, z));

    for edge_index in triangulation {
        if edge_index.is_negative() {
            break;
        }

        let point_indices = EDGES_FOR_MARCHING_CUBES[edge_index as usize];

        let (x0, y0, z0) = POINTS_FOR_MARCHING_CUBES[point_indices.0];
        let (x1, y1, z1) = POINTS_FOR_MARCHING_CUBES[point_indices.1];

        let pos_a = Vec3::new((x + x0) as f32, (y + y0) as f32, (z + z0) as f32);
        let pos_b = Vec3::new((x + x1) as f32, (y + y1) as f32, (z + z1) as f32);

        // среднее между двумя значениями
        // let position = (pos_a + pos_b) * 0.5 - voxel_grid.resolution as f32 / 2.;

        // линейная интерполяция
        let val_a = voxel_grid.read(x + x0, y + y0, z + z0);
        let val_b = voxel_grid.read(x + x1, y + y1, z + z1);

        let t = val_a / (val_a - val_b);
        let position = pos_a + (pos_b - pos_a) * t;

        positions.push(position.into());
    }
}

pub fn polygon_normal(vrtx1: &[f32; 3], vrtx2: &[f32; 3], vrtx3: &[f32; 3]) -> [f32; 3] {
    let vector_vertex_1 = Vec3::from_array(*vrtx1);
    let vector_vertex_2 = Vec3::from_array(*vrtx2);
    let vector_vertex_3 = Vec3::from_array(*vrtx3);

    let vec1 = vector_vertex_2 - vector_vertex_1;
    let vec2 = vector_vertex_3 - vector_vertex_1;

    vec1.cross(vec2).normalize().into()
}
