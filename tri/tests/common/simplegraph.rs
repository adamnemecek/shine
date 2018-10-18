use shine_tri::*;

#[derive(Clone, Debug)]
pub struct TriPos(pub f32, pub f32);

impl Position for TriPos {
    type Real = f32;

    fn x(&self) -> Self::Real {
        self.0
    }

    fn y(&self) -> Self::Real {
        self.1
    }
}

pub struct TriVertex {
    position: TriPos,
    face: FaceIndex,
}

impl Default for TriVertex {
    fn default() -> TriVertex {
        TriVertex {
            position: TriPos(0., 0.),
            face: invalid_face_index(),
        }
    }
}

impl Vertex for TriVertex {
    type Position = TriPos;

    fn position(&self) -> &Self::Position {
        &self.position
    }

    fn position_mut(&mut self) -> &mut Self::Position {
        &mut self.position
    }

    fn face(&self) -> FaceIndex {
        self.face
    }

    fn set_face(&mut self, face: FaceIndex) {
        self.face = face
    }
}

pub struct TriFace {
    vertices: [VertexIndex; 3],
    neighbors: [FaceIndex; 3],
    constraints: [bool; 3],
}

impl Default for TriFace {
    fn default() -> TriFace {
        TriFace {
            vertices: [invalid_vertex_index(); 3],
            neighbors: [invalid_face_index(); 3],
            constraints: [false; 3],
        }
    }
}

impl Face for TriFace {
    type Constraint = bool;

    fn vertex(&self, i: Rot3) -> VertexIndex {
        self.vertices[i.id() as usize]
    }

    fn set_vertex(&mut self, i: Rot3, v: VertexIndex) {
        self.vertices[i.id() as usize] = v
    }

    fn get_vertex_index(&self, v: VertexIndex) -> Option<Rot3> {
        self.vertices.iter().position(|&x| x == v).map(|i| rot3(i as u8))
    }

    fn neighbor(&self, i: Rot3) -> FaceIndex {
        self.neighbors[i.id() as usize]
    }

    fn set_neighbor(&mut self, i: Rot3, f: FaceIndex) {
        self.neighbors[i.id() as usize] = f;
    }

    fn get_neighbor_index(&self, f: FaceIndex) -> Option<Rot3> {
        self.neighbors.iter().position(|&x| x == f).map(|i| rot3(i as u8))
    }

    fn constraint(&self, i: Rot3) -> Self::Constraint {
        self.constraints[i.id() as usize]
    }

    fn set_constraint(&mut self, i: Rot3, c: Self::Constraint) {
        self.constraints[i.id() as usize] = c
    }
}

pub type SimpleTri = Graph<InexactPredicates32<TriPos>, TriVertex, TriFace>;
