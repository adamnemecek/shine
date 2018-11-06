use geometry::{CollinearTest, Orientation, Predicates};
use graph::{Face, Vertex};
use indexing::PositionQuery;
use std::mem;
use traverse::edgecirculator::EdgeCirculator;
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrossingSide {
    Start,
    CCW,
    CW,
    End,
}

pub struct CrossedFace {
    pub vertex: VertexIndex,
    pub face: FaceIndex,
    pub edge: Rot3,
    pub side: CrossingSide,
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
    next: Option<CrossedFace>,
}

impl<'a, PR, V, F> CrossingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Triangulation<PR, V, F>, v0: VertexIndex, v1: VertexIndex) -> CrossingIterator<PR, V, F> {
        assert_eq!(tri.graph.dimension(), 2);
        let mut it = CrossingIterator { tri, v0, v1, next: None };
        it.init();
        it
    }

    fn init(&mut self) {
        let mut direction = CrossingSide::Start;
        let mut circulator = EdgeCirculator::new(self.tri, self.v0);

        self.next = loop {
            let vertex = circulator.vertex();
            if self.tri.graph.is_infinite_vertex(vertex) {
                // skip infinite edges
                circulator.advance_cw();
                continue;
            }

            if vertex == self.v1 {
                break Some(CrossedFace {
                    vertex,
                    face: circulator.face(),
                    edge: circulator.edge(),
                    side: CrossingSide::End,
                });
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
                        break Some(CrossedFace {
                            vertex,
                            face: circulator.face(),
                            edge: circulator.edge(),
                            side: CrossingSide::Start,
                        });
                    }
                    CrossingSide::CCW
                } else if orient.is_ccw() {
                    CrossingSide::CCW
                } else {
                    CrossingSide::CW
                }
            };

            if direction == CrossingSide::Start {
                // "first" loop iteration, find circulating direction
                assert!(orientation == CrossingSide::CCW || orientation == CrossingSide::CW);
                direction = orientation;
            }

            if direction != orientation {
                // orientation has changed -> we have the first triangle crossing the edge
                if direction == CrossingSide::CW {
                    circulator.advance_cw();
                }

                break Some(CrossedFace {
                    vertex: circulator.vertex(),
                    face: circulator.face(),
                    edge: circulator.edge(),
                    side: CrossingSide::Start,
                });
            } else if direction == CrossingSide::CCW {
                circulator.advance_cw();
            } else {
                assert_ne!(direction, CrossingSide::CW);
                circulator.advance_ccw();
            }
        };
    }

    pub fn next(&mut self) -> Option<CrossedFace> {
        let cur = mem::replace(&mut self.next, None);
        if let Some(ref cur) = cur {
            if cur.vertex != self.v1 {
                let face = self.tri.graph[cur.face].neighbor(cur.edge);
                let vertex_index = self.tri.graph[face].get_neighbor_index(cur.face).unwrap();
                let vertex = self.tri.graph[face].vertex(vertex_index);
                if vertex == self.v1 {
                    self.next = Some(CrossedFace {
                        vertex,
                        face,
                        edge: vertex_index.increment(),
                        side: CrossingSide::End,
                    });
                } else {
                    let p0 = &self.tri.graph[PositionQuery::Vertex(self.v0)];
                    let p1 = &self.tri.graph[PositionQuery::Vertex(self.v1)];
                    let pn = &self.tri.graph[PositionQuery::Vertex(cur.vertex)];
                    let orientation = self.tri.predicates.orientation_triangle(p0, p1, pn);
                    assert!(!orientation.is_collinear(), "next != v1, but v0,v1,next are collinear");
                    if orientation.is_ccw() {
                        self.next = Some(CrossedFace {
                            vertex,
                            face,
                            edge: vertex_index.increment(),
                            side: CrossingSide::CCW,
                        });
                    } else {
                        self.next = Some(CrossedFace {
                            vertex,
                            face,
                            edge: vertex_index.decrement(),
                            side: CrossingSide::CW,
                        });
                    }
                }
            }
        }

        cur
    }
}
