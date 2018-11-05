#![allow(dead_code)]

use shine_testutils::webserver::*;
use shine_tri::geometry::position::Posf64;
use shine_tri::geometry::{NearestPointSearch, NearestPointSearchBuilder, Position, Predicates, Predicatesf64};
use shine_tri::indexing::{IndexGet, PositionQuery, VertexIndexQuery};
use shine_tri::types::{rot3, FaceIndex, Rot3, VertexIndex};
use shine_tri::{Face, Graph, Vertex};

/// Color settings for the Trace
pub struct Coloring {
    pub vertex: String,
    pub vertex_text: String,
    pub infinite_vertex: String,
    pub infinite_vertex_text: String,
    pub edge: String,
    pub edge_text: String,
    pub face: String,
    pub face_text: String,
    pub infinite_face: String,
    pub infinite_face_text: String,
}

impl Coloring {
    pub fn new() -> Coloring {
        Coloring {
            vertex: "green".into(),
            vertex_text: "green".into(),
            infinite_vertex: "red".into(),
            infinite_vertex_text: "green".into(),
            edge: "blue".into(),
            edge_text: "blue".into(),
            face: "yellow".into(),
            face_text: "yellow".into(),
            infinite_face: "grey".into(),
            infinite_face_text: "grey".into(),
        }
    }
}

impl Default for Coloring {
    fn default() -> Coloring {
        Coloring::new()
    }
}

/// Vertex trace visualization info
#[derive(Debug)]
pub enum RenderPosition {
    Invisible,
    Virtual(Posf64),
    Real(Posf64),
}

impl RenderPosition {
    pub fn is_visible(&self) -> bool {
        match *self {
            RenderPosition::Virtual(_) => true,
            RenderPosition::Real(_) => true,
            _ => false,
        }
    }

    pub fn is_virtual(&self) -> bool {
        match *self {
            RenderPosition::Virtual(_) => true,
            _ => false,
        }
    }

    pub fn position(&self) -> &Posf64 {
        match *self {
            RenderPosition::Virtual(ref p) => p,
            RenderPosition::Real(ref p) => p,
            _ => panic!("No position for {:?}", self),
        }
    }
}

/// Trace helper to map vertices into virtual positions
pub struct RenderMapping {
    virtual_positions: Vec<Posf64>,
}

impl RenderMapping {
    pub fn new() -> RenderMapping {
        RenderMapping {
            virtual_positions: Default::default(),
        }
    }

    pub fn clear_virtual_position(&mut self) {
        self.virtual_positions.clear();
    }

    pub fn add_virtual_position<VP: Into<Posf64>>(&mut self, p: VP) {
        self.virtual_positions.push(p.into());
    }

    pub fn set_virtual_position<VP: Into<Vec<Posf64>>>(&mut self, p: VP) {
        self.virtual_positions = p.into();
    }

    pub fn map_vertex<P, V, F>(
        &self,
        graph: &Graph<P, V, F>,
        v: VertexIndex,
        vcw: VertexIndex,
        vccw: VertexIndex,
    ) -> RenderPosition
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        if !v.is_valid() {
            return RenderPosition::Invisible;
        }

        if graph.is_finite_vertex(v) {
            let p = Posf64::from(&graph[PositionQuery::Vertex(v)]);
            return RenderPosition::Real(p);
        }

        if self.virtual_positions.is_empty() {
            return RenderPosition::Invisible;
        }

        let predicates = Predicatesf64::new();

        // find virtual point best fitting the convex hull in 2d
        if vcw.is_valid() && graph.is_finite_vertex(vcw) && vccw.is_valid() && graph.is_finite_vertex(vccw) {
            let p = Posf64::from(&graph[PositionQuery::Vertex(v)]);
            let pcw = Posf64::from(&graph[PositionQuery::Vertex(vcw)]);
            let pccw = Posf64::from(&graph[PositionQuery::Vertex(vccw)]);
            let mut best_value = 0.;
            let mut best = None;

            for virt_pos in self.virtual_positions.iter() {
                let value = predicates.orientation_triangle(&pcw, &p, &pccw);
                if value > best_value {
                    best_value = value;
                    best = Some(virt_pos);
                }
            }

            return best
                .map(|p| RenderPosition::Virtual(p.clone()))
                .unwrap_or(RenderPosition::Invisible);
        }

