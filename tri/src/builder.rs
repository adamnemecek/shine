use geometry::{Orientation, Predicates};
use graph::{Face, FaceExt, Graph, Vertex};
use locator::{Location, Locator};
use rand::{self, Rng, ThreadRng};
use types::{invalid_vertex, rot3, FaceIndex, Rot3, VertexIndex};

pub struct Builder<'a, R, P, V, F>
where
    R: 'a + Rng,
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    rng: R,
    pub tri: &'a mut Graph<P, V, F>,
}

impl<'a, P, V, F> Builder<'a, ThreadRng, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &mut Graph<P, V, F>) -> Builder<ThreadRng, P, V, F> {
        Builder {
            tri,
            rng: rand::thread_rng(),
        }
    }
}

impl<'a, R, P, V, F> Builder<'a, R, P, V, F>
where
    R: 'a + Rng,
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new_with_rng(rng: R, tri: &mut Graph<P, V, F>) -> Builder<R, P, V, F> {
        Builder { rng, tri }
    }

    pub fn add_vertex(&mut self, p: P::Position, hint: Option<FaceIndex>) -> VertexIndex {
        let location = {
            let mut locator = Locator::new(&mut self.rng, self.tri);
            locator.locate_position(&p, hint).unwrap()
        };
        self.add_vertex_at(p, location)
    }

    fn add_vertex_at(&mut self, p: P::Position, loc: Location) -> VertexIndex {
        match self.tri.dimension() {
            -1 => match loc {
                Location::Empty => self.extend_to_dim0(p),
                loc => unreachable!(format!("Invalid location for empty triangulation: {:?}", loc)),
            },
            0 => match loc {
                Location::Vertex(f, i) => self.tri[f].vertex(i),
                Location::OutsideAffineHull => self.extend_to_dim1(p),
                loc => unreachable!(format!("Invalid location for 0D triangulation: {:?}", loc)),
            },
            1 => match loc {
                Location::Vertex(f, i) => self.tri[f].vertex(i),
                Location::Edge(f, _) => self.split_edge_dim1(p, f),
                Location::OutsideConvexHull(f) => self.split_edge_dim1(p, f),
                Location::OutsideAffineHullClockwise => self.extend_to_dim2(p, false),
                Location::OutsideAffineHullCounterClockwise => self.extend_to_dim2(p, true),
                loc => unreachable!(format!("Invalid location for 1D triangulation: {:?}", loc)),
            },
            2 => match loc {
                Location::Vertex(f, i) => self.tri[f].vertex(i),
                Location::Edge(f, i) => self.split_edge_dim2(p, f, i),
                Location::Face(f) => self.split_face(p, f),
                Location::OutsideConvexHull(f) => self.insert_outside_convex_hull2(p, f),
                _ => unreachable!(format!("Invalid location for 2D triangulation: {:?}", loc)),
            },
            dim => unreachable!(format!("Invalid dimension: {}, {:?}", dim, loc)),
        }
    }

    fn create_infinite_vertex(&mut self) -> VertexIndex {
        let v = self.tri.store_vertex(Default::default());
        self.tri.set_infinite_vertex(v);
        v
    }

    fn create_vertex_with_position(&mut self, p: P::Position) -> VertexIndex {
        let mut v: V = Default::default();
        v.set_position(p);
        self.tri.store_vertex(v)
    }

    fn create_face(&mut self) -> FaceIndex {
        self.tri.store_face(Default::default())
    }

    fn create_face_with_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> FaceIndex {
        let mut f: F = Default::default();
        f.set_vertices(v0, v1, v2);
        self.tri.store_face(f)
    }

    fn copy_constraint(&mut self, f_from: FaceIndex, i_from: Rot3, f_to: FaceIndex, i_to: Rot3) {
        let c = self.tri[f_from].constraint(i_from);
        self.tri[f_to].set_constraint(i_to, c);
    }

    fn clear_constraint(&mut self, f: FaceIndex, i: Rot3) {
        self.tri[f].set_constraint(i, Default::default());
    }

    fn extend_to_dim0(&mut self, p: P::Position) -> VertexIndex {
        assert!(self.tri.dimension() == -1);
        assert!(!self.tri.infinite_vertex().is_valid());
        assert!(self.tri.vertex_count() == 0);
        assert!(self.tri.face_count() == 0);

        self.tri.set_dimension(0);

        let v0 = self.create_infinite_vertex();
        let v1 = self.create_vertex_with_position(p);
        let f0 = self.create_face_with_vertices(v0, invalid_vertex(), invalid_vertex());
        let f1 = self.create_face_with_vertices(v1, invalid_vertex(), invalid_vertex());

        self.tri[v0].set_face(f0);
        self.tri[v1].set_face(f1);
        self.set_adjacent(f0, rot3(0), f1, rot3(0));

        v1
    }

    /// Extends dimension from 0D to 1D by creating a segment (face) out of the (two) finite points.
    /// In 1D a face is a segment, and the shell is the triangular face (as described in extend_to_dim2). The
    /// infinite vertex is always the vertex corresponding to the 2nd index in each (finite) faces(segments).
    fn extend_to_dim1(&mut self, p: P::Position) -> VertexIndex {
        assert!(self.tri.dimension() == 0);
        assert!(self.tri.vertex_count() == 2);
        assert!(self.tri.face_count() == 2);

        self.tri.set_dimension(1);

        // infinite, finite vertices
        let (v0, v1) = {
            let v0 = VertexIndex(0);
            let v1 = VertexIndex(1);
            if self.tri.is_infinite_vertex(v0) {
                (v0, v1)
            } else {
                (v1, v0)
            }
        };
        // finite (new) vertex
        let v2 = self.create_vertex_with_position(p);

        let f0 = self.tri[v0].face();
        let f1 = self.tri[v1].face();
        let f2 = self.create_face_with_vertices(v2, v0, invalid_vertex());

        self.tri[f0].set_vertex(rot3(1), v1);
        self.tri[f1].set_vertex(rot3(1), v2);
        self.tri[v2].set_face(f2);

        self.set_adjacent(f0, rot3(0), f1, rot3(1));
        self.set_adjacent(f1, rot3(0), f2, rot3(1));
        self.set_adjacent(f2, rot3(0), f0, rot3(1));

        v2
    }

    /// Extends dimension from 1D to 2D by creating triangles(2d face) out of the segments (1D faces).
    /// The infinite vertex and triangulation can be seen as an n+1 dimensional shell. The
    /// edges of the convex hull of an nD object is connected to the infinite vertex, which can be seen as
    /// a normal point in (n+1)D which is "above" the nD points.
    /// For 1D -> 2D lifting we have to extended each segment into a triangle that creates a shell in 3D space
    /// After lifting each segment, we have to fill the holes on the shell in 3D by generating the infinite faces
    /// of 2D.
    fn extend_to_dim2(&mut self, p: P::Position, is_ccw: bool) -> VertexIndex {
        assert_eq!(self.tri.dimension(), 1);

        self.tri.set_dimension(2);

        // face neighborhood:
        // It is assumed that all the segments are directed in the same direction:
        // the series of the vertex indices (Index3) is a (closed) chain of ..010101..
        //
        // F0: starting (infinite) face
        // Fm: ending (infinite) face
        // Cj: original (finite) faces extended to 2D, j in [0..n]
        // Nj: new, generated faces in 2D, j in [0..n]
        // n: number of finite faces - 1
        // i: the Index3 of the next neighbor, either 010101.. or 101010.... sequence (See the note above)

        // input constraint:
        //   F0[  i] = Fm[1-i]
        //   F0[1-i] = C1[i]
        //   Fm[  i] = Cm[1-i]
        //   Fm[1-i] = F0[i]
        //   Cj-1[1-i] = Cj[i]

        let new_vertex = self.create_vertex_with_position(p);

        // F0, start by an infinite face for which the convex hull (segment) and p is in counter-clockwise direction
        // Fm is the other infinite face
        let (f0, i0, fm, im) = {
            let f0 = self.tri.infinite_face();
            let i0 = self.tri[f0].get_vertex_index(self.tri.infinite_vertex()).unwrap();
            let im = i0.mirror(2);
            let fm = self.tri[f0].neighbor(im);

            if is_ccw {
                (f0, i0, fm, im)
            } else {
                (fm, im, f0, i0)
            }
        };

        let c0 = self.tri[f0].neighbor(i0);

        let mut cur = c0;
        let mut new_face = FaceIndex::invalid();
        while cur != fm {
            let prev_new_face = new_face;
            new_face = self.create_face();

            let v0 = self.tri[cur].vertex(rot3(1));
            let v1 = self.tri[cur].vertex(rot3(0));
            let vinf = self.tri.infinite_vertex();
            if i0 == rot3(1) {
                self.tri[new_face].set_vertices(v0, v1, new_vertex);
                self.tri[cur].set_vertex(rot3(2), vinf);
                self.tri[new_vertex].set_face(new_face);
            } else {
                self.tri[new_face].set_vertices(v0, v1, vinf);
                self.tri[cur].set_vertex(rot3(2), new_vertex);
                self.tri[new_vertex].set_face(cur);
            }

            self.set_adjacent(cur, rot3(2), new_face, rot3(2));
            if prev_new_face.is_valid() {
                self.set_adjacent(prev_new_face, im, new_face, i0);
            }

            self.copy_constraint(cur, rot3(2), new_face, rot3(2));

            cur = self.tri[cur].neighbor(i0);
        }

        let cm = self.tri[fm].neighbor(im);
        let n0 = self.tri[c0].neighbor(rot3(2));
        let nm = self.tri[cm].neighbor(rot3(2));

        self.tri[f0].set_vertex(rot3(2), new_vertex);
        self.tri[fm].set_vertex(rot3(2), new_vertex);

        if i0 == rot3(1) {
            self.tri[f0].swap_vertices(rot3(2), rot3(1));
            self.tri[fm].swap_vertices(rot3(0), rot3(2));
            self.set_adjacent(f0, rot3(1), c0, rot3(0));
            self.set_adjacent(fm, rot3(0), cm, rot3(1));
            self.set_adjacent(f0, rot3(2), n0, rot3(1));
            self.set_adjacent(fm, rot3(2), nm, rot3(0));
        } else {
            self.set_adjacent(f0, rot3(2), n0, rot3(0));
            self.set_adjacent(fm, rot3(2), nm, rot3(1));
        }
        new_vertex
    }

    fn split_edge_dim1(&mut self, p: P::Position, f: FaceIndex) -> VertexIndex {
        assert!(self.tri.dimension() == 1);

        // f0 : the face to split
        // f2 : new face
        // v2 : new vertex
        //
        //     v0             v1
        // ----*0-----f0-----1*j--f1---i*---
        //
        //     v0       v2      v1
        // ----*0--f0--1*0-f2--1*j--f1---i*---

        let v2 = self.create_vertex_with_position(p); // new vertex
        let f2 = self.create_face(); // new face

        let f0 = f;
        let f1 = self.tri[f0].neighbor(rot3(0));
        let i = self.tri[f1].get_neighbor_index(f0).unwrap();
        let v1 = self.tri[f1].vertex(i.mirror(2)); // j = 1-i

        self.tri[v1].set_face(f1);
        self.tri[v2].set_face(f2);
        self.tri[f0].set_vertex(rot3(1), v2);
        self.tri[f2].set_vertices(v2, v1, invalid_vertex());
        self.set_adjacent(f2, rot3(1), f0, rot3(0));
        self.set_adjacent(f2, rot3(0), f1, i);

        self.copy_constraint(f0, rot3(2), f2, rot3(2));

        v2
    }

    fn split_edge_dim2(&mut self, p: P::Position, face: FaceIndex, edge: Rot3) -> VertexIndex {
        assert_eq!(self.tri.dimension(), 2);

        //           v0  i02 = edge
        //         /  |2 \
        //       / F0 | N0 \
        // i00 /      |0    1\ i01
        //   v1 ------vp------ v2
        // i11 \      |0    2/ i10
        //       \ F1 | N1 /
        //         \  |1 /
        //           v3  i12

        let vp = self.create_vertex_with_position(p);
        let n0 = self.create_face();
        let n1 = self.create_face();
        let f0 = face;
        let f1 = self.tri[f0].neighbor(edge);
        let i00 = edge.increment();
        let i01 = edge.decrement();
        let i02 = edge;
        let i12 = self.tri[f1].get_neighbor_index(f0).unwrap();
        let i11 = i12.decrement();
        let i10 = i12.increment();

        let v0 = self.tri[f0].vertex(i02);
        //let v1 = self.tri[f0].vertex(i00);
        let v2 = self.tri[f0].vertex(i01);
        let v3 = self.tri[f1].vertex(i12);

        self.tri[n0].set_vertices(vp, v2, v0);
        self.tri[n1].set_vertices(vp, v3, v2);
        self.tri[f0].set_vertex(i01, vp);
        self.tri[f1].set_vertex(i10, vp);
        self.tri[vp].set_face(n0);
        self.tri[v2].set_face(n0);
        self.tri[v0].set_face(n0);
        self.tri[v3].set_face(n1);

        self.move_adjacent(n0, rot3(0), f0, i00);
        self.set_adjacent(n0, rot3(1), f0, i00);
        self.set_adjacent(n0, rot3(2), n1, rot3(1));

        self.move_adjacent(n1, rot3(0), f1, i11);
        self.set_adjacent(n1, rot3(2), f1, i11);

        self.copy_constraint(f0, i00, n0, rot3(0));
        self.copy_constraint(f0, i02, n0, rot3(2));
        self.clear_constraint(f0, i00);
        self.copy_constraint(f1, i11, n1, rot3(0));
        self.copy_constraint(f1, i12, n1, rot3(1));
        self.clear_constraint(f1, i11);

        vp
    }

    fn split_face(&mut self, p: P::Position, face: FaceIndex) -> VertexIndex {
        assert_eq!(self.tri.dimension(), 2);

        //            v2
        //            x
        //         / 2|2 \
        //        /   |   \
        //       /   vp    \
        //      /N0 1/x\0 N1\
        //     /0  /  2  \  1\
        //      /0   F0    1\
        // v0  x-------------x v1

        let vp = self.create_vertex_with_position(p);
        let n0 = self.create_face();
        let n1 = self.create_face();
        let f0 = face;

        let v0 = self.tri[f0].vertex(rot3(0));
        let v1 = self.tri[f0].vertex(rot3(1));
        let v2 = self.tri[f0].vertex(rot3(2));

        self.tri[n0].set_vertices(v0, vp, v2);
        self.tri[n1].set_vertices(vp, v1, v2);
        self.tri[f0].set_vertex(rot3(2), vp);
        self.tri[vp].set_face(f0);
        self.tri[v2].set_face(n0);

        self.set_adjacent(n0, rot3(0), n1, rot3(1));
        self.move_adjacent(n0, rot3(1), f0, rot3(1));
        self.move_adjacent(n1, rot3(0), f0, rot3(0));
        self.set_adjacent(n0, rot3(2), f0, rot3(1));
        self.set_adjacent(n1, rot3(2), f0, rot3(0));

        self.copy_constraint(f0, rot3(1), n0, rot3(1));
        self.copy_constraint(f0, rot3(0), n1, rot3(0));
        self.clear_constraint(f0, rot3(0));
        self.clear_constraint(f0, rot3(1));
        vp
    }

    /// Set adjacent face information for two neighboring faces.
    fn set_adjacent(&mut self, f0: FaceIndex, i0: Rot3, f1: FaceIndex, i1: Rot3) {
        assert!(i0.is_valid() && i1.is_valid());
        assert!(i0.id() <= self.tri.dimension() as u8 && i1.id() <= self.tri.dimension() as u8);
        self.tri[f0].set_neighbor(i0, f1);
        self.tri[f1].set_neighbor(i1, f0);
    }

    /// Move adjacent face information from one face into another.
    fn move_adjacent(&mut self, f_target: FaceIndex, i_target: Rot3, f_source: FaceIndex, i_source: Rot3) {
        let n = self.tri[f_source].neighbor(i_source);
        let i = self.tri[n].get_neighbor_index(f_source).unwrap();
        self.set_adjacent(f_target, i_target, n, i);
    }

    /// Flips the edge in the quadrangle defined by the two neighboring triangles.
    pub fn flip(&mut self, face: FaceIndex, edge: Rot3) {
        assert_eq!(self.tri.dimension(), 2);
        assert!(face.is_valid() && edge.is_valid());

        //            v3                       v3
        //          2 * 1                      *
        //         /  |   \                 /  1  \
        //       /    |     \             /2  F1   0\
        //  v0 *0  F0 | F1  0* v2    v0 * ----------- * v2
        //       \    |    /              \0  F0   2/
        //         \  |  /                  \  1  /
        //          1 * 2                      *
        //            v1                      v1

        let f0 = face;
        let i00 = edge;
        let i01 = i00.increment();
        let i02 = i00.decrement();

        let f1 = self.tri[f0].neighbor(i00);
        let i10 = self.tri[f1].get_neighbor_index(f0).unwrap();
        let i11 = i10.increment();
        let i12 = i10.decrement();

        let v0 = self.tri[f0].vertex(i00);
        let v1 = self.tri[f0].vertex(i01);
        let v3 = self.tri[f0].vertex(i02);
        let v2 = self.tri[f1].vertex(i10);
        assert!(self.tri[f1].vertex(i11) == v3);
        assert!(self.tri[f1].vertex(i12) == v1);

        self.tri[f0].set_vertex(i02, v2);
        self.tri[f1].set_vertex(i12, v0);
        self.tri[v0].set_face(f0);
        self.tri[v1].set_face(f0);
        self.tri[v2].set_face(f0);
        self.tri[v3].set_face(f1);

        self.move_adjacent(f0, i00, f1, i11);
        self.move_adjacent(f1, i10, f0, i01);
        self.set_adjacent(f0, i01, f1, i11);

        self.copy_constraint(f1, i11, f0, i00);
        self.copy_constraint(f0, i01, f1, i10);
        self.clear_constraint(f0, i01);
        self.clear_constraint(f1, i11);
    }

    fn insert_outside_convex_hull2(&mut self, p: P::Position, face: FaceIndex) -> VertexIndex {
        let f0 = face;
        let vinf = self.tri.infinite_vertex();
        let i = self.tri[f0].get_vertex_index(vinf).unwrap();
        let mut fcw = self.tri[f0].neighbor(i.decrement());
        let mut fccw = self.tri[f0].neighbor(i.increment());

        // split the infinite triangle by the given position
        let vp = self.split_face(p, face);

        //correct faces by flipping
        loop {
            let i = self.tri[fcw].get_vertex_index(vinf).unwrap();
            let next = self.tri[fcw].neighbor(i.decrement());
            if !self.tri.get_edge_vertex_orientation(fcw, i, vp).is_ccw() {
                break;
            }
            self.flip(fcw, i.increment());
            fcw = next;
        }

        loop {
            let i = self.tri[fccw].get_vertex_index(vinf).unwrap();
            let next = self.tri[fccw].neighbor(i.increment());
            if !self.tri.get_edge_vertex_orientation(fccw, i, vp).is_ccw() {
                break;
            }
            self.flip(fccw, i.decrement());
            fccw = next;
        }

        vp
    }
}
