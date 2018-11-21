use geometry::{CollinearTest, Orientation, OrientationType, Predicates};
use graph::{Face, Vertex};
use indexing::VertexQuery;
use std::mem;
use traverse::edgecirculator::EdgeCirculator;
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

#[derive(Clone, Debug)]
pub enum Crossing {
    Start { face: FaceIndex, edge: Rot3 },
    CWEdge { face: FaceIndex, edge: Rot3 },
    CCWEdge { face: FaceIndex, edge: Rot3 },
    Vertex { face: FaceIndex, vertex: Rot3 },
    End { face: FaceIndex, vertex: Rot3 },
}

struct CrossingSearch<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    tri: &'a Triangulation<PR, V, F>,
    v0: VertexIndex,
    v1: VertexIndex,
}

impl<'a, PR, V, F> CrossingSearch<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
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
                        return Crossing::Start {
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

                return Crossing::CWEdge {
                    face: circulator.face(),
                    edge: circulator.edge().increment(),
                };
            } else if start_orientation == OrientationType::CCW {
                circulator.advance_cw();
            } else {
                assert_eq!(start_orientation, OrientationType::CW);
                circulator.advance_ccw();
            }
        }
    }

    /// Find next crossing edge by circulating the edges around the vertex of a face
    fn search_face_vertex(&self, start_face: FaceIndex, start_edge: Rot3) -> Crossing {
        let start_vertex = self.tri.graph[start_face].vertex(start_edge);
        self.search_vertex(start_vertex)
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
        assert!(!orientation.is_collinear(), "next != v1, but v0,v1,next are collinear");
        if orientation.is_ccw() {
            return Crossing::CCWEdge {
                face,
                edge: vertex_index.increment(),
            };
        } else {
            return Crossing::CWEdge {
                face,
                edge: vertex_index.decrement(),
            };
        }
    }
}

pub struct CrossingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    search: CrossingSearch<'a, PR, V, F>,
    current: Crossing,
}

impl<'a, PR, V, F> CrossingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Triangulation<PR, V, F>, v0: VertexIndex, v1: VertexIndex) -> CrossingIterator<PR, V, F> {
        assert_eq!(tri.graph.dimension(), 2);
        let search = CrossingSearch { tri, v0, v1 };
        let current = search.search_vertex(v0);
        CrossingIterator { search, current }
    }

    pub fn advance(&mut self) -> Crossing {
        unimplemented!()
        /*let next = {
            let current = self.current.as_ref().unwrap();
            let search = &self.search;
        
            match current {
                Crossing::Start { ref face, ref vertex } => search.search_face_vertex(face, vertex),
                Crossing::Vertex { ref face, ref vertex } => search.search_face_vertex(face, vertex),
                Crossing::CWEdge { ref face, ref edge } => search.search_edge(face, edge),
                Crossing::CCWEdge { ref face, ref edge } => search.search_edge(face, edge),
                Crossing::End {} => panic!("advance beyond end"),
            }
        };
        
        mem::replace(&mut self.current, next)*/
    }
}
