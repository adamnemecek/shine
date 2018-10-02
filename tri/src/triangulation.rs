use geometry::{Orientation, Predicates};
use locator::Locator;
use types::{Edge, FaceIndex, Location, Rot3, VertexIndex};

pub trait TriStore {
    type Position;
    type Constraint: Default;

    fn get_vertex_count(&self) -> usize;
    fn get_face_count(&self) -> usize;

    fn clear(&mut self);

    fn create_vertex(&mut self) -> VertexIndex;
    fn get_vertex_position(&self, v: VertexIndex) -> &Self::Position;
    fn set_vertex_position(&mut self, v: VertexIndex, p: Self::Position);
    fn get_vertex_face(&self, v: VertexIndex) -> FaceIndex;
    fn set_vertex_face(&mut self, v: VertexIndex, f: FaceIndex);

    fn create_face(&mut self) -> FaceIndex;
    fn get_face_vertex(&self, f: FaceIndex, i: Rot3) -> VertexIndex;
    fn set_face_vertex(&mut self, f: FaceIndex, i: Rot3, v: VertexIndex);
    fn get_face_neighbor(&self, f: FaceIndex, i: Rot3) -> FaceIndex;
    fn set_face_neighbor(&mut self, f: FaceIndex, i: Rot3, nf: FaceIndex);
    fn swap_face_vertices(&mut self, f: FaceIndex, i0: Rot3, i1: Rot3);

    fn get_constraint(&self, f: FaceIndex, i: Rot3) -> Self::Constraint;
    fn set_constraint(&self, f: FaceIndex, i: Rot3, c: Self::Constraint);
}

/// (Constraint) Triangulation.
pub struct TriGraph<P, T>
where
    P: Predicates,
    T: TriStore<Position = <P as Predicates>::Position>,
{
    crate predicates: P,
    crate store: T,
    crate dimension: i8,
    crate infinite_vertex: VertexIndex,
}

