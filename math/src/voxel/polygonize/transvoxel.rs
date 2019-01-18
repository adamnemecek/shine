use crate::voxel::polygonize::transvoxel_lut::*;
use crate::voxel::polygonize::{Config, Direction, Mesh, Polygonizer, UVGeneration};
use crate::voxel::Cell;

fn lerp(x: f32, y: f32, w: f32) -> f32 {
    x * (1. - w) + y * w
}

pub struct Transvoxel<'a, M>
where
    M: Mesh,
{
    mesh: &'a mut M,
    config: Config,
}

impl<'a, M> Transvoxel<'a, M>
where
    M: Mesh,
{
    pub fn new<'b>(mesh: &'b mut M) -> Transvoxel<'b, M> {
        Transvoxel {
            mesh,
            config: Config::default(),
        }
    }
}

impl<'a, M> Polygonizer for Transvoxel<'a, M>
where
    M: Mesh,
{
    fn polygonize<C: Cell>(&mut self, cell: &C) {
        let (sx, sy, sz) = cell.resolution();
        let lod = cell.lod();
        let step = 1 << lod;

        for x in 0isize..(sx as isize) {
            for y in 0isize..(sy as isize) {
                for z in 0isize..(sz as isize) {
                    let values = [
                        cell.get(0, x, y, z) ,
                        cell.get(0, x + 1, y, z),
                        cell.get(0, x, y + 1, z),
                        cell.get(0, x + 1, y + 1, z),
                        cell.get(0, x, y, z + 1),
                        cell.get(0, x + 1, y, z + 1),
                        cell.get(0, x, y + 1, z + 1),
                        cell.get(0, x + 1, y + 1, z + 1),
                    ];

                    let caseCode = ((values[0] as u32) << 0)
                        | ((values[1] as u32)<< 1)
                        | ((values[2] as u32)<< 2)
                        | ((values[3] as u32)<< 3)
                        | ((values[4] as u32)<< 4)
                        | ((values[5] as u32)<< 5)
                        | ((values[6] as u32)<< 6)
                        | ((values[7] as u32)<< 7);

                    if caseCode == 0 || caseCode == 255 {
                        // empty or full cell
                        continue;
                    }

                    let cellClass = REGULAR_CELL_CLASS[caseCode as usize];
                    let vertexData = REGULAR_VERTEX_DATA[caseCode as usize];
                    let cellData = REGULAR_CELL_DATA[cellClass as usize];

                    // vertices
                    let mut localVertexIndices = [0; 12];
                    let mut localVertexCount = 0;
                    let positions = [(0.0f32, 0.0f32, 0.0f32); 12];
                    let normals = [(0.0f32, 0.0f32, 0.0f32); 12];
                    let uvs = [(0.0f32, 0.0f32); 12];

                    for vi in (0..cellData.get_vertex_count()) {
                        //int VertexIndex = -2;
                        let edgeCode = vertexData[vi];

                        // A: low point / B: high point
                        let valueIndexA = (edgeCode >> 4) & 0x0F;
                        let valueIndexB = edgeCode & 0x0F;

                        let valueA = values[valueIndexA as usize];
                        let valueB = values[valueIndexB as usize];
                        assert!(valueA != valueB);

                        let edgeIndex = ((edgeCode >> 8) & 0x0F);
                        assert!(edgeIndex >= 1 && edgeIndex < 4);

                        // Direction to go to use an already created vertex:
                        // first bit:  x is different
                        // second bit: y is different
                        // third bit:  z is different
                        // fourth bit: vertex isn't cached
                        let cacheDirection = edgeCode >> 12;

                        if !valueA {
                            edgeIndex = 0;
                            cacheDirection = valueIndexA ^ 7;
                        }
                        if !valueB {
                            assert!(valueA);
                            edgeIndex = 0;
                            cacheDirection = valueIndexB ^ 7;
                        }

                        // Compute vertex

                        let positionA = (
                            ((x + (valueIndexA & 0x01)) * step) as f32,
                            (y + ((valueIndexA & 0x02) >> 1)) * step as f32,
                            (z + ((valueIndexA & 0x04) >> 2)) * step as f32,
                        );
                        let positionB = (
                            ((x + (valueIndexB & 0x01)) * step) as f32,
                            ((y + ((valueIndexB & 0x02) >> 1)) * step) as f32,
                            ((z + ((valueIndexB & 0x04) >> 2)) * step) as f32,
                        );

                        let position = if edgeIndex == 0 {
                            if !valueA {
                                positionA
                            } else {
                                assert!(!valueB);
                                positionB
                            }
                        } else if lod == 0 {
                            // Full resolution
                            let alpha = 0.5; //ValueAtA.ThisDividedByThisMinusA(ValueAtB, bSuccess);

                            match edgeIndex {
                                2 => (lerp(positionA.0, positionB.0, alpha), positionA.1, positionA.2), // X
                                1 => (positionA.0, lerp(positionA.1, positionB.1, alpha), positionA.2), // Y
                                3 => (positionA.0, positionA.1, lerp(positionA.2, positionB.2, alpha)), // Z
                                _ => unreachable!(),
                            }
                        } else {
                            unimplemented!()
                        };

                        assert!((!valueB && valueIndexB == 7) == !cacheDirection);
                        assert!(cacheDirection || edgeIndex == 0);

                        // Save vertex if not on edge
                        /*if (CacheDirection & 0x08 || !CacheDirection) // ValueAtB.IsNull() && LocalIndexB == 7 => !CacheDirection
                        {
                            GetCurrentCache()[GetCacheIndex(EdgeIndex, LX, LY)] = VertexIndex;
                        }*/
                        positions[localVertexCount] = position;
                        localVertexIndices[vi] = localVertexCount;
                        localVertexCount += 1;
                    }

                    // triangle
                    let mut localIndices = [0; 16];
                    let mut localIndexCount = 0;
                    for ti in 0..cellData.get_triangle_count() {
                        let a = localVertexIndices[cellData.indices[3 * ti]];
                        let b = localVertexIndices[cellData.indices[3 * ti + 1]];
                        let c = localVertexIndices[cellData.indices[3 * ti + 2]];

                        localIndices[localIndexCount] = a;
                        localIndexCount += 1;
                        localIndices[localIndexCount] = b;
                        localIndexCount += 1;
                        localIndices[localIndexCount] = c;
                        localIndexCount += 1;

                        /*if (NormalConfig == EVoxelNormalConfig::MeshNormal)
                        {
                            FVector Normal = FVector::CrossProduct(C.Position - A.Position, B.Position - A.Position).GetSafeNormal();
                            A.NormalSum += Normal;
                            B.NormalSum += Normal;
                            C.NormalSum += Normal;
                        }*/
                    }

                    let mut vertexIndices = [0; 12];
                    for vi in 0..localVertexCount {
                        vertexIndices[vi] = self.mesh.add_vertex(positions[vi], (0., 1., 0.), (0., 0., 0.), (0., 0.));
                    }
                    self.mesh
                        .add_indices(localIndices[0..localIndexCount].map(|i| vertexIndices[i]));
                }
            }
        }
    }
}
