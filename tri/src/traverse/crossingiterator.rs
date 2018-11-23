use geometry::{CollinearTest, Orientation, OrientationType, Predicates};
use graph::{Face, Vertex};
use indexing::VertexQuery;
use std::mem;
use traverse::edgecirculator::EdgeCirculator;
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Crossing {
    Start { face: FaceIndex, vertex: Rot3 },
    End { face: FaceIndex, vertex: Rot3 },
    CoincidentEdge { face: FaceIndex, edge: Rot3 },
    PositiveEdge { face: FaceIndex, edge: Rot3 },
    NegativeEdge { face: FaceIndex, edge: Rot3 },
}

pub struct CrossingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    tri: &'a Triangulation<PR, V, F>,
    v0: VertexIndex,
    v1: VertexIndex,
    current: Option<Crossing>,
}

impl<'a, PR, V, F> CrossingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Triangulation<PR, V, F>, v0: VertexIndex, v1: VertexIndex) -> CrossingIterator<PR, V, F> {
        assert_eq!(tri.graph.dimension(), 2);
        assert_ne!(v0, v1);
        assert!(tri.graph.is_finite_vertex(v0));
        assert!(tri.graph.is_finite_vertex(v1));

        let mut iter = CrossingIterator {
            tri,
            v0,
            v1,
            current: None,
        };
        iter.current = Some(iter.search_vertex(iter.v0));
        iter
    }

    fn advance(&mut self) -> Option<Crossing> {
        let next = match self.current {
            None => None,
            Some(Crossing::Start { face, vertex }) => Some(self.search_edge(face, vertex)),
            Some(Crossing::End { face, vertex }) => self.search_face_vertex(face, vertex),
            Some(Crossing::CoincidentEdge { face, edge }) => self.search_face_vertex(face, edge.decrement()),
            Some(Crossing::PositiveEdge { face, edge }) => Some(self.search_edge(face, edge)),
            Some(Crossing::NegativeEdge { face, edge }) => Some(self.search_edge(face, edge)),
        };

        mem::replace(&mut self.current, next)
    }

    /// Find next crossing edge by circulating the edges around a vertex.
    fn search_vertex(&self, start_vertex: VertexIndex) -> Crossing {
        let mut start_orientation = OrientationType::Collinear;
        let mut circulator = EdgeCirculator::new(self.tri, start_vertex);

        loop {
            let vertex = circulator.end_vertex();
            if self.tri.graph.is_infinite_vertex(vertex) {
                // skip infinite edges
                circulator.advance_cw();
                continue;
            }

            if vertex == self.v1 {
                return Crossing::End {
                    face: circulator.face(),
                    vertex: circulator.edge().decrement(),
                };
            }

            let orientation = {
                let p0 = &self.tri.pos(self.v0);
                let p1 = &self.tri.pos(self.v1);
                let pos = &self.tri.pos(vertex);

                let orient = self.tri.predicates.orientation_triangle(p0, p1, pos);
                if orient.is_collinear() {
                    let t = self.tri.predicates.test_collinear_points(p0, p1, pos);
                    if t.is_first() {
                        panic!("invalid triangulation, p0 == pos; p0 == edge.start; edge.end == p, edge has a zero length");
                    } else if t.is_second() {
                        panic!("invalid triangulation, p1 == pos, but v1 != vertex, distinct vertices with the same position");
                    } else if t.is_after() {
                        panic!("invalid triangulation, collinear, pos is not contained in the (p0,p1) segment");
                    } else if t.is_between() {
                        // pe is between p0 and p1
                        return Crossing::CoincidentEdge {
                            face: circulator.face(),
                            edge: circulator.edge(),
                        };
                    }
                    OrientationType::CCW
                } else if orient.is_ccw() {
                    OrientationType::CCW
                } else {
                    OrientationType::CW
                }
            };

            if start_orientation == OrientationType::Collinear {
                // "first" loop iteration, find circulating direction
                assert!(orientation == OrientationType::CCW || orientation == OrientationType::CW);
                start_orientation = orientation;
            }

            if start_orientation != orientation {
                // orientation has changed -> we have the edge crossing the query
                if orientation == OrientationType::CCW {
                    // we have just passed our edge, go back
                    circulator.advance_cw();
                }

                return Crossing::Start {
                    face: circulator.face(),
                    vertex: circulator.edge().increment(),
                };
            } else if start_orientation == OrientationType::CCW {
                circulator.advance_cw();
            } else {
                assert_eq!(start_orientation, OrientationType::CW);
                circulator.advance_ccw();
            }
        }
    }

    /// Find next crossing edge by checking the opposite face.
    fn search_edge(&self, start_face: FaceIndex, start_edge: Rot3) -> Crossing {
        let face = self.tri.graph[start_face].neighbor(start_edge);
        let vertex_index = self.tri.graph[face].get_neighbor_index(start_face).unwrap();
        let vertex = self.tri.graph[face].vertex(vertex_index);

        if vertex == self.v1 {
            return Crossing::End {
                face,
                vertex: vertex_index,
            };
        };

        let p0 = &self.tri.pos(self.v0);
        let p1 = &self.tri.pos(self.v1);
        let pn = &self.tri.pos(vertex);
        let orientation = self.tri.predicates.orientation_triangle(p0, p1, pn);
        if orientation.is_collinear() {
            Crossing::End {
                face,
                vertex: vertex_index,
            }
        } else if orientation.is_ccw() {
            Crossing::NegativeEdge {
                face,
                edge: vertex_index.increment(),
            }
        } else {
            Crossing::PositiveEdge {
                face,
                edge: vertex_index.decrement(),
            }
        }
    }

    /// Find next crossing edge by circulating the edges around the vertex of a face
    fn search_face_vertex(&self, start_face: FaceIndex, start_edge: Rot3) -> Option<Crossing> {
        let start_vertex = self.tri.graph[start_face].vertex(start_edge);
        if start_vertex == self.v1 {
            None
        } else {
            Some(self.search_vertex(start_vertex))
        }
    }
}

impl<'a, PR, V, F> Iterator for CrossingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    type Item = Crossing;

    fn next(&mut self) -> Option<Self::Item> {
        self.advance()
    }
}
