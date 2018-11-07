use geometry::{CollinearTest, Orientation, Position, Predicates};
use graph::{Constraint, Face, FaceExt, Vertex};
use indexing::{IndexGet, PositionQuery, VertexQuery};
use orientationquery::OrientationQuery;
use tagginglocator::{Location, TaggingLocator};
use traverse::{CrossingIterator, CrossingSide};
use triangulation::Triangulation;
use types::{invalid_face_index, invalid_vertex_index, rot3, vertex_index, FaceIndex, Rot3, VertexIndex};
use vertexchain::{ChainIndex, ChainStore};

pub trait Builder {
    type Position: Position;
    type Constraint: Constraint;

    /// Add a point to the triangulation and return the index of the generated vertex.
    /// A hint can be provided to find the triangle containing the given point.
    fn add_vertex(&mut self, p: Self::Position, hint: Option<FaceIndex>) -> VertexIndex;

    /// Add the two (new) vertices and a constraining edge between them.
    fn add_constraint_segment(&mut self, p0: Self::Position, p1: Self::Position, c: Self::Constraint);

    /// Add a constraining edge between the given vertices.
    fn add_constraint_edge(&mut self, v0: VertexIndex, v1: VertexIndex, c: Self::Constraint);
}

impl<PR, V, F> Triangulation<PR, V, F>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
{
    fn create_infinite_vertex(&mut self) -> VertexIndex {
        let v = self.graph.store_vertex(Default::default());
        self.graph.set_infinite_vertex(v);
        v
    }

    fn create_vertex_with_position(&mut self, p: PR::Position) -> VertexIndex {
        let mut v: V = Default::default();
        v.set_position(p);
        self.graph.store_vertex(v)
    }

    fn create_face(&mut self) -> FaceIndex {
        self.graph.store_face(Default::default())
    }

    fn create_face_with_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> FaceIndex {
        let mut f: F = Default::default();
        f.set_vertices(v0, v1, v2);
        self.graph.store_face(f)
    }

    fn copy_constraint(&mut self, f_from: FaceIndex, i_from: Rot3, f_to: FaceIndex, i_to: Rot3) {
        let c = self.graph[f_from].constraint(i_from);
        self.graph[f_to].set_constraint(i_to, c);
    }

    /// Sets the constraint of an edge by updating both adjacent faces.
    fn make_constraint(&mut self, face: FaceIndex, edge: Rot3, c: F::Constraint) {
        let nf = self.graph[face].neighbor(edge);
        let ni = self.graph[nf].get_neighbor_index(face).unwrap();
        self.graph[face].set_constraint(edge, c.clone());
        self.graph[nf].set_constraint(ni, c);
    }

    fn clear_constraint(&mut self, f: FaceIndex, i: Rot3) {
        self.graph[f].set_constraint(i, Default::default());
    }

    /// Set adjacent face information for two neighboring faces.
    fn set_adjacent(&mut self, f0: FaceIndex, i0: Rot3, f1: FaceIndex, i1: Rot3) {
        assert!(i0.is_valid() && i1.is_valid());
        assert!(i0.id() <= self.graph.dimension() as u8 && i1.id() <= self.graph.dimension() as u8);
        self.graph[f0].set_neighbor(i0, f1);
        self.graph[f1].set_neighbor(i1, f0);
    }

    /// Move adjacent face information from one face into another.
    fn move_adjacent(&mut self, f_target: FaceIndex, i_target: Rot3, f_source: FaceIndex, i_source: Rot3) {
        let n = self.graph[f_source].neighbor(i_source);
        let i = self.graph[n].get_neighbor_index(f_source).unwrap();
        self.set_adjacent(f_target, i_target, n, i);
    }

