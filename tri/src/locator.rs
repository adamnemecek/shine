use geometry::{CollinearTest, Orientation, Predicates};
use rand::{self, Rng, ThreadRng};
use triangulation::{TriGraph, TriStore};
use types::{FaceIndex, Location, Rot3, VertexIndex};

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

pub struct Locator<'a, P, R, T>
where
    P: 'a + Predicates,
    R: 'a + Rng,
    T: 'a + TriStore<Position = <P as Predicates>::Position>,
{
    tri: &'a TriGraph<P, T>,
    rng: R,
    iteration_stochastic_limit: usize,
    start_face_sampling_density: usize,
}

impl<'a, P, R, T> Locator<'a, P, R, T>
where
    P: 'a + Predicates,
    R: 'a + Rng,
    T: 'a + TriStore<Position = <P as Predicates>::Position>,
{
    pub fn new_with_rng(tri: &TriGraph<P, T>, rng: R) -> Locator<P, R, T> {
        Locator {
            tri,
            rng,
            iteration_stochastic_limit: 10,
            start_face_sampling_density: 100,
        }
    }
}

impl<'a, P, T> Locator<'a, P, ThreadRng, T>
where
    P: 'a + Predicates,
    T: 'a + TriStore<Position = <P as Predicates>::Position>,
{
    pub fn new(tri: &TriGraph<P, T>) -> Locator<P, ThreadRng, T> {
        Locator {
            tri,
            rng: rand::thread_rng(),
            iteration_stochastic_limit: 10,
            start_face_sampling_density: 100,
        }
    }
}

impl<'a, P, R, T> Locator<'a, P, R, T>
where
    P: 'a + Predicates,
    R: 'a + Rng,
    T: 'a + TriStore<Position = <P as Predicates>::Position>,
{
    pub fn locate_position(&mut self, p: &T::Position, hint: Option<FaceIndex>) -> Result<Location, String> {
        match self.tri.dimension {
            -1 => Ok(Location::Empty),
            0 => self.locate_position_dim0(p),
            1 => self.locate_position_dim1(p),
            2 => self.locate_position_dim2(p, hint),
            dim => unreachable!(format!("Invalid dimension: {}", dim)),
        }
    }

    /// Return some random (finite) vertex from the triangulation
    pub fn get_random_finite_vertex(&mut self) -> VertexIndex {
        assert!(!self.tri.is_empty());
        let cnt = self.tri.get_vertex_count() - 1;
        let rnd = self.rng.gen_range(0, cnt);
        if rnd < self.tri.get_infinite_vertex().0 {
            VertexIndex(rnd)
        } else {
            VertexIndex(rnd + 1)
        }
    }

    /// Find the location of a point in a single point triangulation (dimension = 0).
    fn locate_position_dim0(&mut self, p: &T::Position) -> Result<Location, String> {
        let tri = self.tri;

        assert!(tri.dimension == 0);

        // find the finite vertex
        let v0 = if tri.get_infinite_vertex() == VertexIndex(0) {
            VertexIndex(1)
        } else {
            VertexIndex(0)
        };
        let p0 = tri.get_vertex_position(v0);

        if tri.predicates.test_coincident_points(p0, p) {
            let f = tri.get_vertex_face(v0);
            Ok(Location::Vertex {
                face: f,
                index: tri.get_face_vertex_index(f, v0).unwrap(),
            })
        } else {
            Ok(Location::OutsideAffineHull)
        }
    }

    /// Find the location of a point in a straight line strip. (dimension = 1)
    fn locate_position_dim1(&mut self, p: &T::Position) -> Result<Location, String> {
        let tri = self.tri;
        assert!(tri.dimension == 1);

        // calculate the convex hull of the 1-d mesh
        // the convex hull is a segment made up from the two (finite) neighboring vertices of the infinite vertex

        // first point of the convex hull (segments)
        let f0 = tri.get_infinite_face();
        let iv0 = tri.get_face_vertex_index(f0, tri.get_infinite_vertex()).unwrap();
        let cp0 = tri.get_face_vertex_position(f0, iv0.mirror(2));

        // last point of the convex hull (segments)
        let f1 = tri.get_face_neighbor(f0, iv0.mirror(2));
        let iv1 = tri.get_face_vertex_index(f1, tri.get_infinite_vertex()).unwrap();
        let cp1 = tri.get_face_vertex_position(f1, iv1.mirror(2));

        match tri.predicates.orientation(cp0, cp1, p) {
            Orientation::Clockwise => Ok(Location::OutsideAffineHullClockwise),
            Orientation::CounterClockwise => Ok(Location::OutsideAffineHullCounterClockwise),
            _ => {
                // point is on the line
                let t = tri
                    .predicates
                    .test_collinear_points(cp0, cp1, p)
                    .ok_or("Points are not collinear")?;
                match t {
                    CollinearTest::BeforeA => Ok(Location::OutsideConvexHull { face: f0 }),
                    CollinearTest::A => Ok(Location::Vertex {
                        face: f0,
                        index: iv0.mirror(2),
                    }),
                    CollinearTest::B => Ok(Location::Vertex {
                        face: f1,
                        index: iv1.mirror(2),
                    }),
                    CollinearTest::AfterB => Ok(Location::OutsideConvexHull { face: f1 }),
                    CollinearTest::BetweenAB => {
                        // Start from an infinite face(f0) and advance to the neighboring segments while the
                        // the edge(face) containing the point is not found

                        let mut prev = f0;
                        let mut dir = iv0;
                        loop {
                            let cur = tri.get_face_neighbor(prev, dir);
                            assert!(tri.is_finite_face(cur));
                            let p0 = tri.get_face_vertex_position(cur, Rot3(0));
                            let p1 = tri.get_face_vertex_position(cur, Rot3(1));
                            let t = tri.predicates.test_collinear_points(p0, p1, p).unwrap();
                            match t {
                                CollinearTest::A => {
                                    // identical to p0
                                    break Ok(Location::Vertex {
                                        face: cur,
                                        index: Rot3(0),
                                    });
                                }
                                CollinearTest::B => {
                                    // identical to p1
                                    break Ok(Location::Vertex {
                                        face: cur,
                                        index: Rot3(1),
                                    });
                                }
                                CollinearTest::BetweenAB => {
                                    // inside the (p0,p1) segment
                                    break Ok(Location::Edge {
                                        face: cur,
                                        index: Rot3(2),
                                    });
                                }
                                _ => {
                                    // advance to the next edge
                                    let vi = tri.get_face_neighbor_index(cur, prev).unwrap();
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

    /// Ques point location by finding the nearest vertex to the point from a random sampling of vertices
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
    }
}
