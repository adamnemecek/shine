use build::factory::Factory;
use geometry::{Orientation, Predicates};
use graph::{Face, FaceExt, PredicatesContext, Triangulation, Vertex};
use query::{GeometryQuery, TopologyQuery, VertexClue};
use types::{invalid_face_index, invalid_vertex_index, rot3, vertex_index, FaceIndex, Rot3, VertexIndex};

pub trait Updater {
    /// Split an edge by the given vertex. No geometry constraint is checked.
    fn split_edge(&mut self, face: FaceIndex, edge: Rot3, vert: VertexIndex);

    /// Split a face by the given vertex. No geometry constraint is checked.
    fn split_face(&mut self, face: FaceIndex, vert: VertexIndex);

    /// Extend the dimension of the triangualtion using the given vertex. No geometry constraint is checked.
    fn extend_dimension(&mut self, vert: VertexIndex);

    /// Flip the edges between two adjacent triangles
    fn flip(&mut self, face: FaceIndex, edge: Rot3);
}

impl<PR, V, F, C> Triangulation<PR::Position, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: PredicatesContext<Predicates = PR>,
{
    /// Copy constraint from one face into another
    fn copy_constraint(&mut self, f_from: FaceIndex, i_from: Rot3, f_to: FaceIndex, i_to: Rot3) {
        let c = self[f_from].constraint(i_from);
        self[f_to].set_constraint(i_to, c);
    }

    /// Set adjacent face information for two neighboring faces.
    fn set_adjacent(&mut self, f0: FaceIndex, i0: Rot3, f1: FaceIndex, i1: Rot3) {
        assert!(i0.is_valid() && i1.is_valid());
        assert!(i0.id() <= self.dimension() as u8 && i1.id() <= self.dimension() as u8);
        self[f0].set_neighbor(i0, f1);
        self[f1].set_neighbor(i1, f0);
    }

    /// Move adjacent face information from one face into another.
    fn move_adjacent(&mut self, f_target: FaceIndex, i_target: Rot3, f_source: FaceIndex, i_source: Rot3) {
        let n = self[f_source].neighbor(i_source);
        let i = self[n].get_neighbor_index(f_source).unwrap();
        self.set_adjacent(f_target, i_target, n, i);
    }

    fn split_edge_dim1(&mut self, f: FaceIndex, edge: Rot3, vert: VertexIndex) {
        assert!(self.dimension() == 1);
        assert!(edge == rot3(2));
        self.split_face_dim1(f, vert);
    }

    fn split_edge_dim2(&mut self, face: FaceIndex, edge: Rot3, vert: VertexIndex) {
        assert_eq!(self.dimension(), 2);

        //           v0  i02 = edge
        //         /  |2 \
        //       / F0 | N0 \
        // i00 /      |0    1\ i01
        //   v1 ------vp------ v2
        // i11 \      |0    2/ i10
        //       \ F1 | N1 /
        //         \  |1 /
        //           v3  i12

        let vp = vert;
        let n0 = self.create_face();
        let n1 = self.create_face();
        let f0 = face;
        let f1 = self[f0].neighbor(edge);
        let i00 = edge.increment();
        let i01 = edge.decrement();
        let i02 = edge;
        let i12 = self[f1].get_neighbor_index(f0).unwrap();
        let i11 = i12.decrement();
        let i10 = i12.increment();

        let v0 = self[f0].vertex(i02);
        //let v1 = self[f0].vertex(i00);
        let v2 = self[f0].vertex(i01);
        let v3 = self[f1].vertex(i12);

        self[n0].set_vertices(vp, v2, v0);
        self[n1].set_vertices(vp, v3, v2);
        self[f0].set_vertex(i01, vp);
        self[f1].set_vertex(i10, vp);
        self[vp].set_face(n0);
        self[v2].set_face(n0);
        self[v0].set_face(n0);
        self[v3].set_face(n1);

        self.move_adjacent(n0, rot3(0), f0, i00);
        self.set_adjacent(n0, rot3(1), f0, i00);
        self.set_adjacent(n0, rot3(2), n1, rot3(1));

        self.move_adjacent(n1, rot3(0), f1, i11);
        self.set_adjacent(n1, rot3(2), f1, i11);

        self.copy_constraint(f0, i00, n0, rot3(0));
        self.copy_constraint(f0, i02, n0, rot3(2));
        self[f0].clear_constraint(i00);

        self.copy_constraint(f1, i11, n1, rot3(0));
        self.copy_constraint(f1, i12, n1, rot3(1));
        self[f1].clear_constraint(i11);
    }

