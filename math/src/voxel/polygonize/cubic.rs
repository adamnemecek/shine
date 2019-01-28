use crate::voxel::polygonize::mesh::{Mesh, Vertex};
use crate::voxel::polygonize::{Config, Direction, Polygonizer, UVGeneration};
use crate::voxel::Cell;
use nalgebra_glm::{Vec2, Vec3};

fn add_face(mesh: &mut Mesh, config: &Config, x: isize, y: isize, z: isize, direction: Direction, step: isize) {
    let (a, b, c, d, normal, tangent) = match direction {
        Direction::XMin => (
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(-1., 0., 0.),
            Vec3::new(0., -1., 0.),
        ),
        Direction::XMax => (
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(1., 0., 0.),
            Vec3::new(0., 1., 0.),
        ),
        Direction::YMin => (
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(0., -1., 0.),
            Vec3::new(-1., 0., 0.),
        ),
        Direction::YMax => (
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(0., 1., 0.),
            Vec3::new(1., 0., 0.),
        ),
        Direction::ZMin => (
            Vec3::new(-0.5, 0.5, -0.5),
            Vec3::new(0.5, 0.5, -0.5),
            Vec3::new(0.5, -0.5, -0.5),
            Vec3::new(-0.5, -0.5, -0.5),
            Vec3::new(0., 0., -1.),
            Vec3::new(-1., 0., 0.),
        ),
        Direction::ZMax => (
            Vec3::new(-0.5, -0.5, 0.5),
            Vec3::new(0.5, -0.5, 0.5),
            Vec3::new(0.5, 0.5, 0.5),
            Vec3::new(-0.5, 0.5, 0.5),
            Vec3::new(0., 0., 0.5),
            Vec3::new(1., 0., 0.),
        ),
    };

    let center = Vec3::new(x as f32, y as f32, z as f32);
    let step = step as f32;
    let uv = Vec2::new(0., 0.);

    let a = {
        let vertex = Vertex::new()
            .with_position((a + center) * step)
            .with_normal(normal)
            .with_tangent(tangent)
            .with_uv(match config.uv_generation {
                UVGeneration::PerVoxel => Vec2::new(0., 0.),
                _ => uv,
            });
        mesh.add_vertex(vertex)
    };

    let b = {
        let vertex = Vertex::new()
            .with_position((b + center) * step)
            .with_normal(normal)
            .with_tangent(tangent)
            .with_uv(match config.uv_generation {
                UVGeneration::PerVoxel => Vec2::new(1., 0.),
                _ => uv,
            });
        mesh.add_vertex(vertex)
    };

    let c = {
        let vertex = Vertex::new()
            .with_position((c + center) * step)
            .with_normal(normal)
            .with_tangent(tangent)
            .with_uv(match config.uv_generation {
                UVGeneration::PerVoxel => Vec2::new(1., 1.),
                _ => uv,
            });
        mesh.add_vertex(vertex)
    };

    let d = {
        let vertex = Vertex::new()
            .with_position((d + center) * step)
            .with_normal(normal)
            .with_tangent(tangent)
            .with_uv(match config.uv_generation {
                UVGeneration::PerVoxel => Vec2::new(0., 1.),
                _ => uv,
            });
        mesh.add_vertex(vertex)
    };

    mesh.add_triangle(a, b, c);
    mesh.add_triangle(a, c, d);
}

pub struct Cubic {
    config: Config,
}

impl Cubic {
    pub fn new() -> Cubic {
        Cubic {
            config: Config::default(),
        }
    }

    pub fn with_config(self, config: Config) -> Self {
        Cubic { config, ..self }
    }
}

impl Polygonizer for Cubic {
    fn polygonize<C: Cell>(&mut self, mesh: &mut Mesh, cell: &C) {
        let (sx, sy, sz) = cell.resolution();
        let lod = cell.lod();
        let step = 1 << lod;

        for x in 0isize..(sx as isize) {
            for y in 0isize..(sy as isize) {
                for z in 0isize..(sz as isize) {
                    let value = cell.get(0, x, y, z);
                    if value >= 0 {
                        // outside of the volume
                        continue;
                    }

                    // check if neighbor is outside along any direction (this voxel is inside)
                    if cell.get(0, x - 1, y, z) >= 0 {
                        add_face(mesh, &self.config, x, y, z, Direction::XMin, step);
                    }
                    if cell.get(0, x + 1, y, z) >= 0 {
                        add_face(mesh, &self.config, x, y, z, Direction::XMax, step);
                    }
                    if cell.get(0, x, y - 1, z) >= 0 {
                        add_face(mesh, &self.config, x, y, z, Direction::YMin, step);
                    }
                    if cell.get(0, x, y + 1, z) >= 0 {
                        add_face(mesh, &self.config, x, y, z, Direction::YMax, step);
                    }
                    if cell.get(0, x, y, z - 1) >= 0 {
                        add_face(mesh, &self.config, x, y, z, Direction::ZMin, step);
                    }
                    if cell.get(0, x, y, z + 1) >= 0 {
                        add_face(mesh, &self.config, x, y, z, Direction::ZMax, step);
                    }
                }
            }
        }
    }
}
