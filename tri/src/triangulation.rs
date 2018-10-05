use geometry::{Orientation, Position, Predicates};
use std::ops::{Index, IndexMut};
use types::{Edge, FaceIndex, Rot3, VertexIndex};

/// A vertex of the triangulation
pub struct Vertex<P, D = ()>
where
    P: Position + Default,
    D: Default,
{
    pub position: P,
    pub face: FaceIndex,
    pub data: D,
}

impl<P, D> Vertex<P, D>
where
    P: Position + Default,
    D: Default,
{
    pub fn new() -> Vertex<P, D> {
        Vertex {
            position: Default::default(),
            face: FaceIndex::invalid(),
            data: Default::default(),
        }
    }
}

impl<C, D> Default for Vertex<C, D>
where
    C: Position + Default,
    D: Default,
{
    fn default() -> Vertex<C, D> {
        Vertex::new()
    }
}

/// Container to store vertices of a face
pub struct FaceVertices([VertexIndex; 3]);

impl FaceVertices {
    pub fn index_of(&self, v: VertexIndex) -> Option<Rot3> {
        self.0.iter().position(|&nf| nf == v).map(|i| Rot3(i as u8))
    }

    pub fn swap(&mut self, a: Rot3, b: Rot3) {
        self.0.swap(a.0 as usize, b.0 as usize);
    }
}

impl Default for FaceVertices {
    fn default() -> FaceVertices {
        FaceVertices([VertexIndex::invalid(); 3])
    }
}

impl Index<Rot3> for FaceVertices {
    type Output = VertexIndex;

    fn index(&self, i: Rot3) -> &Self::Output {
        self.0.index(i.0 as usize)
    }
}

impl IndexMut<Rot3> for FaceVertices {
    fn index_mut(&mut self, i: Rot3) -> &mut Self::Output {
        self.0.index_mut(i.0 as usize)
    }
}

/// Container to store neighbors of a face
pub struct FaceNeighbors([FaceIndex; 3]);

impl FaceNeighbors {
    pub fn index_of(&self, v: FaceIndex) -> Option<Rot3> {
        self.0.iter().position(|&nf| nf == v).map(|i| Rot3(i as u8))
    }

    pub fn swap(&mut self, a: Rot3, b: Rot3) {
        self.0.swap(a.0 as usize, b.0 as usize);
    }
}

impl Default for FaceNeighbors {
    fn default() -> FaceNeighbors {
        FaceNeighbors([FaceIndex::invalid(); 3])
    }
}

impl Index<Rot3> for FaceNeighbors {
    type Output = FaceIndex;

    fn index(&self, i: Rot3) -> &Self::Output {
        self.0.index(i.0 as usize)
    }
}

impl IndexMut<Rot3> for FaceNeighbors {
    fn index_mut(&mut self, i: Rot3) -> &mut Self::Output {
        self.0.index_mut(i.0 as usize)
    }
}

/// Container to store edge data of a face
pub struct FaceEdges<E>([E; 3]);

impl<E> FaceEdges<E> {
    /*pub fn index_of(&self, v: FaceIndex) -> Option<Rot3> {
        self.0.iter().position(|&nf| nf == v).map(|i| Rot3(i as u8))
    }*/

    pub fn swap(&mut self, a: Rot3, b: Rot3) {
        self.0.swap(a.0 as usize, b.0 as usize);
    }
}

impl<E> Default for FaceEdges<E>
where
    E: Copy + Default,
{
    fn default() -> FaceEdges<E> {
        FaceEdges([Default::default(); 3])
    }
}

impl<E> Index<Rot3> for FaceEdges<E> {
    type Output = E;

    fn index(&self, i: Rot3) -> &Self::Output {
        self.0.index(i.0 as usize)
    }
}

impl<E> IndexMut<Rot3> for FaceEdges<E> {
    fn index_mut(&mut self, i: Rot3) -> &mut Self::Output {
        self.0.index_mut(i.0 as usize)
    }
}

/// A face of the triangualtion
pub struct Face<E, D = ()>
where
    E: Copy + Default,
    D: Default,
{
    pub vertices: FaceVertices,
    pub neighbors: FaceNeighbors,
    pub edges: FaceEdges<E>,
    pub data: D,
}

