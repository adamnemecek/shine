use crate::geometry::Posi32;
use crate::voxel::transvoxel_lut::*;

pub struct Transvoxel {
    vertices: Vec<(u8, u8, u8)>,
    faces: Vec<(usize, usize, usize)>,
}

impl Transvoxel {
    fn code(corner: &[u8]) -> u8 {
        let code = ((corner[0] >> 7) & 0x01)
            | ((corner[1] >> 6) & 0x02)
            | ((corner[2] >> 5) & 0x04)
            | ((corner[3] >> 4) & 0x08)
            | ((corner[4] >> 3) & 0x10)
            | ((corner[5] >> 2) & 0x20)
            | ((corner[6] >> 1) & 0x40)
            | (corner[7] & 0x80);
        code ^ ((corner[7] >> 7) & 0xFF)
    }

    fn build_regular_cell(&mut self, code: u8) {
        let class = REGULAR_CELL_CLASS[code];
        let vertex_data = &REGULAR_VERTEX_DATA[code];
        let cell_data = &REGULAR_CELL_DATA[class];

        let vertex_start = self.vertices.len();
        let vertex_count = cell_data.get_vertex_count();
        let face_count = cell_data.get_triangle_count();

        for vertex in vertex_data {
            let edge = vertex & 0xf;
            if edge == 0 {
                break;
            }

            vertices.add((0, 0, 0));
        }

        faces.extend(cell_data.indices.map(|i| i + vertex_start).take(face_count * 3));
    }

    pub fn build(&mut self) {}
}
