#![allow(dead_code)]

use shine_tri::geometry::Position;
use shine_tri::geometry::{Posf32, Posf64, Posi32, Posi64, Predicatesf32, Predicatesf64, Predicatesi32, Predicatesi64};
use shine_tri::types::{invalid_face_index, invalid_vertex_index, rot3, FaceIndex, Rot3, VertexIndex};
use shine_tri::{Constraint, Face, Vertex};

pub struct SimpleVertex<P>
where
    P: Default + Position,
{
    position: P,
    face: FaceIndex,
}

impl<P> Default for SimpleVertex<P>
where
    P: Default + Position,
{
    fn default() -> SimpleVertex<P> {
        SimpleVertex {
            position: Default::default(),
            face: invalid_face_index(),
        }
    }
}

impl<P> Vertex for SimpleVertex<P>
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

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct SimpleConstraint(pub u16);

impl Default for SimpleConstraint {
    fn default() -> Self {
        SimpleConstraint(0)
    }
}

impl Constraint for SimpleConstraint {
    fn is_constraint(&self) -> bool {
        self.0 != 0
    }
}

impl From<u16> for SimpleConstraint {
    fn from(v: u16) -> SimpleConstraint {
        SimpleConstraint(v)
    }
}

pub struct SimpleFace {
    vertices: [VertexIndex; 3],
    neighbors: [FaceIndex; 3],
    constraints: [SimpleConstraint; 3],
    tag: usize,
}

impl Default for SimpleFace {
    fn default() -> SimpleFace {
        SimpleFace {
            vertices: [invalid_vertex_index(); 3],
            neighbors: [invalid_face_index(); 3],
            constraints: [SimpleConstraint::default(); 3],
            tag: 0,
        }
    }
}

impl Face for SimpleFace {
    type Constraint = SimpleConstraint;

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
        self.constraints[i.id() as usize].0 = c.0;
    }

    fn merge_constraint(&mut self, i: Rot3, c: Self::Constraint) {
        self.constraints[i.id() as usize].0 |= c.0;
    }

    fn tag(&self) -> usize {
        self.tag
    }

    fn set_tag(&mut self, tag: usize) {
        self.tag = tag
    }
}
/*
pub struct SimpleTri<P>
where
    P: Position
{
    phantom: P,
}

impl SimpleTri<P>
where
    P: Position
{
    pub  fn new() -> Trinagulation<>
}
*/

pub type PredicatesPosf32 = Predicatesf32<Posf32>;
pub type PredicatesPosf64 = Predicatesf64<Posf64>;
pub type PredicatesPosi32 = Predicatesi32<Posi32>;
pub type PredicatesPosi64 = Predicatesi64<Posi64>;
