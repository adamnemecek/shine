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
    vertex_cache: Vec<u32>,
}

impl Transvoxel {
    pub fn new() -> Transvoxel {
        Transvoxel {
            config: Config::default(),
            vertex_cache: Vec::new(),
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

        self.vertex_cache.resize(2 * 3 * sx * sy, u32::max_value());

        for z in 0isize..(sz as isize) {
            for y in 0isize..(sy as isize) {
                for x in 0isize..(sx as isize) {
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

                    let case_code = ((values[0] as u16) >> 15 & 0x01)
                        | ((values[1] as u16) >> 14 & 0x02)
                        | ((values[2] as u16) >> 13 & 0x04)
                        | ((values[3] as u16) >> 12 & 0x08)
                        | ((values[4] as u16) >> 11 & 0x10)
                        | ((values[5] as u16) >> 10 & 0x20)
                        | ((values[6] as u16) >> 9 & 0x40)
                        | ((values[7] as u16) >> 8 & 0x80);

                    //println!("x,y,z: ({},{},{})", x, y, z);
                    //println!("values: {:?}", values);
                    //println!("case_code: {}", case_code);

                    if case_code == 0 || case_code == 255 {
                        // empty or full cell
                        continue;
                    }

                    let cell_class = REGULAR_CELL_CLASS[case_code as usize];
                    let cell_data = &REGULAR_CELL_DATA[cell_class as usize];
                    let vertex_data = REGULAR_VERTEX_DATA[case_code as usize];

                    //println!("vertex_count: {}", cell_data.get_vertex_count());
                    //println!("triangle_count: {}", cell_data.get_triangle_count());

                    // vertices
                    let mut generated_vertices = [0; 12];
                    let mut generated_vertex_count = 0;
                    for vi in 0..cell_data.get_vertex_count() {
                        let edge = vertex_data[vi];

                        // A: low point / B: high point
                        let start_index = edge.get_start_index();
                        let end_index = edge.get_end_index();
                        //println!("  vi: {}", vi);
                        //println!("    start_index: {}", start_index);
                        //println!("    end_index: {}", end_index);

                        assert!((values[start_index] < 0) != (values[end_index] < 0)); // It is really an edge

                        let edge_index = edge.get_edge_index();
                        assert!(edge_index > 0 && edge_index < 4);
                        let is_cached = edge.is_cached();
                        let cached_direction = edge.get_cached_direction();
                        //println!("    cached_direction: {}", cached_direction);

                        let cx = if (cached_direction & 0x01) != 0 { x - 1 } else { x };
                        let cy = if (cached_direction & 0x02) != 0 { y - 1 } else { y };
                        let cz = if (cached_direction & 0x04) != 0 { z - 1 } else { z };
                        //println!("    cached at: {},{},{}", cx,cy,cz);

                        let generated_vertex = if is_cached && cx >= 0 && cy >= 0 && cz >= 0 {
                            let cache_page = cz as usize % 2;
                            let cache_index = cache_page * 3 * sy * sx
                                + ((edge_index - 1) as usize) * sy * sx
                                + (cy as usize) * sx
                                + (cx as usize);
                            let v = self.vertex_cache[cache_index];
                            //println!("    cache reuse: {}", v);
                            assert!((v as usize) < mesh.vertices.len());
                            v
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
                                let start_value = values[start_index] as f32;
                                let end_value = values[end_index] as f32;

                                // Full resolution
                                let alpha = start_value / (start_value - end_value);
                                //let alpha = 0.5;
                                //println!("    alpha: {} ({},{})", alpha, start_value, end_value);

                                match edge_index {
                                    // y direction
                                    1 => Vec3::new(
                                        start_position.x,
                                        lerp(start_position.y, end_position.y, alpha),
                                        start_position.z,
                                    ),
                                    // x direction
                                    2 => Vec3::new(
                                        lerp(start_position.x, end_position.x, alpha),
                                        start_position.y,
                                        start_position.z,
                                    ),
                                    // z direction
                                    3 => Vec3::new(
                                        start_position.x,
                                        start_position.y,
                                        lerp(start_position.z, end_position.z, alpha),
                                    ),
                                    _ => unreachable!(),
                                }
                            } else {
                                unimplemented!()
                            };

                            let vertex = Vertex::new().with_position(position);
                            mesh.add_vertex(vertex)
                        };
                        generated_vertices[vi] = generated_vertex;
                        generated_vertex_count += 1;

                        // Save vertex index to reuse in the next voxels
                        if !is_cached {
                            let cache_page = (z as usize) % 2;
                            let cache_index = cache_page * 3 * sy * sx
                                + ((edge_index - 1) as usize) * sy * sx
                                + (y as usize) * sx
                                + (x as usize);
                            self.vertex_cache[cache_index] = generated_vertex;
                        }
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
                } // x
            } // y
        } // z

        for v in mesh.vertices[start_vertex..].iter_mut() {
            v.normal = glm::normalize(&v.normal);
        }

        self.vertex_cache.clear();
    }
}