        // find virtual point best fitting the convex hull in 1d
        for &candidate in [vcw, vccw].iter() {
            if candidate.is_valid() && graph.is_finite_vertex(candidate) {
                let p = Posf64::from(&graph[PositionQuery::Vertex(candidate)]);
                let mut search = predicates.nearest_point_search(&p);

                for virt_pos in self.virtual_positions.iter() {
                    search.test(virt_pos, ());
                }
                return search
                    .nearest()
                    .map(|(p, _)| RenderPosition::Virtual(p.clone()))
                    .unwrap_or(RenderPosition::Invisible);
            }
        }

        RenderPosition::Invisible
    }
}

impl Default for RenderMapping {
    fn default() -> RenderMapping {
        RenderMapping::new()
    }
}

pub struct Trace<'a, P, V, F>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    graph: &'a Graph<P, V, F>,
    mapping: &'a RenderMapping,
    coloring: &'a Coloring,
}

impl<'a, P, V, F> Trace<'a, P, V, F>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    pub fn new<'b>(graph: &'b Graph<P, V, F>, mapping: &'b RenderMapping, coloring: &'b Coloring) -> Trace<'b, P, V, F> {
        Trace {
            graph,
            mapping,
            coloring,
        }
    }

    pub fn add_vertex(&self, tr: &mut D2Trace, v: VertexIndex, msg: Option<&str>)
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        if !v.is_valid() {
            return;
        }

        let msg = msg.map(|m| format!("V: {}", m)).unwrap_or_else(|| format!("V: {}", v.id()));

        if self.graph.is_finite_vertex(v) {
            let p = Posf64::from(&self.graph[PositionQuery::Vertex(v)]);
            tr.add_point(&(p.x, p.y), self.coloring.vertex.clone());
            tr.add_text(&(p.x, p.y), msg, self.coloring.vertex_text.clone());
        } else {
            for p in self.mapping.virtual_positions.iter() {
                tr.add_point(&(p.x, p.y), self.coloring.infinite_vertex.clone());
                tr.add_text(&(p.x, p.y), msg.clone(), self.coloring.infinite_vertex_text.clone());
            }
        }
    }

    pub fn add_edge(&self, tr: &mut D2Trace, a: VertexIndex, b: VertexIndex, msg: Option<&str>)
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        if self.graph.is_infinite_vertex(a) || self.graph.is_infinite_vertex(b) {
            return;
        }

        let msg = msg
            .map(|m| format!("E: {}", m))
            .unwrap_or_else(|| format!("E: ({},{})", a.id(), b.id()));

        let pa = Posf64::from(&self.graph[PositionQuery::Vertex(a)]);
        let pb = Posf64::from(&self.graph[PositionQuery::Vertex(b)]);
        tr.add_line(&(pa.x, pa.y), &(pb.x, pb.y), self.coloring.edge.clone());
        let x = (pa.x + pb.x) * 0.5;
        let y = (pa.y + pb.y) * 0.5;
        tr.add_text(&(x, y), msg, self.coloring.edge_text.clone());
    }

    pub fn add_face_edge(&self, tr: &mut D2Trace, f: FaceIndex, i: Rot3, msg: Option<&str>) {
        self.add_edge(
            tr,
            self.graph.index_get(VertexIndexQuery::EdgeStart(f, i)),
            self.graph.index_get(VertexIndexQuery::EdgeEnd(f, i)),
            msg,
        );
    }

    fn add_face(&self, tr: &mut D2Trace, f: FaceIndex, msg: Option<&str>) {
        if !f.is_valid() {
            return;
        }

        let verts = [
            self.graph[f].vertex(rot3(0)),
            self.graph[f].vertex(rot3(1)),
            self.graph[f].vertex(rot3(2)),
        ];
        let positions = [
            self.mapping.map_vertex(self.graph, verts[0], verts[1], verts[2]),
            self.mapping.map_vertex(self.graph, verts[1], verts[2], verts[0]),
            self.mapping.map_vertex(self.graph, verts[2], verts[0], verts[1]),
        ];

        for edge in 0..3 {
            // vertex
            if positions[edge].is_visible() {
                let color = if positions[edge].is_virtual() {
                    self.coloring.face_text.clone()
                } else {
                    self.coloring.infinite_face_text.clone()
                };
                let p = positions[edge].position();
                tr.add_text(&(p.x, p.y), format!("{}.{} = {}", f.id(), edge, verts[edge].id()), color);
            }

            // edges
            let edge_start = rot3(edge as u8).decrement().id() as usize;
            let edge_end = rot3(edge as u8).increment().id() as usize;
            if !positions[edge_start].is_visible() || !positions[edge_end].is_visible() {
                continue;
            }

            //let is_virtual = positions[ edge_start ].is_virtual() || positions[ edge_end ].is_virtual();
            //bool is_constraint = !!aself.graph[ aFace ].getConstraint( edge );

            //let n = self.graph[f].neighbor(rot3(edge));
            //let  col = isConstraint ? aColor.edgeConstrained_ : isVirtual ? aColor.edgeInfinite_ : aColor.edge_;
            let a = positions[edge_start].position();
            let b = positions[edge_end].position();
            let color = self.coloring.face.clone();
            tr.add_line(&(a.x, a.y), &(b.x, b.y), color);
            //glm::vec2 ab = ( a + b ) * 0.5f;
            //addText( ab, stdext::format( "n", aFace, ".", edge, "=", n ), col );
        }

        // text
        let msg = msg.map(|m| format!("F: {}", m)).unwrap_or_else(|| format!("F: {}", f.id()));
        let mut center = Posf64 { x: 0., y: 0. };
        let mut cnt = 0.;
        for p in positions.iter() {
            if p.is_visible() {
                let pos = p.position();
                center.x += pos.x;
                center.y += pos.y;
                cnt += 1.;
            }
        }

        if cnt > 0. {
            let color = if self.graph.is_finite_face(f) {
                &self.coloring.face_text
            } else {
                &self.coloring.infinite_face_text
            };
            tr.add_text(&(center.x / cnt, center.y / cnt), msg, color.clone());
        }
    }
}

