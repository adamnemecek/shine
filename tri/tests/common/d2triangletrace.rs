#![allow(dead_code)]

use shine_testutils::webserver::*;
use shine_tri::{TracePrimitive, Triangulation};

impl TracePrimitive for D2Trace {}

impl<'a, P, V, F, C> IntoD2Image for (&'a Triangulation<P, V, F, C>, &'a mut D2Trace)
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
    C: 'a + TraceContext,
{
}

pub fn trace_graph<'a, P, V, F>(
    graph: &'a Graph<P, V, F>,
    mapping: &'a RenderMapping,
    coloring: &'a Coloring,
) -> Trace<'a, P, V, F>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    Trace::new(graph, mapping, coloring)
}
