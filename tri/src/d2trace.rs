#![cfg(test)]
use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use indexing::PositionIndex;
use inexactgeometry::InexactPosition64;
use shine_testutils::*;
use trace::{RenderMapping, Trace};
use types::{rot3, FaceIndex, Rot3, VertexIndex};

impl<P, V, F> Trace<P, V, F> for webserver::D2Trace
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
{
    fn set_viewport(&mut self, min: (f64, f64), max: (f64, f64)) {
        self.set_scale(min.0, min.1, max.0, max.1);
    }

    fn add_vertex(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, v: VertexIndex, msg: Option<&str>)
    where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
        if !v.is_valid() {
            return;
        }

        let msg = msg.map(|m| format!("V, {}", m)).unwrap_or(format!("V, {}", v.0));

        if tri.is_finite_vertex(v) {
            let p = InexactPosition64::from(&tri[PositionIndex::Vertex(v)]);
            let color = mapping.coloring.vertex.clone();
            self.add_point(&(p.x, p.y), color);
            let color = mapping.coloring.vertex_text.clone();
            self.add_text(&(p.x, p.y), msg, color);
        } else {
            for p in mapping.virtual_positions.iter() {
                let color = mapping.coloring.infinite_vertex.clone();
                self.add_point(&(p.x, p.y), color);
                let color = mapping.coloring.infinite_vertex_text.clone();
                self.add_text(&(p.x, p.y), msg.clone(), color);
            }
        }
    }

    fn add_edge(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, a: VertexIndex, b: VertexIndex, msg: Option<&str>)
    where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
        if tri.is_infinite_vertex(a) || tri.is_infinite_vertex(b) {
            return;
        }

        let msg = msg.map(|m| format!("E, {}", m));

        let pa = InexactPosition64::from(&tri[PositionIndex::Vertex(a)]);
        let pb = InexactPosition64::from(&tri[PositionIndex::Vertex(b)]);
        let color = mapping.coloring.edge.clone();
        self.add_line(&(pa.x, pa.y), &(pb.x, pb.y), color);
        let x = (pa.x + pb.x) * 0.5;
        let y = (pa.y + pb.y) * 0.5;
        let color = mapping.coloring.edge_text.clone();
        if let Some(msg) = msg {
            self.add_text(&(x, y), msg, color);
        }
    }

    fn add_face_edge(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, i: Rot3, msg: Option<&str>) {
        //self.add_edge( tri, mapping, tri[aTri.getStartVertex( aEdge )], aTri.getEndVertex( aEdge ), aMessage, aColor );
    }

    fn add_face(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, msg: Option<&str>) {
        if !f.is_valid() {
            return;
        }

        let verts = [tri[f].vertex(rot3(0)), tri[f].vertex(rot3(1)), tri[f].vertex(rot3(2))];
        let positions = [
            mapping.map_vertex(tri, verts[0], verts[1], verts[2]),
            mapping.map_vertex(tri, verts[1], verts[2], verts[0]),
            mapping.map_vertex(tri, verts[2], verts[0], verts[1]),
        ];

        for edge in 0..3 {
            // vertex
            if positions[edge].is_visible() {
                let color = if positions[edge].is_virtual() {
                    mapping.coloring.face_text.clone()
                } else {
                    mapping.coloring.infinite_face_text.clone()
                };
                let p = positions[edge].position();
                self.add_text(&(p.x, p.y), format!("{}.{} = {}", f.0, edge, verts[edge].0), color);
            }

            // edges
            let edge_start = rot3(edge as u8).decrement().id() as usize;
            let edge_end = rot3(edge as u8).increment().id() as usize;
            if !positions[edge_start].is_visible() || !positions[edge_end].is_visible() {
                continue;
            }

            //let is_virtual = positions[ edge_start ].is_virtual() || positions[ edge_end ].is_virtual();
            //bool is_constraint = !!aTri[ aFace ].getConstraint( edge );

            //let n = tri[f].neighbor(rot3(edge));
            //let  col = isConstraint ? aColor.edgeConstrained_ : isVirtual ? aColor.edgeInfinite_ : aColor.edge_;
            let a = positions[edge_start].position();
            let b = positions[edge_end].position();
            let color = mapping.coloring.face.clone();
            self.add_line(&(a.x, a.y), &(b.x, b.y), color);
            //glm::vec2 ab = ( a + b ) * 0.5f;
            //addText( ab, stdext::format( "n", aFace, ".", edge, "=", n ), col );
        }

        // text
    /*std::string msg = aMessage.value_or( stdext::format( "F:", aFace ) );
    if( !msg.empty() ) {
    glm::vec2 center;
    float cnt = 0;
    for( const auto& p: positions ) {
        if( p.isVisible() ) {
        center += p.pos;
        cnt += 1;
        }
    }
    if( cnt > 0 ) {
        center /= cnt;
        addText( center, msg, aTri.isFinite( aFace ) ? aColor.faceText_ : aColor.faceInfiniteText );
    }
    }*/    }

    fn add_circum_circle(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, msg: Option<&str>) {}
}
