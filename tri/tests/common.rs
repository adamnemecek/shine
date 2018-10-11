extern crate shine_tri;

use shine_tri::*;

#[derive(Clone, Debug)]
pub struct Pos(pub f32, pub f32);

impl Position for Pos {
    type Real = f32;

    fn x(&self) -> Self::Real {
        self.0
    }

    fn y(&self) -> Self::Real {
        self.1
    }
}

pub struct SimpleVertex {
    position: Pos,
    face: FaceIndex,
}

impl Default for SimpleVertex {
    fn default() -> SimpleVertex {
        SimpleVertex {
            position: Pos(0., 0.),
            face: FaceIndex::invalid(),
        }
    }
}

impl Vertex for SimpleVertex {
    type Position = Pos;

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

pub struct SimpleFace {
    vertices: [VertexIndex; 3],
    neighbors: [FaceIndex; 3],
    constraints: [bool; 3],
}

impl Default for SimpleFace {
    fn default() -> SimpleFace {
        SimpleFace {
            vertices: [VertexIndex::invalid(); 3],
            neighbors: [FaceIndex::invalid(); 3],
            constraints: [false; 3],
        }
    }
}

impl Face for SimpleFace {
    type Constraint = bool;

    fn vertex(&self, i: Rot3) -> VertexIndex {
        self.vertices[i.0 as usize]
    }

    fn set_vertex(&mut self, i: Rot3, v: VertexIndex) {
        self.vertices[i.0 as usize] = v
    }

    fn get_vertex_index(&self, v: VertexIndex) -> Option<Rot3> {
        self.vertices.iter().position(|&x| x == v).map(|i| Rot3(i as u8))
    }

    fn neighbor(&self, i: Rot3) -> FaceIndex {
        self.neighbors[i.0 as usize]
    }

    fn set_neighbor(&mut self, i: Rot3, f: FaceIndex) {
        self.neighbors[i.0 as usize] = f;
    }

    fn get_neighbor_index(&self, f: FaceIndex) -> Option<Rot3> {
        self.neighbors.iter().position(|&x| x == f).map(|i| Rot3(i as u8))
    }

    fn constraint(&self, i: Rot3) -> Self::Constraint {
        self.constraints[i.0 as usize]
    }

    fn set_constraint(&mut self, i: Rot3, c: Self::Constraint) {
        self.constraints[i.0 as usize] = c
    }
}

pub type SimpleTriGraph = TriGraph<InexactPredicates<Pos>, SimpleVertex, SimpleFace>;
