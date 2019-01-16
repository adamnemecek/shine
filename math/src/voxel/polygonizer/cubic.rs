
use crate::voxel::Cell;
use crate::voxel::polygonizer::{Config, Direction, MeshBuilder, Polygonizer, UVGeneration};

pub struct Cubic<M>
where
    M: MeshBuilder,
{
    mesh_builder: M,
}

impl<M> Cubic<M>
where
    M: MeshBuilder,
{
    fn add_face(&mut self, cfg: &Config, x: isize, y: isize, z: isize, direction: Direction, step: isize) {
        let (a, b, c, d, normal, tangent, reversed) = match direction {
            Direction::XMin => (
                (0., 0., 1.),
                (0., 1., 1.),
                (0., 1., 0.),
                (0., 0., 0.),
                (-1., 0., 0.),
                (0., -1., 0.),
                false,
            ),
            Direction::XMax => (
                (1., 0., 1.),
                (1., 1., 1.),
                (1., 1., 0.),
                (1., 0., 0.),
                (1., 0., 0.),
                (0., 1., 0.),
                true,
            ),
            Direction::YMin => (
                (0., 0., 1.),
                (1., 0., 1.),
                (1., 0., 0.),
                (0., 0., 0.),
                (0., -1., 0.),
                (-1., 0., 0.),
                true,
            ),
            Direction::YMax => (
                (0., 1., 1.),
                (1., 1., 1.),
                (1., 1., 0.),
                (0., 1., 0.),
                (0., 1., 0.),
                (1., 0., 0.),
                false,
            ),
            Direction::ZMin => (
                (0., 1., 0.),
                (1., 1., 0.),
                (1., 0., 0.),
                (0., 0., 0.),
                (0., 0., -1.),
                (-1., 0., 0.),
                false,
            ),
            Direction::ZMax => (
                (0., 1., 1.),
                (1., 1., 1.),
                (1., 0., 1.),
                (0., 0., 1.),
                (0., 0., 1.),
                (1., 0., 0.),
                true,
            ),
        };

        let center = (x as f32, y as f32, z as f32);
        let step = step as f32;
        let mut uv = (0., 0.);

        let a = {
            let pos = ((a.0 + center.0) * step, (a.1 + center.1) * step, (a.2 + center.2) * step);
            match cfg.uv_generation {
                UVGeneration::PerVoxel => uv = if reversed { (1., 0.) } else { (0., 0.) },
                _ => {}
            }
            self.mesh_builder.add_vertex(pos, normal, tangent, uv)
        };

        let b = {
            let pos = ((b.0 + center.0) * step, (b.1 + center.1) * step, (b.2 + center.2) * step);
            match cfg.uv_generation {
                UVGeneration::PerVoxel => uv = if reversed { (0., 0.) } else { (1., 0.) },
                _ => {}
            }
            self.mesh_builder.add_vertex(pos, normal, tangent, uv)
        };

        let c = {
            let pos = ((c.0 + center.0) * step, (c.1 + center.1) * step, (c.2 + center.2) * step);
            match cfg.uv_generation {
                UVGeneration::PerVoxel => uv = if reversed { (0., 1.) } else { (1., 1.) },
                _ => {}
            }
            self.mesh_builder.add_vertex(pos, normal, tangent, uv)
        };

        let d = {
            let pos = ((d.0 + center.0) * step, (d.1 + center.1) * step, (d.2 + center.2) * step);
            match cfg.uv_generation {
                UVGeneration::PerVoxel => uv = if reversed { (1., 1.) } else { (0., 1.) },
                _ => {}
            }
            self.mesh_builder.add_vertex(pos, normal, tangent, uv)
        };

        if !reversed {
            self.mesh_builder.add_triangle(d, b, a);
            self.mesh_builder.add_triangle(d, c, b);
        } else {
            self.mesh_builder.add_triangle(a, b, d);
            self.mesh_builder.add_triangle(b, c, d);
        }
    }
}

impl<M> Polygonizer for Cubic<M> 
where
    M: MeshBuilder
{
    fn polygonize<C: Cell>(&mut self, cfg: &Config, cell: &C, lod: u32) {
        let step = 1 << lod;

        for x in 0..(cell.x_size() as isize) {
            for y in 0..(cell.y_size() as isize) {
                for z in 0..(cell.z_size() as isize) {
                    let value = cell.get(x, y, z);
                    if !value {
                        continue;
                    }

                    if cell.get(x - 1, y, z) {
                        self.add_face(cfg, x, y, z, Direction::XMin, step);
                    }
                    if cell.get(x + 1, y, z) {
                        self.add_face(cfg, x, y, z, Direction::XMax, step);
                    }
                    if cell.get(x, y - 1, z) {
                        self.add_face(cfg, x, y, z, Direction::XMin, step);
                    }
                    if cell.get(x, y + 1, z) {
                        self.add_face(cfg, x, y, z, Direction::YMax, step);
                    }
                    if cell.get(x, y, z - 1) {
                        self.add_face(cfg, x, y, z, Direction::ZMin, step);
                    }
                    if cell.get(x, y, z + 1) {
                        self.add_face(cfg, x, y, z, Direction::ZMax, step);
                    }
                }
            }
        }
    }
}