impl<P, T> TriGraph<P, T>
where
    P: Predicates,
    T: TriStore<Position = <P as Predicates>::Position>,
{
    pub fn new(predicates: P, store: T) -> TriGraph<P, T> {
        TriGraph {
            predicates,
            store,
            dimension: -1,
            infinite_vertex: VertexIndex::invalid(),
        }
    }

    pub fn get_dimension(&self) -> i8 {
        self.dimension
    }

    pub fn is_empty(&self) -> bool {
        self.dimension == -1
    }

    pub fn get_vertex_count(&self) -> usize {
        self.store.get_vertex_count()
    }

    pub fn get_face_count(&self) -> usize {
        self.store.get_face_count()
    }

    pub fn clear(&mut self) {
        self.store.clear();
        self.dimension = 0;
        self.infinite_vertex = VertexIndex::invalid();
    }

    //region vertex methods
    pub fn get_vertex_position(&self, v: VertexIndex) -> &T::Position {
        self.store.get_vertex_position(v)
    }

    pub fn set_vertex_position(&mut self, v: VertexIndex, p: T::Position) {
        self.store.set_vertex_position(v, p);
    }

    pub fn create_vertex(&mut self) -> VertexIndex {
        self.store.create_vertex()
    }

    pub fn create_vertex_with_infinite_position(&mut self) -> VertexIndex {
        assert!(!self.infinite_vertex.is_valid());
        self.infinite_vertex = self.create_vertex();
        self.infinite_vertex
    }

    pub fn create_vertex_with_position(&mut self, p: T::Position) -> VertexIndex {
        let vertex = self.create_vertex();
        self.set_vertex_position(vertex, p);
        vertex
    }

    pub fn get_infinite_vertex(&self) -> VertexIndex {
        self.infinite_vertex
    }

    pub fn is_finite_vertex(&self, v: VertexIndex) -> bool {
        v != self.infinite_vertex
    }

    pub fn get_vertex_face(&self, v: VertexIndex) -> FaceIndex {
        self.store.get_vertex_face(v)
    }

    pub fn set_vertex_face(&mut self, v: VertexIndex, f: FaceIndex) {
        self.store.set_vertex_face(v, f);
    }
    //endregion

    //region face methods
    pub fn create_face(&mut self) -> FaceIndex {
        self.store.create_face()
    }

    pub fn create_face_with_vertices(&mut self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> FaceIndex {
        let face = self.create_face();
        self.set_face_vertices(face, v0, v1, v2);
        face
    }

    pub fn get_infinite_face(&self) -> FaceIndex {
        self.get_vertex_face(self.infinite_vertex)
    }

    pub fn is_finite_face(&self, f: FaceIndex) -> bool {
        self.get_face_vertex_index(f, self.infinite_vertex).is_none()
    }

    pub fn get_face_vertex(&self, f: FaceIndex, i: Rot3) -> VertexIndex {
        assert!(i.is_valid());
        self.store.get_face_vertex(f, i)
    }

    pub fn get_face_vertex_position(&self, f: FaceIndex, i: Rot3) -> &T::Position {
        let v = self.get_face_vertex(f, i);
        self.get_vertex_position(v)
    }

    pub fn get_face_vertex_index(&self, f: FaceIndex, v: VertexIndex) -> Option<Rot3> {
        for i in 0..3 {
            let r = Rot3(i);
            if self.get_face_vertex(f, r) == v {
                return Some(r);
            }
        }
        None
    }

    pub fn set_face_vertex(&mut self, f: FaceIndex, i: Rot3, v: VertexIndex) {
        assert!(i.is_valid());
        self.store.set_face_vertex(f, i, v);
    }

    pub fn set_face_vertices(&mut self, f: FaceIndex, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) {
        self.store.set_face_vertex(f, Rot3(0), v0);
        self.store.set_face_vertex(f, Rot3(1), v1);
        self.store.set_face_vertex(f, Rot3(2), v2);
    }

    pub fn get_face_neighbor(&self, f: FaceIndex, i: Rot3) -> FaceIndex {
        assert!(i.is_valid());
        self.store.get_face_neighbor(f, i)
    }

    pub fn get_face_neighbor_index(&self, f: FaceIndex, nf: FaceIndex) -> Option<Rot3> {
        for i in 0..3 {
            let r = Rot3(i);
            if self.get_face_neighbor(f, r) == nf {
                return Some(r);
            }
        }
        None
    }

    pub fn set_face_neighbor(&mut self, f: FaceIndex, i: Rot3, nf: FaceIndex) {
        assert!(i.is_valid());
        self.store.set_face_neighbor(f, i, nf);
    }
    //endregion

    //region topology modificationface methods
    pub fn swap_face_vertices(&mut self, f: FaceIndex, i0: Rot3, i1: Rot3) {
        assert!(i0.is_valid() && i1.is_valid());
        self.store.swap_face_vertices(f, i0, i1);
    }

    pub fn set_adjacent(&mut self, f0: FaceIndex, i0: Rot3, f1: FaceIndex, i1: Rot3) {
        assert!(i0.is_valid() && i1.is_valid());
        assert!(i0.0 <= self.dimension as u8 && i1.0 <= self.dimension as u8);
        self.set_face_neighbor(f0, i0, f1);
        self.set_face_neighbor(f1, i1, f0);
    }

    /// Move adjacent face information from one triangle into another.
    pub fn move_adjacent(&mut self, target_f: FaceIndex, target_i: Rot3, source_f: FaceIndex, source_i: Rot3) {
        let n = self.get_face_neighbor(source_f, source_i);
        let i = self.get_face_neighbor_index(n, source_f).unwrap();
        self.set_adjacent(target_f, target_i, n, i);
    }

    /// Flips the edge in the quadrangle defined by the two neighboring triangles.
    pub fn flip(&mut self, face: FaceIndex, edge: Rot3) {
        assert!(self.dimension == 2);
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

        let f1 = self.get_face_neighbor(f0, i00);
        let i10 = self.get_face_neighbor_index(f1, f0).unwrap();
        let i11 = i10.increment();
        let i12 = i10.decrement();

        let v0 = self.get_face_vertex(f0, i00);
        let v1 = self.get_face_vertex(f0, i01);
        let v3 = self.get_face_vertex(f0, i02);
        let v2 = self.get_face_vertex(f1, i10);
        assert!(self.get_face_vertex(f1, i11) == v3);
        assert!(self.get_face_vertex(f1, i12) == v1);

        self.set_face_vertex(f0, i02, v2);
        self.set_face_vertex(f1, i12, v0);
        self.set_vertex_face(v0, f0);
        self.set_vertex_face(v1, f0);
        self.set_vertex_face(v2, f0);
        self.set_vertex_face(v3, f1);

        self.move_adjacent(f0, i00, f1, i11);
        self.move_adjacent(f1, i10, f0, i01);
        self.set_adjacent(f0, i01, f1, i11);

        self.set_constraint(f0, i00, self.get_constraint(f1, i11));
        self.set_constraint(f1, i10, self.get_constraint(f0, i01));
        self.clear_constraint(f0, i01);
        self.clear_constraint(f1, i11);
    }
    //endregion

    //region constraints
    pub fn get_constraint(&self, f: FaceIndex, i: Rot3) -> T::Constraint {
        assert!(i.is_valid());
        self.store.get_constraint(f, i)
    }

    pub fn set_constraint(&self, f: FaceIndex, i: Rot3, c: T::Constraint) {
        assert!(i.is_valid());
        self.store.set_constraint(f, i, c);
    }

    pub fn clear_constraint(&self, f: FaceIndex, i: Rot3) {
        assert!(i.is_valid());
        self.store.set_constraint(f, i, Default::default());
    }
    //endregion

    //region query
    pub fn locate(&self, p: &T::Position, hint: Option<FaceIndex>) -> Location {
        let mut locator = Locator::new(self);
        locator.locate_position(p, hint).unwrap()
    }
    //endregion

    //region geometry relationship
    pub fn get_orientation_vertices(&self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> Orientation {
        assert!(v0 != self.infinite_vertex && v1 != self.infinite_vertex && v2 != self.infinite_vertex);
        let a = self.get_vertex_position(v0);
        let b = self.get_vertex_position(v1);
        let c = self.get_vertex_position(v2);
        self.predicates.orientation(a, b, c)
    }

    /// Finds the orientation of an edge and a vertex    
    pub fn get_orientation_edge_vertex(&self, e: Edge, v: VertexIndex) -> Orientation {
        let va = v;
        let vb = self.get_face_vertex(e.0, e.1.increment());
        let vc = self.get_face_vertex(e.0, e.1.decrement());
        self.get_orientation_vertices(va, vb, vc)
    }
    //endregion
}