    /// Flips the edge in the quadrangle defined by the two neighboring triangles.
    pub fn flip(&mut self, face: FaceIndex, edge: Rot3) {
        assert_eq!(self.graph.dimension(), 2);
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

        let f1 = self.graph[f0].neighbor(i00);
        let i10 = self.graph[f1].get_neighbor_index(f0).unwrap();
        let i11 = i10.increment();
        let i12 = i10.decrement();

        let v0 = self.graph[f0].vertex(i00);
        let v1 = self.graph[f0].vertex(i01);
        let v3 = self.graph[f0].vertex(i02);
        let v2 = self.graph[f1].vertex(i10);
        assert!(self.graph[f1].vertex(i11) == v3);
        assert!(self.graph[f1].vertex(i12) == v1);

        self.graph[f0].set_vertex(i02, v2);
        self.graph[f1].set_vertex(i12, v0);
        self.graph[v0].set_face(f0);
        self.graph[v1].set_face(f0);
        self.graph[v2].set_face(f0);
        self.graph[v3].set_face(f1);

        self.move_adjacent(f0, i00, f1, i11);
        self.move_adjacent(f1, i10, f0, i01);
        self.set_adjacent(f0, i01, f1, i11);

        self.copy_constraint(f1, i11, f0, i00);
        self.copy_constraint(f0, i01, f1, i10);
        self.clear_constraint(f0, i01);
        self.clear_constraint(f1, i11);
    }

    fn add_vertex_at(&mut self, p: PR::Position, loc: Location) -> VertexIndex {
        match self.graph.dimension() {
            -1 => match loc {
                Location::Empty => self.extend_to_dim0(p),
                loc => unreachable!(format!("Invalid location for empty triangulation: {:?}", loc)),
            },
            0 => match loc {
                Location::Vertex(f, i) => self.graph[f].vertex(i),
                Location::OutsideAffineHull => self.extend_to_dim1(p),
                loc => unreachable!(format!("Invalid location for 0D triangulation: {:?}", loc)),
            },
            1 => match loc {
                Location::Vertex(f, i) => self.graph[f].vertex(i),
                Location::Edge(f, _) => self.split_edge_dim1(p, f),
                Location::OutsideConvexHull(f) => self.split_edge_dim1(p, f),
                Location::OutsideAffineHullClockwise => self.extend_to_dim2(p, false),
                Location::OutsideAffineHullCounterClockwise => self.extend_to_dim2(p, true),
                loc => unreachable!(format!("Invalid location for 1D triangulation: {:?}", loc)),
            },
            2 => match loc {
                Location::Vertex(f, i) => self.graph[f].vertex(i),
                Location::Edge(f, i) => self.split_edge_dim2(p, f, i),
                Location::Face(f) => self.split_face(p, f),
                Location::OutsideConvexHull(f) => self.insert_outside_convex_hull2(p, f),
                _ => unreachable!(format!("Invalid location for 2D triangulation: {:?}", loc)),
            },
            dim => unreachable!(format!("Invalid dimension: {}, {:?}", dim, loc)),
        }
    }

    fn extend_to_dim0(&mut self, p: PR::Position) -> VertexIndex {
        assert!(self.graph.dimension() == -1);
        assert!(!self.graph.infinite_vertex().is_valid());
        assert!(self.graph.vertex_count() == 0);
        assert!(self.graph.face_count() == 0);

        self.graph.set_dimension(0);

        let v0 = self.create_infinite_vertex();
        let v1 = self.create_vertex_with_position(p);
        let f0 = self.create_face_with_vertices(v0, invalid_vertex_index(), invalid_vertex_index());
        let f1 = self.create_face_with_vertices(v1, invalid_vertex_index(), invalid_vertex_index());

        self.graph[v0].set_face(f0);
        self.graph[v1].set_face(f1);
        self.set_adjacent(f0, rot3(0), f1, rot3(0));

        v1
    }

