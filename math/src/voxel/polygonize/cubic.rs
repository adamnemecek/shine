use crate::voxel::polygonize::{Config, Direction, Mesh, Polygonizer, UVGeneration};
use crate::voxel::Cell;

pub struct Cubic<'a, M>
where
    M: Mesh,
{
    mesh: &'a mut M,
    config: Config,
}

impl<'a, M> Cubic<'a, M>
where
    M: Mesh,
{
    pub fn new<'b>(mesh: &'b mut M) -> Cubic<'b, M> {
        Cubic {
            mesh,
            config: Config::default(),
        }
    }

    pub fn with_config(self, config: Config) -> Self {
        Cubic { config, ..self }
    }

    fn add_face(&mut self, x: isize, y: isize, z: isize, direction: Direction, step: isize) {
        let (a, b, c, d, normal, tangent) = match direction {
            Direction::XMin => (
                (0., 0., 1.),
                (0., 1., 1.),
                (0., 1., 0.),
                (0., 0., 0.),
                (-1., 0., 0.),
                (0., -1., 0.),
            ),
            Direction::XMax => (
                (1., 0., 0.),
                (1., 1., 0.),
                (1., 1., 1.),
                (1., 0., 1.),
                (1., 0., 0.),
                (0., 1., 0.),
            ),
            Direction::YMin => (
                (0., 0., 1.),
                (0., 0., 0.),
                (1., 0., 0.),
                (1., 0., 1.),
                (0., -1., 0.),
                (-1., 0., 0.),
            ),
            Direction::YMax => (
                (1., 1., 1.),
                (1., 1., 0.),
                (0., 1., 0.),
                (0., 1., 1.),
                (0., 1., 0.),
                (1., 0., 0.),
            ),
            Direction::ZMin => (
                (0., 1., 0.),
                (1., 1., 0.),
                (1., 0., 0.),
                (0., 0., 0.),
                (0., 0., -1.),
                (-1., 0., 0.),
            ),
            Direction::ZMax => (
                (0., 0., 1.),
                (1., 0., 1.),
                (1., 1., 1.),
                (0., 1., 1.),
                (0., 0., 1.),
                (1., 0., 0.),
            ),
        };

        let center = (x as f32, y as f32, z as f32);
        let step = step as f32;
        let mut uv = (0., 0.);

        let a = {
            let pos = ((a.0 + center.0) * step, (a.1 + center.1) * step, (a.2 + center.2) * step);
            match self.config.uv_generation {
                UVGeneration::PerVoxel => uv = (0., 0.),
                _ => {}
            }
            self.mesh.add_vertex(pos, normal, tangent, uv)
        };

        let b = {
            let pos = ((b.0 + center.0) * step, (b.1 + center.1) * step, (b.2 + center.2) * step);
            match self.config.uv_generation {
                UVGeneration::PerVoxel => uv = (1., 0.),
                _ => {}
            }
            self.mesh.add_vertex(pos, normal, tangent, uv)
        };

        let c = {
            let pos = ((c.0 + center.0) * step, (c.1 + center.1) * step, (c.2 + center.2) * step);
            match self.config.uv_generation {
                UVGeneration::PerVoxel => uv = (1., 1.),
                _ => {}
            }
            self.mesh.add_vertex(pos, normal, tangent, uv)
        };

        let d = {
            let pos = ((d.0 + center.0) * step, (d.1 + center.1) * step, (d.2 + center.2) * step);
            match self.config.uv_generation {
                UVGeneration::PerVoxel => uv = (0., 1.),
                _ => {}
            }
            self.mesh.add_vertex(pos, normal, tangent, uv)
        };

        self.mesh.add_triangle(a, b, c);
        self.mesh.add_triangle(a, c, d);
    }
}

impl<'a, M> Polygonizer for Cubic<'a, M>
where
    M: Mesh,
{
    fn polygonize<C: Cell>(&mut self, cell: &C, lod: u32) {
        let step = 1 << lod;

        for x in cell.x_range() {
            for y in cell.y_range() {
                for z in cell.z_range() {
                    let value = cell.get(lod, x, y, z);
                    if !value {
                        continue;
                    }

                    if !cell.get(lod, x - 1, y, z) {
                        self.add_face(x, y, z, Direction::XMin, step);
                    }
                    if !cell.get(lod, x + 1, y, z) {
                        self.add_face(x, y, z, Direction::XMax, step);
                    }
                    if !cell.get(lod, x, y - 1, z) {
                        self.add_face(x, y, z, Direction::YMin, step);
                    }
                    if !cell.get(lod, x, y + 1, z) {
                        self.add_face(x, y, z, Direction::YMax, step);
                    }
                    if !cell.get(lod, x, y, z - 1) {
                        self.add_face(x, y, z, Direction::ZMin, step);
                    }
                    if !cell.get(lod, x, y, z + 1) {
                        self.add_face(x, y, z, Direction::ZMax, step);
                    }
                }
            }
        }
    }
}
