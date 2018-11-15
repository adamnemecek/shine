use geometry::{CollinearTest, Orientation, Predicates};
use graph::{Face, Vertex};
use indexing::PositionQuery;
use std::mem;
use traverse::edgecirculator::EdgeCirculator;
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrossingSide {
    Coincident,
    CCW,
    CW,
    End,
}

#[derive(Debug, Clone)]
pub struct CrossedEdge {
    pub face: FaceIndex,
    pub edge: Rot3,
    pub side: CrossingSide,
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
    fn search_vertex(&self, start_vertex: VertexIndex) -> Option<CrossedEdge> {
        let mut start_orientation = CrossingSide::Coincident;
        let mut circulator = EdgeCirculator::new(self.tri, start_vertex);

        loop {
            let vertex = circulator.vertex();
            if self.tri.graph.is_infinite_vertex(vertex) {
                // skip infinite edges
                circulator.advance_cw();
                continue;
            }

            if vertex == self.v1 {
                return None;
            }

            let orientation = {
                let p0 = &self.tri.graph[PositionQuery::Vertex(self.v0)];
                let p1 = &self.tri.graph[PositionQuery::Vertex(self.v1)];
                let pos = &self.tri.graph[PositionQuery::Vertex(vertex)];

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
                        return Some(CrossedEdge {
                            face: circulator.face(),
                            edge: circulator.edge(),
                            side: CrossingSide::Coincident,
                        });
                    }
                    CrossingSide::CCW
                } else if orient.is_ccw() {
                    CrossingSide::CCW
                } else {
                    CrossingSide::CW
                }
            };

            if start_orientation == CrossingSide::Coincident {
                // "first" loop iteration, find circulating direction
                assert!(orientation == CrossingSide::CCW || orientation == CrossingSide::CW);
                start_orientation = orientation;
            }

            if start_orientation != orientation {
                // orientation has changed -> we have the edge crossing the query
                if orientation == CrossingSide::CCW {
                    // we have just passed our edge, go back
                    circulator.advance_cw();
                }

                return Some(CrossedEdge {
                    face: circulator.face(),
                    edge: circulator.edge().increment(),
                    side: CrossingSide::CW,
                });
            } else if start_orientation == CrossingSide::CCW {
                circulator.advance_cw();
            } else {
                assert_eq!(start_orientation, CrossingSide::CW);
                circulator.advance_ccw();
            }
        }
    }

    /// Find next crossing edge by circulating the edges around the vertex of a face
    fn search_face_vertex(&self, start_face: FaceIndex, start_edge: Rot3) -> Option<CrossedEdge> {
        let start_vertex = self.tri.graph[start_face].vertex(start_edge);
        self.search_vertex(start_vertex)
    }

    /// Find next crossing edge by checking the opposite face.
    fn search_edge(&self, start_face: FaceIndex, start_edge: Rot3) -> Option<CrossedEdge> {
        let face = self.tri.graph[start_face].neighbor(start_edge);
        let vertex_index = self.tri.graph[face].get_neighbor_index(start_face).unwrap();
        let vertex = self.tri.graph[face].vertex(vertex_index);

        if vertex == self.v1 {
            return None;
        };

        let p0 = &self.tri.graph[PositionQuery::Vertex(self.v0)];
        let p1 = &self.tri.graph[PositionQuery::Vertex(self.v1)];
        let pn = &self.tri.graph[PositionQuery::Vertex(vertex)];
        let orientation = self.tri.predicates.orientation_triangle(p0, p1, pn);
        assert!(!orientation.is_collinear(), "next != v1, but v0,v1,next are collinear");
        if orientation.is_ccw() {
            return Some(CrossedEdge {
                face,
                edge: vertex_index.increment(),
                side: CrossingSide::CCW,
            });
        } else {
            return Some(CrossedEdge {
                face,
                edge: vertex_index.decrement(),
                side: CrossingSide::CW,
            });
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
    current: Option<CrossedEdge>,
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

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<CrossedEdge> {
        let next = {
            let current = self.current.as_ref().unwrap();
            let search = &self.search;

            match current.side {
                CrossingSide::Coincident => search.search_face_vertex(current.face, current.edge),
                CrossingSide::CW => search.search_edge(current.face, current.edge),
                CrossingSide::CCW => search.search_edge(current.face, current.edge),
            }
        };

        mem::replace(&mut self.current, next)
    }
}