    fn split_face_dim1(&mut self, f: FaceIndex, vert: VertexIndex) {
        assert!(self.dimension() == 1);

        // f0 : the face to split
        // f2 : new face
        // v2 : new vertex
        //
        //     v0             v1
        // ----*0-----f0-----1*j--f1---i*---
        //
        //     v0       v2      v1
        // ----*0--f0--1*0-f2--1*j--f1---i*---

        let v2 = vert;
        let f2 = self.create_face(); // new face

        let f0 = f;
        let f1 = self[f0].neighbor(rot3(0));
        let i = self[f1].get_neighbor_index(f0).unwrap();
        let v1 = self[f1].vertex(i.mirror(2)); // j = 1-i

        self[v1].set_face(f1);
        self[v2].set_face(f2);
        self[f0].set_vertex(rot3(1), v2);
        self[f2].set_vertices(v2, v1, invalid_vertex_index());
        self.set_adjacent(f2, rot3(1), f0, rot3(0));
        self.set_adjacent(f2, rot3(0), f1, i);

        self.copy_constraint(f0, rot3(2), f2, rot3(2));
    }

    fn split_finite_face_dim2(&mut self, face: FaceIndex, vert: VertexIndex) {
        assert_eq!(self.dimension(), 2);

        //            v2
        //            x
        //         / 2|2 \
        //        /   |   \
        //       /   vp    \
        //      /N0 1/x\0 N1\
        //     /0  /  2  \  1\
        //      /0   F0    1\
        // v0  x-------------x v1

        let vp = vert;
        let n0 = self.create_face();
        let n1 = self.create_face();
        let f0 = face;

        let v0 = self[f0].vertex(rot3(0));
        let v1 = self[f0].vertex(rot3(1));
        let v2 = self[f0].vertex(rot3(2));

        self[n0].set_vertices(v0, vp, v2);
        self[n1].set_vertices(vp, v1, v2);
        self[f0].set_vertex(rot3(2), vp);
        self[vp].set_face(f0);
        self[v2].set_face(n0);

        self.set_adjacent(n0, rot3(0), n1, rot3(1));
        self.move_adjacent(n0, rot3(1), f0, rot3(1));
        self.move_adjacent(n1, rot3(0), f0, rot3(0));
        self.set_adjacent(n0, rot3(2), f0, rot3(1));
        self.set_adjacent(n1, rot3(2), f0, rot3(0));

        self.copy_constraint(f0, rot3(1), n0, rot3(1));
        self.copy_constraint(f0, rot3(0), n1, rot3(0));
        self[f0].clear_constraint(rot3(0));
        self[f0].clear_constraint(rot3(1));
    }

    fn split_face_dim2(&mut self, face: FaceIndex, vert: VertexIndex) {
        let f0 = face;
        let vinf = self.infinite_vertex();

        // extract info of the infinte faces to handle the case when the convexx hull is extened
        let infinite_info = self[f0].get_vertex_index(vinf).map(|i| {
            let fcw = self[f0].neighbor(i.decrement());
            let fccw = self[f0].neighbor(i.increment());
            (fcw, fccw)
        });

        // perform a normal split
        self.split_finite_face_dim2(face, vert);

        if let Some((mut fcw, mut fccw)) = infinite_info {
            //correct faces by flipping
            loop {
                let i = self[fcw].get_vertex_index(vinf).unwrap();
                let next = self[fcw].neighbor(i.decrement());
                if !self.get_edge_vertex_orientation(fcw, i, vert).is_ccw() {
                    break;
                }
                self.flip(fcw, i.increment());
                fcw = next;
            }

            loop {
                let i = self[fccw].get_vertex_index(vinf).unwrap();
                let next = self[fccw].neighbor(i.increment());
                if !self.get_edge_vertex_orientation(fccw, i, vert).is_ccw() {
                    break;
                }
                self.flip(fccw, i.decrement());
                fccw = next;
            }
        }
    }