impl<E, D> Face<E, D>
where
    E: Copy + Default,
    D: Default,
{
    pub fn new() -> Face<E, D> {
        Face {
            vertices: Default::default(),
            neighbors: Default::default(),
            edges: Default::default(),
            data: Default::default(),
        }
    }

    /// Swaps the two vertices.
    pub fn swap_vertices(&mut self, i0: Rot3, i1: Rot3) {
        assert!(i0.is_valid());
        assert!(i1.is_valid());
        assert!(i0 != i1);
        self.vertices.swap(i0.into(), i1.into());
        self.neighbors.swap(i0.into(), i1.into());
        self.edges.swap(i0.into(), i1.into());
    }
}

impl<E, D> Default for Face<E, D>
where
    E: Copy + Default,
    D: Default,
{
    fn default() -> Face<E, D> {
        Face::new()
    }
}

pub trait Tri {
    type Position: Position + Default;
    type Predicate;
    type VertexData: Default;
    type EdgeData: Copy + Default;
    type FaceData: Default;
}

/// (Constraint) Triangulation.
pub struct TriGraph<T>
where
    T: Tri,
{
    crate predicates: T::Predicate,
    crate dimension: i8,
    crate vertices: Vec<Vertex<T::Position, T::VertexData>>,
    crate faces: Vec<Face<T::EdgeData, T::FaceData>>,
    crate infinite_vertex: VertexIndex,
}

