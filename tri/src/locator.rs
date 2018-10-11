use geometry::{CollinearTest, Orientation, Predicates};
use rand::{self, Rng};
use triangulation::{Face, TriGraph, Vertex};
use types::{FaceIndex, Rot3, VertexIndex};

enum ContainmentResult {
    Continue(Rot3),
    Stop(u8),
}

impl ContainmentResult {
    fn set(&mut self, i: Rot3, b: bool) {
        assert!(i.is_valid());
        if b {
            match self {
                &mut ContainmentResult::Stop(ref mut t) => *t |= 1 << i.0,
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
    tri: &'a TriGraph<P, V, F>,
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
    pub fn new<'b>(rng: &'b mut R, tri: &'b TriGraph<P, V, F>) -> Locator<'b, R, P, V, F> {
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
            //2 => self.locate_position_dim2(p, hint),
            dim => unreachable!(format!("Invalid dimension: {}", dim)),
        }
    }

    /// Return some random (finite) vertex from the triangulation
    pub fn get_random_finite_vertex(&mut self) -> Result<VertexIndex, String> {
        let cnt = self.tri.vertex_count();
        if cnt < 2 {
            return Err("Triangulation is empty".into());
        }

        let cnt = cnt - 1;
        let rnd = self.rng.gen_range(0, cnt);
        if rnd < self.tri.infinite_vertex().0 {
            Ok(VertexIndex(rnd))
        } else {
            Ok(VertexIndex(rnd + 1))
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
        let cp0 = {
            let v = self.tri[f0].vertex(iv0.mirror(2));
            self.tri[v].position()
        };

        // last point of the convex hull (segments)
        let f1 = self.tri[f0].neighbor(iv0.mirror(2));
        let iv1 = self.tri[f1].get_vertex_index(vinf).unwrap();
        let cp1 = {
            let v = self.tri[f1].vertex(iv1.mirror(2));
            self.tri[v].position()
        };

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

                            let p0 = {
                                let v = self.tri[cur].vertex(Rot3(0));
                                self.tri[v].position()
                            };
                            let p1 = {
                                let v = self.tri[cur].vertex(Rot3(1));
                                self.tri[v].position()
                            };

                            let t = self.tri.predicates().test_collinear_points(p0, p1, p);
                            match t {
                                CollinearTest::First => {
                                    // identical to p0
                                    break Ok(Location::Vertex(cur, Rot3(0)));
                                }
                                CollinearTest::Second => {
                                    // identical to p1
                                    break Ok(Location::Vertex(cur, Rot3(1)));
                                }
                                CollinearTest::Between => {
                                    // inside the (p0,p1) segment
                                    break Ok(Location::Edge(cur, Rot3(2)));
                                }
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

    /*/// Ques point location by finding the nearest vertex to the point from a random sampling of vertices
    fn guess_start_vertex(&mut self, sample_count: usize, p: &T::Position) -> VertexIndex {
        if sample_count <= 0 {
            self.tri.get_infinite_vertex()
        } else {
            let mut min_vert = self.get_random_finite_vertex();
            let mut min_dist = self.tri.predicates.distance_points(p, self.tri.get_vertex_position(min_vert));
            for _ in 0..sample_count {
                let mut vert = self.get_random_finite_vertex();
                let dist = self.tri.predicates.distance_points(p, self.tri.get_vertex_position(vert));
                if dist < min_dist {
                    min_vert = vert;
                    min_dist = dist;
                }
            }
            min_vert
        }
    }

    /// Test the containment of the p position for the (a,b) and (b,c) sides in this order
    fn test_containment_ab_bc(&mut self, p: &T::Position, f: FaceIndex, a: Rot3, b: Rot3, c: Rot3) -> ContainmentResult {
        let tri = &self.tri;
        let pa = tri.get_face_vertex_position(f, a);
        let pb = tri.get_face_vertex_position(f, b);
        let ab = tri.predicates.orientation(pa, pb, p);
        if ab == Orientation::Clockwise {
            ContainmentResult::Continue(c)
        } else {
            let pc = tri.get_face_vertex_position(f, c);
            let bc = tri.predicates.orientation(pb, pc, p);
            if bc == Orientation::Clockwise {
                ContainmentResult::Continue(a)
            } else {
                let mut test = ContainmentResult::Stop(0);
                test.set(c, ab == Orientation::Clockwise);
                test.set(a, bc == Orientation::Clockwise);
                test
            }
        }
    }

    // Test the containment of the p position for the (b,c) and (a,b) sides in this order
    fn test_containment_bc_ab(&mut self, p: &T::Position, f: FaceIndex, a: Rot3, b: Rot3, c: Rot3) -> ContainmentResult {
        let tri = &self.tri;
        let pb = tri.get_face_vertex_position(f, b);
        let pc = tri.get_face_vertex_position(f, c);
        let bc = tri.predicates.orientation(pb, pc, p);
        if bc == Orientation::Clockwise {
            ContainmentResult::Continue(a)
        } else {
            let pa = tri.get_face_vertex_position(f, a);
            let ab = tri.predicates.orientation(pa, pb, p);
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
    fn locate_position_dim2(&mut self, p: &T::Position, hint: Option<FaceIndex>) -> Result<Location, String> {
        assert!(self.tri.dimension == 2);

        // find the start_face
        let face = match hint {
            Some(f) => f,
            None => {
                let vertex_count = self.tri.get_vertex_count();
                let sample_count = vertex_count / self.start_face_sampling_density;
                let v = self.guess_start_vertex(sample_count, p);
                let f = self.tri.get_vertex_face(v);
                if self.tri.is_finite_face(f) {
                    f
                } else {
                    let i = self.tri.get_face_vertex_index(f, self.tri.get_infinite_vertex()).unwrap();
                    self.tri.get_face_neighbor(f, i)
                }
            }
        };
        if !self.tri.is_finite_face(face) {
            return Err("Could not find start face".into());
        }

        // traverse triangulation and advance to the target position through the neighboring faces
        let mut iteration = 0;
        let mut prev;
        let mut cur = face;
        let mut from: Option<Rot3> = None;
        loop {
            iteration += 1;
            if iteration > self.tri.get_face_count() * 3 {
                break Err("Infinite location loop detected".into());
            } else if !self.tri.is_finite_face(cur) {
                break Ok(Location::OutsideConvexHull { face: cur });
            } else {
                // match_order: there exist configurations, where we could select two directions to follow the point.
                //  If the same direction is chosen all the time, one may end up in an infinite loop going around the point.
                //  After a limit, the walk becomes stochastic to break the infinite loop.
                //  Another option could be to store the visited faces and perform some tree traverse algorithm.
                let test_order = if iteration > self.iteration_stochastic_limit {
                    iteration % 2 == 0
                } else {
                    self.rng.gen::<bool>()
                };

                let test_result = match (from, test_order) {
                    (Some(Rot3(1)), true) => self.test_containment_ab_bc(p, cur, Rot3(2), Rot3(0), Rot3(1)),
                    (Some(Rot3(1)), false) => self.test_containment_bc_ab(p, cur, Rot3(2), Rot3(0), Rot3(1)),
                    (Some(Rot3(2)), true) => self.test_containment_ab_bc(p, cur, Rot3(0), Rot3(1), Rot3(2)),
                    (Some(Rot3(2)), false) => self.test_containment_bc_ab(p, cur, Rot3(0), Rot3(1), Rot3(2)),

                    (Some(Rot3(3)), true) => self.test_containment_ab_bc(p, cur, Rot3(1), Rot3(2), Rot3(0)),
                    (Some(Rot3(3)), false) => self.test_containment_bc_ab(p, cur, Rot3(1), Rot3(2), Rot3(0)),
                    (None, _) => {
                        //initial guess, test all the edges
                        let p0 = self.tri.get_face_vertex_position(cur, Rot3(0));
                        let p1 = self.tri.get_face_vertex_position(cur, Rot3(1));
                        let e01 = self.tri.predicates.orientation(p0, p1, p);
                        if e01 == Orientation::Clockwise {
                            ContainmentResult::Continue(Rot3(2))
                        } else {
                            let p2 = self.tri.get_face_vertex_position(cur, Rot3(2));
                            let e20 = self.tri.predicates.orientation(p2, p0, p);
                            if e20 == Orientation::Clockwise {
                                ContainmentResult::Continue(Rot3(1))
                            } else {
                                let e12 = self.tri.predicates.orientation(p1, p2, p);
                                if e12 == Orientation::Clockwise {
                                    ContainmentResult::Continue(Rot3(0))
                                } else {
                                    let mut test = ContainmentResult::Stop(0);
                                    test.set(Rot3(2), e01 == Orientation::Collinear);
                                    test.set(Rot3(0), e12 == Orientation::Collinear);
                                    test.set(Rot3(1), e20 == Orientation::Collinear);
                                    test
                                }
                            }
                        }
                    }
                    _ => unreachable!(),
                };

                match test_result {
                    ContainmentResult::Continue(dir) => {
                        // continue in the given direction
                        prev = cur;
                        cur = self.tri.get_face_neighbor(prev, dir);
                        from = self.tri.get_face_neighbor_index(cur, prev);
                    }
                    ContainmentResult::Stop(0) => {
                        // inside a face
                        break Ok(Location::Face { face: cur });
                    }
                    ContainmentResult::Stop(1) => {
                        // only on 0 edge
                        break Ok(Location::Edge {
                            face: cur,
                            index: Rot3(0),
                        });
                    }
                    ContainmentResult::Stop(2) => {
                        // only on 1 edge
                        break Ok(Location::Edge {
                            face: cur,
                            index: Rot3(1),
                        });
                    }
                    ContainmentResult::Stop(3) => {
                        //both on 0,1 edge
                        break Ok(Location::Vertex {
                            face: cur,
                            index: Rot3(2),
                        });
                    }
                    ContainmentResult::Stop(4) => {
                        // only on 2 edge
                        break Ok(Location::Edge {
                            face: cur,
                            index: Rot3(2),
                        });
                    }
                    ContainmentResult::Stop(5) => {
                        //both on 0,2 edge
                        break Ok(Location::Vertex {
                            face: cur,
                            index: Rot3(1),
                        });
                    }
                    ContainmentResult::Stop(6) => {
                        //both on 1,2 edge
                        break Ok(Location::Vertex {
                            face: cur,
                            index: Rot3(0),
                        });
                    }
                    _ => unreachable!(),
                }
            }
        }
    }*/
}