    /// Extends dimension from none to 0D by creating the infinite vertices.
    fn extend_to_dim0(&mut self, vert: VertexIndex) {
        assert!(self.dimension() == -1);
        assert!(!self.infinite_vertex().is_valid());
        assert!(self.vertex_count() == 1); // includes the new vertex
        assert!(self.face_count() == 0);

        self.set_dimension(0);

        let v0 = self.create_infinite_vertex();
        let v1 = vert;
        let f0 = self.create_face_with_vertices(v0, invalid_vertex_index(), invalid_vertex_index());
        let f1 = self.create_face_with_vertices(v1, invalid_vertex_index(), invalid_vertex_index());

        self[v0].set_face(f0);
        self[v1].set_face(f1);
        self.set_adjacent(f0, rot3(0), f1, rot3(0));
    }

    /// Extends dimension from 0D to 1D by creating a segment (face) out of the (two) finite points.
    /// In 1D a face is a segment, and the shell is the triangular face (as described in extend_to_dim2). The
    /// infinite vertex is always the vertex corresponding to the 2nd index in each (finite) faces(segments).
    fn extend_to_dim1(&mut self, vert: VertexIndex) {
        assert!(self.dimension() == 0);
        assert!(self.vertex_count() == 3); // includes the new vertex
        assert!(self.face_count() == 2);

        self.set_dimension(1);

        // infinite, finite vertices
        let (v0, v1) = {
            let v0 = vertex_index(0);
            let v1 = vertex_index(1);
            if self.is_infinite_vertex(v0) {
                (v0, v1)
            } else {
                (v1, v0)
            }
        };
        // finite (new) vertex
        let v2 = vert;

        let f0 = self[v0].face();
        let f1 = self[v1].face();
        let f2 = self.create_face_with_vertices(v2, v0, invalid_vertex_index());

        self[f0].set_vertex(rot3(1), v1);
        self[f1].set_vertex(rot3(1), v2);
        self[v2].set_face(f2);

        self.set_adjacent(f0, rot3(0), f1, rot3(1));
        self.set_adjacent(f1, rot3(0), f2, rot3(1));
        self.set_adjacent(f2, rot3(0), f0, rot3(1));
    }

    /// Extends dimension from 1D to 2D by creating triangles (2d fac0es) out of the segments (1D faces).
    /// The infinite vertex and triangulation can be seen as an n+1 dimensional shell. The
    /// edges of the convex hull of an nD object is connected to the infinite vertex, which can be seen as
    /// a normal point in (n+1)D which is "above" the nD points.
    /// For 1D -> 2D lifting we have to extended each segment into a triangle that creates a shell in 3D space.
    /// After transforming each segment int a triangle, we have to add the cap in 3D by generating the infinite faces.
    fn extend_to_dim2(&mut self, vert: VertexIndex) {
        assert_eq!(self.dimension(), 1);

        self.set_dimension(2);

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

        // F0, start by an infinite face for which the convex hull (segment) and p is in counter-clockwise direction
        // Fm is the other infinite face
        let (f0, i0, fm, im) = {
            let f0 = self.infinite_face();
            let i0 = self[f0].get_vertex_index(self.infinite_vertex()).unwrap();
            let im = i0.mirror(2);
            let fm = self[f0].neighbor(im);

            let orient = {
                let pr = self.context.predicates();
                let cp0 = self.p(VertexClue::face_vertex(f0, im));
                let cp1 = self.p(VertexClue::face_vertex(fm, i0));
                let p = self.p(vert);
                pr.orientation_triangle(cp0, cp1, p)
            };
            assert!(!orient.is_collinear());

            if orient.is_ccw() {
                (f0, i0, fm, im)
            } else {
                (fm, im, f0, i0)
            }
        };

        let c0 = self[f0].neighbor(i0);

        let mut cur = c0;
        let mut new_face = invalid_face_index();
        while cur != fm {
            let prev_new_face = new_face;
            new_face = self.create_face();

            let v0 = self[cur].vertex(rot3(1));
            let v1 = self[cur].vertex(rot3(0));
            let vinf = self.infinite_vertex();
            if i0 == rot3(1) {
                self[new_face].set_vertices(v0, v1, vert);
                self[cur].set_vertex(rot3(2), vinf);
                self[vert].set_face(new_face);
            } else {
                self[new_face].set_vertices(v0, v1, vinf);
                self[cur].set_vertex(rot3(2), vert);
                self[vert].set_face(cur);
            }

            self.set_adjacent(cur, rot3(2), new_face, rot3(2));
            if prev_new_face.is_valid() {
                self.set_adjacent(prev_new_face, im, new_face, i0);
            }

            self.copy_constraint(cur, rot3(2), new_face, rot3(2));

            cur = self[cur].neighbor(i0);
        }

        let cm = self[fm].neighbor(im);
        let n0 = self[c0].neighbor(rot3(2));
        let nm = self[cm].neighbor(rot3(2));

        self[f0].set_vertex(rot3(2), vert);
        self[fm].set_vertex(rot3(2), vert);

        if i0 == rot3(1) {
            self[f0].swap_vertices(rot3(2), rot3(1));
            self[fm].swap_vertices(rot3(0), rot3(2));
            self.set_adjacent(f0, rot3(1), c0, rot3(0));
            self.set_adjacent(fm, rot3(0), cm, rot3(1));
            self.set_adjacent(f0, rot3(2), n0, rot3(1));
            self.set_adjacent(fm, rot3(2), nm, rot3(0));
        } else {
            self.set_adjacent(f0, rot3(2), n0, rot3(0));
            self.set_adjacent(fm, rot3(2), nm, rot3(1));
        }
    }

