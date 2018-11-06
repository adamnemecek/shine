use geometry::{CollinearTest, Orientation, Predicates};
use graph::{Face, Vertex};
use indexing::{IndexGet, PositionQuery, VertexQuery};
use traverse::edgecirculator::EdgeCirculator;
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Side {
    Any, // no side information is know, or the edges are coincident
    CCW,
    CW,
}

pub struct IntersectingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    tri: &'a Triangulation<PR, V, F>,

    v0: VertexIndex,
    v1: VertexIndex,

    vertex: VertexIndex,
    face: FaceIndex,
    edge: Rot3,
    side: Side,
}

impl<'a, PR, V, F> IntersectingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Triangulation<PR, V, F>, v0: VertexIndex, v1: VertexIndex) -> IntersectingIterator<PR, V, F> {
        assert_eq!(tri.graph.dimension(), 2);
        let p0 = &tri.graph[PositionQuery::Vertex(v0)];
        let p1 = &tri.graph[PositionQuery::Vertex(v1)];

        let mut direction = Side::Any;
        let mut circulator = EdgeCirculator::new(tri, v0);
        loop {
            let vertex = circulator.vertex();
            if tri.graph.is_infinite_vertex(vertex) {
                // skip infinite edges
                circulator.advance_cw();
                continue;
            }

            if vertex == v1 {
                return IntersectingIterator {
                    tri,
                    v0,
                    v1,
                    vertex,
                    face: circulator.face(),
                    edge: circulator.edge(),
                    side: Side::Any,
                };
            }

            let pos = &tri.graph[PositionQuery::Vertex(vertex)];
            let orientation = {
                let orient = tri.predicates.orientation_triangle(p0, p1, pos);
                if orient.is_collinear() {
                    let t = tri.predicates.test_collinear_points(p0, p1, pos);

                    assert!(
                        !t.is_first(),
                        "invalid triangulation, p0 == pos; p0 == edge.start; edge.end == p, edge has a zero length",
                    );
                    assert!(
                        !t.is_second(),
                        "invalid triangulation, p1 == pos, but v1 != vertex, distinct vertices with the same position",
                    );
                    assert!(
                        !t.is_after(),
                        "invalid triangulation, collinear, pos is not contained in the (p0,p1) segment",
                    );

                    if t.is_between() {
                        // pe is between p0 and p1
                        return IntersectingIterator {
                            tri,
                            v0,
                            v1,
                            vertex,
                            face: circulator.face(),
                            edge: circulator.edge(),
                            side: Side::Any,
                        };
                    }
                    Side::CCW
                } else if orient.is_ccw() {
                    Side::CCW
                } else {
                    Side::CW
                }
            };

            if direction == Side::Any {
                // "first" loop iteration, find circulating direction
                assert_ne!(orientation, Side::Any);
                direction = orientation;
            }

            if direction != orientation {
                // orientation has changed -> we have the first triangle crossing the edge
                if direction == Side::CW {
                    circulator.advance_cw();
                }

                return IntersectingIterator {
                    tri,
                    v0,
                    v1,
                    vertex: circulator.vertex(),
                    face: circulator.face(),
                    edge: circulator.edge(),
                    side: Side::Any,
                };
            } else if direction == Side::CCW {
                circulator.advance_cw();
            } else {
                assert_ne!(direction, Side::CW);
                circulator.advance_ccw();
            }
        }
    }

    pub fn advance(&mut self) {
        assert!(self.vertex != self.v1);

        let nf = self.tri.graph[self.face].neighbor(self.edge);
        let iv = self.tri.graph[nf].get_neighbor_index(self.face).unwrap();
        self.vertex = self.tri.graph[nf].vertex(iv);
        self.face = nf;

        if self.vertex == self.v1 {
            self.edge = iv.increment();
            self.side = Side::Any;
        } else {
            let p0 = &self.tri.graph[PositionQuery::Vertex(self.v0)];
            let p1 = &self.tri.graph[PositionQuery::Vertex(self.v1)];
            let pn = &self.tri.graph[PositionQuery::Vertex(self.vertex)];
            let orientation = self.tri.predicates.orientation_triangle(p0, p1, pn);
            assert!(!orientation.is_collinear(), "next != v1, but v0,v1,next are collinear");
            if orientation.is_ccw() {
                self.edge = iv.increment();
                self.side = Side::CCW;
            } else {
                self.edge = iv.decrement();
                self.side = Side::CW;
            };
        }
    }

    pub fn has_more(&self) -> bool {
        self.vertex != self.v1
    }

    pub fn vertex(&self) -> VertexIndex {
        self.tri.graph.index_get(VertexQuery::EdgeEnd(self.face, self.edge))
    }

    pub fn face(&self) -> FaceIndex {
        self.face
    }

    pub fn edge(&self) -> Rot3 {
        self.edge
    }

    pub fn side(&self) -> Side {
        self.side
    }
}
