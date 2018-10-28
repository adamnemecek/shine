use common::position::{Posf32, Posf64, Posi32, Posi64};
use shine_tri::geometry::{Position, Predicatesf32, Predicatesf64, Predicatesi32, Predicatesi64};
use shine_tri::types::{invalid_face_index, invalid_vertex_index, rot3, FaceIndex, Rot3, VertexIndex};
use shine_tri::{Face, Graph, Vertex};

pub struct TriVertex<P>
where
    P: Default + Position,
{
    position: P,
    face: FaceIndex,
}

impl<P> Default for TriVertex<P>
where
    P: Default + Position,
{
    fn default() -> TriVertex<P> {
        TriVertex {
            position: Default::default(),
            face: invalid_face_index(),
        }
    }
}

impl<P> Vertex for TriVertex<P>
where
    P: Default + Position,
{
    type Position = P;

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

type SimpleTri<P, PR> = Graph<PR, TriVertex<P>, TriFace>;

pub type SimpleTrif32 = SimpleTri<Posf32, Predicatesf32<Posf32>>;
pub type SimpleTrif64 = SimpleTri<Posf64, Predicatesf64<Posf64>>;
pub type SimpleTrii32 = SimpleTri<Posi32, Predicatesi32<Posi32>>;
pub type SimpleTrii64 = SimpleTri<Posi64, Predicatesi64<Posi64>>;