    fn flip_face(&mut self, face: FaceIndex, edge: Rot3) {
        assert_eq!(self.dimension(), 2);
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

        let f1 = self[f0].neighbor(i00);
        let i10 = self[f1].get_neighbor_index(f0).unwrap();
        let i11 = i10.increment();
        let i12 = i10.decrement();

        let v0 = self[f0].vertex(i00);
        let v1 = self[f0].vertex(i01);
        let v3 = self[f0].vertex(i02);
        let v2 = self[f1].vertex(i10);
        assert!(self[f1].vertex(i11) == v3);
        assert!(self[f1].vertex(i12) == v1);

        self[f0].set_vertex(i02, v2);
        self[f1].set_vertex(i12, v0);
        self[v0].set_face(f0);
        self[v1].set_face(f0);
        self[v2].set_face(f0);
        self[v3].set_face(f1);

        self.move_adjacent(f0, i00, f1, i11);
        self.move_adjacent(f1, i10, f0, i01);
        self.set_adjacent(f0, i01, f1, i11);

        self.copy_constraint(f1, i11, f0, i00);
        self.copy_constraint(f0, i01, f1, i10);
        self.clear_constraint(f0, i01);
        self.clear_constraint(f1, i11);
    }
}

impl<PR, V, F, C> Updater for Triangulation<PR::Position, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: PredicatesContext<Predicates = PR>,
{
    fn split_edge(&mut self, face: FaceIndex, edge: Rot3, vert: VertexIndex) {
        match self.dimension() {
            1 => self.split_edge_dim1(face, edge, vert),
            2 => self.split_edge_dim2(face, edge, vert),
            _ => panic!("invalid dimension for edge split: {}", self.dimension()),
        };
    }

    fn split_face(&mut self, face: FaceIndex, vert: VertexIndex) {
        match self.dimension() {
            1 => self.split_face_dim1(face, vert),
            2 => self.split_face_dim2(face, vert),
            _ => panic!("invalid dimension for face split: {}", self.dimension()),
        };
    }

    fn extend_dimension(&mut self, vert: VertexIndex) {
        match self.dimension() {
            -1 => self.extend_to_dim0(vert),
            0 => self.extend_to_dim1(vert),
            1 => self.extend_to_dim2(vert),
            _ => panic!("invalid dimension for face split: {}", self.dimension()),
        };
    }

    fn flip(&mut self, face: FaceIndex, edge: Rot3) {
        assert_eq!(self.dimension(), 2);
        assert!(face.is_valid() && edge.is_valid());
        self.flip_face(face, edge);
    }
}