    /// Extends dimension from 0D to 1D by creating a segment (face) out of the (two) finite points.
    /// In 1D a face is a segment, and the shell is the triangular face (as described in extend_to_dim2). The
    /// infinite vertex is always the vertex corresponding to the 2nd index in each (finite) faces(segments).
    fn extend_to_dim1(&mut self, p: PR::Position) -> VertexIndex {
        assert!(self.graph.dimension() == 0);
        assert!(self.graph.vertex_count() == 2);
        assert!(self.graph.face_count() == 2);

        self.graph.set_dimension(1);

        // infinite, finite vertices
        let (v0, v1) = {
            let v0 = vertex_index(0);
            let v1 = vertex_index(1);
            if self.graph.is_infinite_vertex(v0) {
                (v0, v1)
            } else {
                (v1, v0)
            }
        };
        // finite (new) vertex
        let v2 = self.create_vertex_with_position(p);

        let f0 = self.graph[v0].face();
        let f1 = self.graph[v1].face();
        let f2 = self.create_face_with_vertices(v2, v0, invalid_vertex_index());

        self.graph[f0].set_vertex(rot3(1), v1);
        self.graph[f1].set_vertex(rot3(1), v2);
        self.graph[v2].set_face(f2);

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
    fn extend_to_dim2(&mut self, p: PR::Position, is_ccw: bool) -> VertexIndex {
        assert_eq!(self.graph.dimension(), 1);

        self.graph.set_dimension(2);

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
            let f0 = self.graph.infinite_face();
            let i0 = self.graph[f0].get_vertex_index(self.graph.infinite_vertex()).unwrap();
            let im = i0.mirror(2);
            let fm = self.graph[f0].neighbor(im);

            if is_ccw {
                (f0, i0, fm, im)
            } else {
                (fm, im, f0, i0)
            }
        };

        let c0 = self.graph[f0].neighbor(i0);

        let mut cur = c0;
        let mut new_face = invalid_face_index();
        while cur != fm {
            let prev_new_face = new_face;
            new_face = self.create_face();

            let v0 = self.graph[cur].vertex(rot3(1));
            let v1 = self.graph[cur].vertex(rot3(0));
            let vinf = self.graph.infinite_vertex();
            if i0 == rot3(1) {
                self.graph[new_face].set_vertices(v0, v1, new_vertex);
                self.graph[cur].set_vertex(rot3(2), vinf);
                self.graph[new_vertex].set_face(new_face);
            } else {
                self.graph[new_face].set_vertices(v0, v1, vinf);
                self.graph[cur].set_vertex(rot3(2), new_vertex);
                self.graph[new_vertex].set_face(cur);
            }

            self.set_adjacent(cur, rot3(2), new_face, rot3(2));
            if prev_new_face.is_valid() {
                self.set_adjacent(prev_new_face, im, new_face, i0);
            }

            self.copy_constraint(cur, rot3(2), new_face, rot3(2));

            cur = self.graph[cur].neighbor(i0);
        }

        let cm = self.graph[fm].neighbor(im);
        let n0 = self.graph[c0].neighbor(rot3(2));
        let nm = self.graph[cm].neighbor(rot3(2));

        self.graph[f0].set_vertex(rot3(2), new_vertex);
        self.graph[fm].set_vertex(rot3(2), new_vertex);

        if i0 == rot3(1) {
            self.graph[f0].swap_vertices(rot3(2), rot3(1));
            self.graph[fm].swap_vertices(rot3(0), rot3(2));
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

    fn split_edge_dim1(&mut self, p: PR::Position, f: FaceIndex) -> VertexIndex {
        assert!(self.graph.dimension() == 1);

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
        let f1 = self.graph[f0].neighbor(rot3(0));
        let i = self.graph[f1].get_neighbor_index(f0).unwrap();
        let v1 = self.graph[f1].vertex(i.mirror(2)); // j = 1-i

        self.graph[v1].set_face(f1);
        self.graph[v2].set_face(f2);
        self.graph[f0].set_vertex(rot3(1), v2);
        self.graph[f2].set_vertices(v2, v1, invalid_vertex_index());
        self.set_adjacent(f2, rot3(1), f0, rot3(0));
        self.set_adjacent(f2, rot3(0), f1, i);

        self.copy_constraint(f0, rot3(2), f2, rot3(2));

        v2
    }

    fn split_edge_dim2(&mut self, p: PR::Position, face: FaceIndex, edge: Rot3) -> VertexIndex {
        assert_eq!(self.graph.dimension(), 2);

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
        let f1 = self.graph[f0].neighbor(edge);
        let i00 = edge.increment();
        let i01 = edge.decrement();
        let i02 = edge;
        let i12 = self.graph[f1].get_neighbor_index(f0).unwrap();
        let i11 = i12.decrement();
        let i10 = i12.increment();

        let v0 = self.graph[f0].vertex(i02);
        //let v1 = self.graph[f0].vertex(i00);
        let v2 = self.graph[f0].vertex(i01);
        let v3 = self.graph[f1].vertex(i12);

        self.graph[n0].set_vertices(vp, v2, v0);
        self.graph[n1].set_vertices(vp, v3, v2);
        self.graph[f0].set_vertex(i01, vp);
        self.graph[f1].set_vertex(i10, vp);
        self.graph[vp].set_face(n0);
        self.graph[v2].set_face(n0);
        self.graph[v0].set_face(n0);
        self.graph[v3].set_face(n1);

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

    fn split_face(&mut self, p: PR::Position, face: FaceIndex) -> VertexIndex {
        assert_eq!(self.graph.dimension(), 2);

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

        let v0 = self.graph[f0].vertex(rot3(0));
        let v1 = self.graph[f0].vertex(rot3(1));
        let v2 = self.graph[f0].vertex(rot3(2));

        self.graph[n0].set_vertices(v0, vp, v2);
        self.graph[n1].set_vertices(vp, v1, v2);
        self.graph[f0].set_vertex(rot3(2), vp);
        self.graph[vp].set_face(f0);
        self.graph[v2].set_face(n0);

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

    fn insert_outside_convex_hull2(&mut self, p: PR::Position, face: FaceIndex) -> VertexIndex {
        let f0 = face;
        let vinf = self.graph.infinite_vertex();
        let i = self.graph[f0].get_vertex_index(vinf).unwrap();
        let mut fcw = self.graph[f0].neighbor(i.decrement());
        let mut fccw = self.graph[f0].neighbor(i.increment());

        // split the infinite triangle by the given position
        let vp = self.split_face(p, face);

        //correct faces by flipping
        loop {
            let i = self.graph[fcw].get_vertex_index(vinf).unwrap();
            let next = self.graph[fcw].neighbor(i.decrement());
            if !self.get_edge_vertex_orientation(fcw, i, vp).is_ccw() {
                break;
            }
            self.flip(fcw, i.increment());
            fcw = next;
        }

        loop {
            let i = self.graph[fccw].get_vertex_index(vinf).unwrap();
            let next = self.graph[fccw].neighbor(i.increment());
            if !self.get_edge_vertex_orientation(fccw, i, vp).is_ccw() {
                break;
            }
            self.flip(fccw, i.decrement());
            fccw = next;
        }

        vp
    }

    /// Adds the constraining edge between the two vertex when dim=1 (all faces are segments)
    fn add_constraint_dim1(&mut self, v0: VertexIndex, v1: VertexIndex, c: &F::Constraint) {
        assert!(self.graph.is_finite_vertex(v0));
        assert!(self.graph.is_finite_vertex(v1));
        assert_ne!(v1, v0);

        // start by the face of the first vertex
        let f0 = self.graph[v0].face();
        let i0 = self.graph[f0].get_vertex_index(v0).unwrap();

        // next vertex
        let vn = self.graph[f0].vertex(i0.mirror(2));
        if vn == v1 {
            // v0-v1 edge was just found
            self.graph[f0].set_constraint(rot3(2), c.clone());
            return;
        }

        // find the direction to reach v1 from v0
        let reverse_dir = if self.graph.is_finite_vertex(vn) {
            // test direction to traverse by point order
            let p0 = &self.graph[PositionQuery::Vertex(v0)];
            let p1 = &self.graph[PositionQuery::Vertex(v1)];
            let pn = &self.graph[PositionQuery::Vertex(vn)];
            // p0,p1,pn and any other (finite) point must be collinear as dim==1,
            let direction = self.predicates.test_collinear_points(p0, p1, pn);
            assert!(
                direction.is_before() || direction.is_between(),
                "Internal error, direction test result"
            );
            direction.is_before()
        } else {
            true
        };

        let (mut cur, mut cur_i) = if reverse_dir {
            // opposite direction
            let next = self.graph[f0].neighbor(i0.mirror(2));
            let next_i = self.graph[next].get_neighbor_index(f0).unwrap().mirror(2);
            (next, next_i)
        } else {
            (f0, i0)
        };

        // mark all edges constraint until the end vertex is reached
        // no geometry have to be tested, as we are on a straight line and no segment may overlap
        loop {
            self.graph[cur].set_constraint(rot3(2), c.clone());
            if self.graph[cur].vertex(cur_i.mirror(2)) == v1 {
                break;
            }

            let next = self.graph[cur].neighbor(cur_i);
            cur_i = self.graph[next].get_neighbor_index(cur).unwrap().mirror(2);
            cur = next;
        }
    }

    /// Adds the constraining edge between the two vertex when dim=2
    fn add_constraint_dim2(&mut self, v0: VertexIndex, v1: VertexIndex, c: &F::Constraint) {
        let mut chain_store = ChainStore::new();
        let mut start = v0;
        while start != v1 {
            // collect intersecting faces and generate the two (top/bottom) chains
            // The edge-chain is not a whole polygon the new constraining edge is the missing closing edge

            let (face, edge) = {
                let mut crossing_iter = CrossingIterator::new(self, start, v1);

                let cross = crossing_iter.next().unwrap();
                if cross.side == CrossingSide::Start {
                    let mut top_chain = chain_store.new_chain(cross.face, cross.edge.increment(), true);
                    let bottom_chain = chain_store.new_chain(cross.face, cross.edge.decrement(), true);

                    while let Some(cross) = crossing_iter.next() {
                        //if self.tri.graph.getConstraint( edge ) ) {
                        // insert constraint-constraint intersection points
                        //unimplemented!( "Not implemented" );
                        //}

                        match cross.side {
                            CrossingSide::CCW => {
                                top_chain = chain_store.insert_before(top_chain, cross.face, cross.edge.increment());
                            }
                            CrossingSide::CW => {
                                chain_store.insert_before(bottom_chain, cross.face, cross.edge.decrement());
                            }
                            CrossingSide::End => {
                                top_chain = chain_store.insert_before(top_chain, cross.face, cross.edge.increment());
                                chain_store.insert_before(bottom_chain, cross.face, cross.edge.decrement());
                                chain_store.split_before(top_chain);
                                chain_store.split_before(bottom_chain);
                            }
                            _ => unreachable!(),
                        };
                    }

                    //edge = triangulateHole( chain_store, top_chain, bottom_chain );
                    //startV = tri_.getEndVertex( edge );
                    chain_store.clear();
                    unimplemented!()
                } else {
                    (cross.face, cross.edge)
                }
            };

            self.make_constraint(face, edge, c.clone());
            start = self.graph.index_get(VertexQuery::EdgeEnd(face, edge));
        }
    }
}

impl<PR, V, F> Builder for Triangulation<PR, V, F>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
{
    type Position = PR::Position;
    type Constraint = F::Constraint;

    fn add_vertex(&mut self, p: PR::Position, hint: Option<FaceIndex>) -> VertexIndex {
        let location = self.locate_position(&p, hint).unwrap();
        self.add_vertex_at(p, location)
    }

    fn add_constraint_segment(&mut self, p0: PR::Position, p1: PR::Position, c: F::Constraint) {
        assert!(c.is_constraint());
        let v0 = self.add_vertex(p0, None);
        let start_face = self.graph[v0].face();
        let v1 = self.add_vertex(p1, Some(start_face));
        self.add_constraint_edge(v0, v1, c);
    }

    fn add_constraint_edge(&mut self, v0: VertexIndex, v1: VertexIndex, c: F::Constraint) {
        assert!(c.is_constraint());
        assert!(v0.is_valid());
        assert!(v1.is_valid());
        assert!(self.graph.is_finite_vertex(v0));
        assert!(self.graph.is_finite_vertex(v1));
        if v0 == v1 {
            return;
        }

        match self.graph.dimension() {
            1 => self.add_constraint_dim1(v0, v1, &c),
            2 => self.add_constraint_dim2(v0, v1, &c),
            _ => unreachable!("Inconsistent triangulation"),
        }
    }
}