impl<'a, P, V, F> IntoD2Image for Trace<'a, P, V, F>
where
    P: 'a + Position,
    V: 'a + Vertex<Position = P>,
    F: 'a + Face,
{
    fn trace(&self, tr: &mut D2Trace) {
        use std::f64;
        let (mut minx, mut miny) = (f64::MAX, f64::MAX);
        let (mut maxx, mut maxy) = (f64::MIN, f64::MIN);

        for v in self.graph.vertex_index_iter() {
            let p = Posf64::from(&self.graph[PositionQuery::Vertex(v)]);
            minx = if p.x < minx { p.x } else { minx };
            maxx = if p.x > maxx { p.x } else { maxx };
            miny = if p.y < minx { p.y } else { minx };
            maxy = if p.y > maxx { p.y } else { maxx };
        }

        let w = maxx - minx;
        let h = maxy - miny;
        minx = minx - w * 0.2;
        miny = miny - h * 0.2;
        maxx = maxx + w * 0.2;
        maxy = maxy + h * 0.2;

        tr.set_scale(minx, miny, maxx, maxy);

        for f in self.graph.face_index_iter() {
            self.add_face(tr, f, None);
            //add_circum_circle( f, None );
        }

        for v in self.graph.vertex_index_iter() {
            self.add_vertex(tr, v, None);
        }
    }
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
