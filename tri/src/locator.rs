use geometry::{CollinearTest, Orientation, Predicates};
use graph::{Face, Graph, Vertex};
use indexing::PositionIndex;
use rand::Rng;
use types::{rot3, FaceIndex, Rot3, VertexIndex};

#[derive(Debug)]
enum ContainmentResult {
    Continue(Rot3),
    Stop(u8),
}

impl ContainmentResult {
    fn set(&mut self, i: Rot3, b: bool) {
        assert!(i.is_valid());
        if b {
            match self {
                &mut ContainmentResult::Stop(ref mut t) => *t |= 1 << i.id(),
                _ => unreachable!(),
            }
        }
    }
}

///Result of a point location query
#[derive(Debug)]
pub enum Location {
    /// Point is inside a triangle
    Face(FaceIndex),

    /// Point is on the edge of a triangle
    Edge(FaceIndex, Rot3),

    /// Point is on the vertex of a triangle
    Vertex(FaceIndex, Rot3),

    /// Triangulation is empty
    Empty,

    /// Point is outside the affine hull of a 0D triangulation (the query point and triangulation forms a segment)
    OutsideAffineHull,

    /// Point is outside the affine hull of a 1D triangulation (the query point and triangulation forms a cw triangle)
    OutsideAffineHullClockwise,

    /// Point is outside the affine hull of a 1D triangulation (the query point and triangulation forms a ccw triangle)
    OutsideAffineHullCounterClockwise,

    /// Point is outside the affine hull of a 2D triangulation, with the given nearest face
    OutsideConvexHull(FaceIndex),
}

pub struct Locator<'a, R, P, V, F>
where
    R: 'a + Rng,
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    tri: &'a Graph<P, V, F>,
    rng: &'a mut R,
    iteration_stochastic_limit: usize,
    start_face_sampling_density: usize,
}

impl<'a, R, P, V, F> Locator<'a, R, P, V, F>
where
    R: 'a + Rng,
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new<'b>(rng: &'b mut R, tri: &'b Graph<P, V, F>) -> Locator<'b, R, P, V, F> {
        Locator {
            tri,
            rng,
            iteration_stochastic_limit: 10,
            start_face_sampling_density: 100,
        }
    }
}

