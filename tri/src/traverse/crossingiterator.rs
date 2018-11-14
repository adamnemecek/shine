use geometry::{CollinearTest, Orientation, Predicates};
use graph::{Face, Vertex};
use indexing::PositionQuery;
use traverse::edgecirculator::EdgeCirculator;
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrossingSide {
    /// crossed edge and query-edge are coincident
    Coincident,
    /// vertex_from is on the positive side of edge
    CCW,
    /// vertex_from is on the negative side of edge
    CW,
    /// End of iteration, end of query (v1) reached
    End,
}

#[derive(Debug, Clone)]
pub struct CrossedEdge {
    pub face: FaceIndex,
    pub edge: Rot3,
    pub vertex_from: VertexIndex,
    pub side_from: CrossingSide,
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
    current: CrossedEdge,
}

impl<'a, PR, V, F> CrossingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Triangulation<PR, V, F>, v0: VertexIndex, v1: VertexIndex) -> CrossingIterator<PR, V, F> {
        assert_eq!(tri.graph.dimension(), 2);
        let face = tri.graph[v0].face();
        let edge = tri.graph[face].get_vertex_index(v0).unwrap();
        CrossingIterator {
            tri,
            v0,
            v1,
            current: CrossedEdge {
                face,
                edge,
                vertex_from: v0,
                side_from: CrossingSide::Coincident,
            },
        }
    }

    /// Find next edge to continue after a coincident edge.
    /// It requires a full circular search around the vertex_from point as the next crossed face might not be adjacent
    /// to the previous face.
    fn next_restart(&self) -> CrossedEdge {
        assert_eq!(self.current.side_from, CrossingSide::Coincident);
        assert_ne!(self.current.vertex_from, self.v1);

        println!("next_restart {:?},{:?}, {:?}", self.v0, self.v1, self.current.vertex_from);

        let mut start_orientation = CrossingSide::Coincident;
        let mut circulator = EdgeCirculator::new(self.tri, self.current.vertex_from);

        loop {
            let vertex = circulator.vertex();
            println!(
                "edge {:?},{:?},{:?}",
                circulator.face(),
                circulator.edge(),
                circulator.vertex()
            );
            if self.tri.graph.is_infinite_vertex(vertex) {
                // skip infinite edges
                circulator.advance_cw();
                continue;
            }

            if vertex == self.v1 {
                return CrossedEdge {
                    face: circulator.face(),
                    edge: circulator.edge(),
                    vertex_from: vertex,
                    side_from: CrossingSide::End,
                };
            }

            let orientation = {
                let p0 = &self.tri.graph[PositionQuery::Vertex(self.v0)];
                let p1 = &self.tri.graph[PositionQuery::Vertex(self.v1)];
                let pos = &self.tri.graph[PositionQuery::Vertex(vertex)];

                let orient = self.tri.predicates.orientation_triangle(p0, p1, pos);
                println!("orient({:?},{:?},{:?}) = {:?}", self.v0, self.v1, vertex, orient);
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
                        return CrossedEdge {
                            face: circulator.face(),
                            edge: circulator.edge(),
                            vertex_from: vertex,
                            side_from: CrossingSide::Coincident,
                        };
                    }
                    CrossingSide::CCW
                } else if orient.is_ccw() {
                    CrossingSide::CCW
                } else {
                    CrossingSide::CW
                }
            };
            println!("start_orientation = {:?}", start_orientation);
            println!("orientation = {:?}", orientation);

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

                return CrossedEdge {
                    face: circulator.face(),
                    edge: circulator.edge().increment(),
                    vertex_from: circulator.vertex(),
                    side_from: CrossingSide::CW,
                };
            } else if start_orientation == CrossingSide::CCW {
                circulator.advance_cw();
            } else {
                assert_eq!(start_orientation, CrossingSide::CW);
                circulator.advance_ccw();
            }
        }
    }

    fn next_crossed(&self) -> CrossedEdge {
        assert!(self.current.side_from != CrossingSide::CCW || self.current.side_from != CrossingSide::CW);
        assert!(self.current.vertex_from != self.v1);

        println!(
            "next_crossed {:?},{:?},{:?}",
            self.current.face, self.current.edge, self.current.vertex_from
        );

        let face = self.tri.graph[self.current.face].neighbor(self.current.edge);
        let vertex_index = self.tri.graph[face].get_neighbor_index(self.current.face).unwrap();
        let vertex = self.tri.graph[face].vertex(vertex_index);

        println!("check {:?},{:?},{:?}", face, vertex_index, vertex);

        if vertex == self.v1 {
            return CrossedEdge {
                face,
                edge: vertex_index.increment(),
                vertex_from: vertex,
                side_from: CrossingSide::End,
            };
        };

        let p0 = &self.tri.graph[PositionQuery::Vertex(self.v0)];
        let p1 = &self.tri.graph[PositionQuery::Vertex(self.v1)];
        let pn = &self.tri.graph[PositionQuery::Vertex(vertex)];
        let orientation = self.tri.predicates.orientation_triangle(p0, p1, pn);
        println!("orientation({:?},{:?},{:?}): {:?}", self.v0, self.v1, vertex, orientation);
        assert!(!orientation.is_collinear(), "next != v1, but v0,v1,next are collinear");
        if orientation.is_ccw() {
            return CrossedEdge {
                face,
                edge: vertex_index.increment(),
                vertex_from: vertex,
                side_from: CrossingSide::CCW,
            };
        } else {
            return CrossedEdge {
                face,
                edge: vertex_index.decrement(),
                vertex_from: vertex,
                side_from: CrossingSide::CW,
            };
        }
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<CrossedEdge> {
        let next = match self.current.side_from {
            CrossingSide::Coincident => self.next_restart(),
            CrossingSide::CCW | CrossingSide::CW => self.next_crossed(),
            CrossingSide::End => return None,
        };

        self.current = next;
        Some(self.current.clone())
    }
}
