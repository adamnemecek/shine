use crate::voxel::polygonize::mesh::{Mesh, Vertex};
use crate::voxel::polygonize::transvoxel_lut::*;
use crate::voxel::polygonize::{Config, Direction, Polygonizer, UVGeneration};
use crate::voxel::Cell;
use nalgebra_glm::{self as glm, Vec3};

fn lerp(x: f32, y: f32, w: f32) -> f32 {
    x * (1. - w) + y * w
}

pub struct Transvoxel {
    config: Config,
}

impl Transvoxel {
    pub fn new() -> Transvoxel {
        Transvoxel {
            config: Config::default(),
        }
    }

    pub fn with_config(self, config: Config) -> Self {
        Transvoxel { config, ..self }
    }
}

impl Polygonizer for Transvoxel {
    fn polygonize<C: Cell>(&mut self, mesh: &mut Mesh, cell: &C) {
        let (sx, sy, sz) = cell.resolution();
        let lod = cell.lod();
        let step = 1 << lod;
        let start_vertex = mesh.vertices.len();

        for x in 0isize..(sx as isize) {
            for y in 0isize..(sy as isize) {
                for z in 0isize..(sz as isize) {
                    let values = [
                        cell.get(0, x, y, z),
                        cell.get(0, x + 1, y, z),
                        cell.get(0, x, y + 1, z),
                        cell.get(0, x + 1, y + 1, z),
                        cell.get(0, x, y, z + 1),
                        cell.get(0, x + 1, y, z + 1),
                        cell.get(0, x, y + 1, z + 1),
                        cell.get(0, x + 1, y + 1, z + 1),
                    ];

                    let case_code = (((values[0] > 0) as u8) << 0)
                        | (((values[1] > 0) as u8) << 1)
                        | (((values[2] > 0) as u8) << 2)
                        | (((values[3] > 0) as u8) << 3)
                        | (((values[4] > 0) as u8) << 4)
                        | (((values[5] > 0) as u8) << 5)
                        | (((values[6] > 0) as u8) << 6)
                        | (((values[7] > 0) as u8) << 7);

                    if case_code == 0 || case_code == 255 {
                        // empty or full cell
                        continue;
                    }

                    let cell_class = REGULAR_CELL_CLASS[case_code as usize];
                    let cell_data = &REGULAR_CELL_DATA[cell_class as usize];
                    let vertex_data = REGULAR_VERTEX_DATA[case_code as usize];

                    // vertices
                    let mut generated_vertices = [0; 12];
                    let mut generated_vertex_count = 0;
                    for vi in 0..cell_data.get_vertex_count() {
                        let edge = vertex_data[vi];

                        // A: low point / B: high point
                        let start_index = edge.get_start_index();
                        let end_index = edge.get_end_index();

                        let start_value = values[start_index];
                        let end_value = values[end_index];
                        assert!((start_value > 0) != (end_value > 0)); // It is really an edge (error in table)

                        let is_cached = edge.is_cached();
                        let cached_edge = edge.get_cached_index();
                        let cached_direction = edge.get_cached_direction();

                        let generated_vertex = if is_cached  {
                            unimplemented!()
                        } else {

                            // Compute vertex
                            let start_position = Vec3::new(
                                ((x + (((start_index & 0x01) >> 0) as isize)) * step) as f32,
                                ((y + (((start_index & 0x02) >> 1) as isize)) * step) as f32,
                                ((z + (((start_index & 0x04) >> 2) as isize)) * step) as f32,
                            );
                            let end_position = Vec3::new(
                                ((x + (((end_index & 0x01) >> 0) as isize)) * step) as f32,
                                ((y + (((end_index & 0x02) >> 1) as isize)) * step) as f32,
                                ((z + (((end_index & 0x04) >> 2) as isize)) * step) as f32,
                            );

                            let position = if lod == 0 {
                                // Full resolution
                                let alpha = (start_value as f32) / (start_value as f32 - end_value as f32);

                                match cached_edge {
                                    1 => Vec3::new(
                                        start_position.x,
                                        lerp(start_position.y, end_position.y, alpha),
                                        start_position.z,
                                    ), // y
                                    2 => Vec3::new(
                                        lerp(start_position.x, end_position.x, alpha),
                                        start_position.y,
                                        start_position.z,
                                    ), // x
                                    3 => Vec3::new(
                                        start_position.x,
                                        start_position.y,
                                        lerp(start_position.z, end_position.z, alpha),
                                    ), // z
                                    _ => unreachable!(),
                                }
                            } else {
                                unimplemented!()
                            };

                            // Save vertex if not on edge
                            /*if (isCacheDirection & 0x08) // start_valuetB.IsNull() && LocalIndexB == 7 => !CacheDirection
                            {
                                GetCurrentCache()[GetCacheIndex(EdgeIndex, LX, LY)] = VertexIndex;
                            }*/                        
                        
                            let vertex = Vertex::new().with_position(position);
                            mesh.add_vertex(vertex)
                        };
                        generated_vertices[vi] = generated_vertex;
                        generated_vertex_count += 1;
                    }

                    // triangle
                    for ti in 0..cell_data.get_triangle_count() {
                        let a = generated_vertices[cell_data.indices[3 * ti] as usize];
                        let b = generated_vertices[cell_data.indices[3 * ti + 1] as usize];
                        let c = generated_vertices[cell_data.indices[3 * ti + 2] as usize];

                        mesh.add_triangle(a, b, c);
                        let (va, vb, vc) = mesh.get_triangle_vertices_mut(a, b, c);

                        //if (NormalConfig == EVoxelNormalConfig::SmoothNormal)
                        {
                            let ac = vc.position - va.position;
                            let ab = vb.position - va.position;
                            let normal = ac.cross(&ab).normalize();

                            va.normal += normal;
                            vb.normal += normal;
                            vc.normal += normal;
                        }
                    }
                }
            }
        }

    let mut a = 0;
        for v in mesh.vertices[start_vertex..].iter_mut() {
            v.normal = glm::normalize(&v.normal);
            println!("{}. {}",a, glm::length(&v.normal));
            a += 1;
        }
    }
}