impl<'a, R, P, V, F> Locator<'a, R, P, V, F>
where
    R: 'a + Rng,
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn locate_position(&mut self, p: &P::Position, hint: Option<FaceIndex>) -> Result<Location, String> {
        match self.tri.dimension() {
            -1 => Ok(Location::Empty),
            0 => self.locate_position_dim0(p),
            1 => self.locate_position_dim1(p),
            2 => self.locate_position_dim2(p, hint),
            dim => unreachable!(format!("Invalid dimension: {}", dim)),
        }
    }

    /// Find the location of a point in a single point triangulation (dimension = 0).
    fn locate_position_dim0(&mut self, p: &P::Position) -> Result<Location, String> {
        let tri = self.tri;

        assert!(tri.dimension() == 0);

        // find the (only) finite vertex
        let v0 = {
            let v = VertexIndex(1);
            if !tri.is_infinite_vertex(v) {
                v
            } else {
                VertexIndex(0)
            }
        };

        let p0 = &tri[v0].position();
        if tri.predicates().test_coincident_points(p, p0) {
            let f0 = tri[v0].face();
            Ok(Location::Vertex(f0, tri[f0].get_vertex_index(v0).unwrap()))
        } else {
            Ok(Location::OutsideAffineHull)
        }
    }

    /// Find the location of a point in a straight line strip. (dimension = 1)
    fn locate_position_dim1(&mut self, p: &P::Position) -> Result<Location, String> {
        assert!(self.tri.dimension() == 1);

        // calculate the convex hull of the 1-d mesh
        // the convex hull is a segment made up from the two (finite) neighboring vertices of the infinite vertex

        let vinf = self.tri.infinite_vertex();
        // first point of the convex hull (segments)
        let f0 = self.tri.infinite_face();
        let iv0 = self.tri[f0].get_vertex_index(vinf).unwrap();
        let cp0 = &self.tri[PositionIndex::Face(f0, iv0.mirror(2))];

        // last point of the convex hull (segments)
        let f1 = self.tri[f0].neighbor(iv0.mirror(2));
        let iv1 = self.tri[f1].get_vertex_index(vinf).unwrap();
        let cp1 = &self.tri[PositionIndex::Face(f1, iv1.mirror(2))];

        match self.tri.predicates().orientation(cp0, cp1, p) {
            Orientation::Clockwise => Ok(Location::OutsideAffineHullClockwise),
            Orientation::CounterClockwise => Ok(Location::OutsideAffineHullCounterClockwise),
            _ => {
                // point is on the line
                let t = self.tri.predicates().test_collinear_points(cp0, cp1, p);
                match t {
                    CollinearTest::Before => Ok(Location::OutsideConvexHull(f0)),
                    CollinearTest::First => Ok(Location::Vertex(f0, iv0.mirror(2))),
                    CollinearTest::Second => Ok(Location::Vertex(f1, iv1.mirror(2))),
                    CollinearTest::After => Ok(Location::OutsideConvexHull(f1)),
                    CollinearTest::Between => {
                        // Start from an infinite face(f0) and advance to the neighboring segments while the
                        // the edge(face) containing the point is not found
                        let mut prev = f0;
                        let mut dir = iv0;
                        loop {
                            let cur = self.tri[prev].neighbor(dir);
                            assert!(self.tri.is_finite_face(cur));

                            let p0 = &self.tri[PositionIndex::Face(cur, rot3(0))];
                            let p1 = &self.tri[PositionIndex::Face(cur, rot3(1))];

                            let t = self.tri.predicates().test_collinear_points(p0, p1, p);
                            match t {
                                CollinearTest::First => return Ok(Location::Vertex(cur, rot3(0))), // identical to p0
                                CollinearTest::Second => return Ok(Location::Vertex(cur, rot3(1))), // identical to p1
                                CollinearTest::Between => return Ok(Location::Edge(cur, rot3(2))), // inside the (p0,p1) segment

                                _ => {
                                    // advance to the next edge
                                    let vi = self.tri[cur].get_neighbor_index(prev).unwrap();
                                    prev = cur;
                                    dir = vi.mirror(2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    /// Return some random (finite) vertex from the triangulation
    fn get_random_finite_vertex(&mut self) -> VertexIndex {
        let cnt = self.tri.vertex_count();
        assert!(cnt >= 2, "Triangulation is empty");

        let cnt = cnt - 1;
        let rnd = self.rng.gen_range(0, cnt);
        if rnd < self.tri.infinite_vertex().0 {
            VertexIndex(rnd)
        } else {
            VertexIndex(rnd + 1)
        }
    }

    /// Ques point location by finding the nearest vertex to the point from a random sampling of vertices
    fn guess_start_vertex(&mut self, sample_count: usize, p: &P::Position) -> VertexIndex {
        if sample_count <= 0 {
            self.tri.infinite_vertex()
        } else {
            let mut min_vert = self.get_random_finite_vertex();
            let mut min_dist = self
                .tri
                .predicates()
                .distance_points(p, &self.tri[PositionIndex::Vertex(min_vert)]);
            for _ in 0..sample_count {
                let mut vert = self.get_random_finite_vertex();
                let dist = self
                    .tri
                    .predicates()
                    .distance_points(p, &self.tri[PositionIndex::Vertex(vert)]);
                if dist < min_dist {
                    min_vert = vert;
                    min_dist = dist;
                }
            }
            min_vert
        }
    }

    /// Test which halfspace contains the p point.
    fn test_containment_face(&mut self, p: &P::Position, f: FaceIndex) -> ContainmentResult {
        let p0 = &self.tri[PositionIndex::Face(f, rot3(0))];
        let p1 = &self.tri[PositionIndex::Face(f, rot3(1))];
        let p2 = &self.tri[PositionIndex::Face(f, rot3(2))];

        let e01 = self.tri.predicates().orientation(p0, p1, p);
        if e01 == Orientation::Clockwise {
            return ContainmentResult::Continue(rot3(2));
        }

        let e20 = self.tri.predicates().orientation(p2, p0, p);
        if e20 == Orientation::Clockwise {
            return ContainmentResult::Continue(rot3(1));
        }

        let e12 = self.tri.predicates().orientation(&p1, &p2, p);
        if e12 == Orientation::Clockwise {
            return ContainmentResult::Continue(rot3(0));
        }

        let mut test = ContainmentResult::Stop(0);
        test.set(rot3(2), e01 == Orientation::Collinear);
        test.set(rot3(0), e12 == Orientation::Collinear);
        test.set(rot3(1), e20 == Orientation::Collinear);
        println!("test {:?}", test);
        test
    }

    /// Test the containment of the p position for the (a,b) and (b,c) sides in this order
    fn test_containment_ab_bc(&mut self, p: &P::Position, f: FaceIndex, a: Rot3, b: Rot3, c: Rot3) -> ContainmentResult {
        let pa = &self.tri[PositionIndex::Face(f, a)];
        let pb = &self.tri[PositionIndex::Face(f, b)];
        let ab = self.tri.predicates().orientation(&pa, &pb, p);
        if ab == Orientation::Clockwise {
            ContainmentResult::Continue(c)
        } else {
            let pc = &self.tri[PositionIndex::Face(f, c)];
            let bc = self.tri.predicates().orientation(pb, pc, p);
            if bc == Orientation::Clockwise {
                ContainmentResult::Continue(a)
            } else {
                let mut test = ContainmentResult::Stop(0);
                test.set(c, ab == Orientation::Collinear);
                test.set(a, bc == Orientation::Collinear);
                test
            }
        }
    }

    // Test the containment of the p position for the (b,c) and (a,b) sides in this order
    fn test_containment_bc_ab(&mut self, p: &P::Position, f: FaceIndex, a: Rot3, b: Rot3, c: Rot3) -> ContainmentResult {
        let pb = &self.tri[PositionIndex::Face(f, b)];
        let pc = &self.tri[PositionIndex::Face(f, c)];
        let bc = self.tri.predicates().orientation(&pb, &pc, p);
        if bc == Orientation::Clockwise {
            ContainmentResult::Continue(a)
        } else {
            let pa = &self.tri[PositionIndex::Face(f, a)];
            let ab = self.tri.predicates().orientation(&pa, &pb, p);
            if ab == Orientation::Clockwise {
                ContainmentResult::Continue(c)
            } else {
                let mut test = ContainmentResult::Stop(0);
                test.set(c.into(), ab == Orientation::Collinear);
                test.set(a.into(), bc == Orientation::Collinear);
                test
            }
        }
    }

    // Finds the location of a point in a non-degenerate triangulation. (dimension = 2)
    fn locate_position_dim2(&mut self, p: &P::Position, hint: Option<FaceIndex>) -> Result<Location, String> {
        assert_eq!(self.tri.dimension(), 2);

        let start = match hint {
            Some(f) => f,
            None => {
                let cnt = self.tri.vertex_count();
                let sample_count = (cnt + self.start_face_sampling_density - 1) / self.start_face_sampling_density;
                let v = self.guess_start_vertex(sample_count, p);
                let start = self.tri[v].face();
                match self.tri[start].get_vertex_index(self.tri.infinite_vertex()) {
                    None => start,                          // finite face
                    Some(i) => self.tri[start].neighbor(i), // the opposite face to an infinite vertex is finite
                }
            }
        };
        assert!(self.tri.is_finite_face(start));

        let mut prev = FaceIndex::invalid();
        let mut cur = start;
        let mut count = 0;

        loop {
            println!("cur: {:?}", cur);
            if self.tri.is_infinite_face(cur) {
                return Ok(Location::OutsideConvexHull(cur));
            }

            let from = self.tri[cur].get_neighbor_index(prev);
            println!("from: {:?}", from);
            let order = if count < self.iteration_stochastic_limit {
                count % 2 == 0
            } else {
                self.rng.gen()
            };

            let test_result = match (from.map(|r| r.id()), order) {
                (None, _) => self.test_containment_face(p, cur),
                (Some(0), true) => self.test_containment_ab_bc(p, cur, rot3(2), rot3(0), rot3(1)),
                (Some(0), false) => self.test_containment_bc_ab(p, cur, rot3(2), rot3(0), rot3(1)),
                (Some(1), true) => self.test_containment_ab_bc(p, cur, rot3(0), rot3(1), rot3(2)),
                (Some(1), false) => self.test_containment_bc_ab(p, cur, rot3(0), rot3(1), rot3(2)),
                (Some(2), true) => self.test_containment_ab_bc(p, cur, rot3(1), rot3(2), rot3(0)),
                (Some(2), false) => self.test_containment_bc_ab(p, cur, rot3(1), rot3(2), rot3(0)),
                (Some(i), _) => unreachable!(format!("Invalid index: {:?}", i)),
            };
            println!("test_result: {:?}", test_result);

            match test_result {
                ContainmentResult::Continue(r) => {
                    prev = cur;
                    cur = self.tri[cur].neighbor(r);
                    count += 1;
                }

                ContainmentResult::Stop(0) => return Ok(Location::Face(cur)),
                ContainmentResult::Stop(1) => return Ok(Location::Edge(cur, rot3(0))), // only on 0 edge
                ContainmentResult::Stop(2) => return Ok(Location::Edge(cur, rot3(1))), // only on 1 edge
                ContainmentResult::Stop(4) => return Ok(Location::Edge(cur, rot3(2))), // only on 2 edge
                ContainmentResult::Stop(6) => return Ok(Location::Vertex(cur, rot3(0))), //both on 1,2 edge
                ContainmentResult::Stop(5) => return Ok(Location::Vertex(cur, rot3(1))), //both on 0,2 edge
                ContainmentResult::Stop(3) => return Ok(Location::Vertex(cur, rot3(2))), //both on 0,1 edge

                ContainmentResult::Stop(e) => unreachable!("Invalid test_result: {}", e),
            }
        }
    }
}