impl<T> TriGraph<T>
where
    T: Tri,
{
    pub fn new(predicates: T::Predicate) -> TriGraph<T> {
        TriGraph {
            predicates,
            dimension: -1,
            vertices: Default::default(),
            faces: Default::default(),
            infinite_vertex: VertexIndex::invalid(),
        }
    }

    pub fn get_dimension(&self) -> i8 {
        self.dimension
    }

    //region container
    pub fn is_empty(&self) -> bool {
        self.dimension == -1
    }

    pub fn get_vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn get_face_count(&self) -> usize {
        self.faces.len()
    }

    pub fn clear(&mut self) {
        self.dimension = 0;
        self.faces.clear();
        self.vertices.clear();
        self.infinite_vertex = VertexIndex::invalid();
    }

    pub fn vertex(&self, v: VertexIndex) -> &Vertex<T::Position, T::VertexData> {
        &self.vertices[v.0]
    }

    pub fn vertex_mut(&mut self, v: VertexIndex) -> &mut Vertex<T::Position, T::VertexData> {
        &mut self.vertices[v.0]
    }

    pub fn store_vertex(&mut self, vert: Vertex<T::Position, T::VertexData>) -> VertexIndex {
        self.vertices.push(vert);
        VertexIndex(self.vertices.len() - 1)
    }

    pub fn store_infinite_vertex(&mut self) -> VertexIndex {
        assert!(!self.infinite_vertex.is_valid());
        self.infinite_vertex = self.store_vertex(Default::default());
        self.infinite_vertex
    }

    pub fn face(&self, f: FaceIndex) -> &Face<T::EdgeData, T::FaceData> {
        &self.faces[f.0]
    }

    pub fn face_mut(&mut self, f: FaceIndex) -> &mut Face<T::EdgeData, T::FaceData> {
        &mut self.faces[f.0]
    }

    pub fn store_face(&mut self, face: Face<T::EdgeData, T::FaceData>) -> FaceIndex {
        self.faces.push(face);
        FaceIndex(self.faces.len() - 1)
    }
    //endregion

    //region infinite elements
    pub fn get_infinite_vertex(&self) -> VertexIndex {
        self.infinite_vertex
    }

    pub fn is_infinite_vertex(&self, v: VertexIndex) -> bool {
        assert!(!self.is_empty());
        v == self.infinite_vertex
    }

    pub fn is_finite_vertex(&self, v: VertexIndex) -> bool {
        !self.is_infinite_vertex(v)
    }

    pub fn get_infinite_face(&self) -> FaceIndex {
        self[self.get_infinite_vertex()].face
    }

    pub fn is_infinite_face(&self, f: FaceIndex) -> bool {
        assert!(!self.is_empty());
        self[f].vertices.index_of(self.infinite_vertex).is_some()
    }

    pub fn is_finite_face(&self, f: FaceIndex) -> bool {
        !self.is_infinite_face(f)
    }
    //endregion

    //region topology modificationface methods
    /// Set adjacent face information for two neighboring faces.
    pub fn set_adjacent(&mut self, f0: FaceIndex, i0: Rot3, f1: FaceIndex, i1: Rot3) {
        assert!(i0.is_valid() && i1.is_valid());
        assert!(i0.0 <= self.dimension as u8 && i1.0 <= self.dimension as u8);
        self[f0].neighbors[i0] = f1;
        self[f1].neighbors[i1] = f0;
    }

    /// Move adjacent face information from one face into another.
    pub fn move_adjacent(&mut self, target_f: FaceIndex, target_i: Rot3, source_f: FaceIndex, source_i: Rot3) {
        let n = self[source_f].neighbors[source_i];
        let i = self[n].neighbors.index_of(source_f).unwrap();
        self.set_adjacent(target_f, target_i, n, i);
    }

    /// Flips the edge in the quadrangle defined by the two neighboring triangles.
    pub fn flip(&mut self, edge: Edge) {
        assert!(self.dimension == 2);

        let Edge(face, edge) = edge;
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

        let f1 = self[f0].neighbors[i00];
        let i10 = self[f1].neighbors.index_of(f0).unwrap();
        let i11 = i10.increment();
        let i12 = i10.decrement();

        let v0 = self[f0].vertices[i00];
        let v1 = self[f0].vertices[i01];
        let v3 = self[f0].vertices[i02];
        let v2 = self[f1].vertices[i10];
        assert!(self[f1].vertices[i11] == v3);
        assert!(self[f1].vertices[i12] == v1);

        self[f0].vertices[i02] = v2;
        self[f1].vertices[i12] = v0;
        self[v0].face = f0;
        self[v1].face = f0;
        self[v2].face = f0;
        self[v3].face = f1;

        self.move_adjacent(f0, i00, f1, i11);
        self.move_adjacent(f1, i10, f0, i01);
        self.move_adjacent(f0, i01, f1, i11);

        self[f0].edges[i00] = self[f1].edges[i11];
        self[f1].edges[i10] = self[f0].edges[i01];
        self[f0].edges[i01] = Default::default();
        self[f1].edges[i11] = Default::default();
    }
    //endregion

    //region geometry relationship
   /* pub fn get_vertices_orientation(&self, v0: VertexIndex, v1: VertexIndex, v2: VertexIndex) -> Orientation {
        assert!(v0 != self.infinite_vertex && v1 != self.infinite_vertex && v2 != self.infinite_vertex);
        let a = self[v0].position;
        let b = self[v1].position;
        let c = self[v2].position;
        self.predicates.orientation(a, b, c)
    }

    /// Finds the orientation of an edge and a vertex    
    pub fn get_edge_vertex_orientation(&self, e: Edge, v: VertexIndex) -> Orientation {
        let va = v;
        let vb = self[e.0].vertices[e.1.increment().into()];
        let vc = self[e.0].vertices[e.1.decrement().into()];
        self.get_vertices_orientation(va, vb, vc)
    }*/

    //fn is_convex(&self, edge: Edge) -> bool {}
    //endregion
}

impl<T> Index<VertexIndex> for TriGraph<T>
where
    T: Tri,
{
    type Output = Vertex<T::Position, T::VertexData>;

    fn index(&self, v: VertexIndex) -> &Self::Output {
        self.vertex(v)
    }
}

impl<T> IndexMut<VertexIndex> for TriGraph<T>
where
    T: Tri,
{
    fn index_mut(&mut self, v: VertexIndex) -> &mut Self::Output {
        self.vertex_mut(v)
    }
}

impl<T> Index<FaceIndex> for TriGraph<T>
where
    T: Tri,
{
    type Output = Face<T::EdgeData, T::FaceData>;

    fn index(&self, f: FaceIndex) -> &Self::Output {
        self.face(f)
    }
}

impl<T> IndexMut<FaceIndex> for TriGraph<T>
where
    T: Tri,
{
    fn index_mut(&mut self, f: FaceIndex) -> &mut Self::Output {
        self.face_mut(f)
    }
}
