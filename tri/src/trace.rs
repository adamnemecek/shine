use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use indexing::PositionIndex;
use inexactgeometry::{InexactPosition64, InexactPredicates64};
use types::{FaceIndex, Rot3, VertexIndex};

impl Default for RenderMapping {
    fn default() -> RenderMapping {
        RenderMapping::new()
    }
}

/// Render triangulation for tracing
pub trait Trace<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn set_viewport(&mut self, min: (f64, f64), max: (f64, f64));

    fn add_vertex(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, v: VertexIndex, msg: Option<&str>);
    fn add_edge(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, a: VertexIndex, b: VertexIndex, msg: Option<&str>);
    fn add_face_edge(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, i: Rot3, msg: Option<&str>);
    fn add_face(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, msg: Option<&str>);
    fn add_circum_circle(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, msg: Option<&str>);
}

pub trait TraceExt<P, V, F>: Trace<P, V, F>
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
}
impl<P, V, F, T> TraceExt<P, V, F> for T
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
    T: Trace<P, V, F>,
{}
