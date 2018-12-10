use checker::TracePosition;
use geometry::{InexactPredicates, Posf64};
use geometry::{NearestPointSearch, NearestPointSearchBuilder, Position, Predicates};
use graph::{Constraint, Face, TraceContext, Triangulation, Vertex};
use query::{TopologyQuery, VertexClue};
use types::{rot3, FaceIndex, Rot3, VertexIndex};

pub trait TraceRender {
    fn begin(&mut self);
    fn end(&mut self);

    fn set_viewport(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64);

    fn push_layer(&mut self);
    fn pop_layer(&mut self);

    fn add_point(&mut self, p: &(f64, f64), color: String);
    fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64), color: String);
    fn add_text(&mut self, p: &(f64, f64), msg: String, color: String, size: f32);
}

pub trait Trace {
    fn trace_map_vertex(&self, v: VertexIndex, vcw: VertexIndex, vccw: VertexIndex) -> TracePosition;

    fn trace_vertex(&self, v: VertexIndex, msg: Option<&str>);
    fn trace_edge(&self, a: VertexIndex, b: VertexIndex, msg: Option<&str>);
    fn trace_face(&self, f: FaceIndex, msg: Option<&str>);
    fn trace_face_edge(&self, f: FaceIndex, i: Rot3, msg: Option<&str>);

    fn trace(&self);

    fn trace_begin(&self);
    fn trace_end(&self);
}

impl<P, V, F, C> Trace for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
    C: TraceContext,
{
    fn trace_map_vertex(&self, v: VertexIndex, vcw: VertexIndex, vccw: VertexIndex) -> TracePosition {
        if !v.is_valid() {
            return TracePosition::Invisible;
        }

        let mapping = self.context.trace_mapping();
        let approximate_predicates = InexactPredicates::<Posf64>::new();

        if self.is_finite_vertex(v) {
            let p = Posf64::from(self.p(v));
            return TracePosition::Real(p);
        }

        if mapping.virtual_positions.is_empty() {
            return TracePosition::Invisible;
        }

        // find virtual point best fitting the convex hull in 2d
        if vcw.is_valid() && self.is_finite_vertex(vcw) && vccw.is_valid() && self.is_finite_vertex(vccw) {
            let pcw = Posf64::from(self.p(vcw));
            let pccw = Posf64::from(self.p(vccw));

            let mut best_value = 0.;
            let mut best = None;

            for virt_pos in mapping.virtual_positions.iter() {
                let value = approximate_predicates.orientation_triangle(&pccw, &virt_pos, &pcw);
                if value.0 > best_value {
                    best_value = value.0;
                    best = Some(virt_pos);
                }
            }

            return best
                .map(|p| TracePosition::Virtual(p.clone()))
                .unwrap_or(TracePosition::Invisible);
        }

        // find virtual point best fitting the convex hull in 1d
        for &candidate in [vcw, vccw].iter() {
            if candidate.is_valid() && self.is_finite_vertex(candidate) {
                let p = Posf64::from(self.p(candidate));
                let mut search = approximate_predicates.nearest_point_search(&p);

                for virt_pos in mapping.virtual_positions.iter() {
                    search.test(virt_pos, ());
                }
                return search
                    .nearest()
                    .map(|(p, _)| TracePosition::Virtual(p.clone()))
                    .unwrap_or(TracePosition::Invisible);
            }
        }

        TracePosition::Invisible
    }

    fn trace_vertex(&self, v: VertexIndex, msg: Option<&str>)
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        if !v.is_valid() {
            return;
        }

        let mut tr = self.context.trace_render();
        let mapping = self.context.trace_mapping();
        let coloring = self.context.trace_coloring();

        let msg = msg.map(|m| format!("V: {}", m)).unwrap_or_else(|| format!("V: {}", v.id()));

        if self.is_finite_vertex(v) {
            let p = Posf64::from(self.p(v));
            tr.add_point(&(p.x, p.y), coloring.vertex.clone());
            tr.add_text(&(p.x, p.y), msg, coloring.vertex_text.0.clone(), coloring.vertex_text.1);
        } else {
            for p in mapping.virtual_positions.iter() {
                tr.add_point(&(p.x, p.y), coloring.infinite_vertex.clone());
                tr.add_text(
                    &(p.x, p.y),
                    msg.clone(),
                    coloring.infinite_vertex_text.0.clone(),
                    coloring.infinite_vertex_text.1,
                );
            }
        }
    }

    fn trace_edge(&self, a: VertexIndex, b: VertexIndex, msg: Option<&str>)
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        if self.is_infinite_vertex(a) || self.is_infinite_vertex(b) {
            return;
        }

        let mut tr = self.context.trace_render();
        let coloring = self.context.trace_coloring();

        let msg = msg
            .map(|m| format!("E: {}", m))
            .unwrap_or_else(|| format!("E: ({},{})", a.id(), b.id()));

        let pa = Posf64::from(self.p(a));
        let pb = Posf64::from(self.p(b));
        tr.add_line(&(pa.x, pa.y), &(pb.x, pb.y), coloring.edge.clone());
        let x = (pa.x + pb.x) * 0.5;
        let y = (pa.y + pb.y) * 0.5;
        tr.add_text(&(x, y), msg, coloring.edge_text.0.clone(), coloring.edge_text.1);
    }

    fn trace_face_edge(&self, f: FaceIndex, i: Rot3, msg: Option<&str>) {
        self.trace_edge(
            self.vi(VertexClue::edge_start(f, i)),
            self.vi(VertexClue::edge_end(f, i)),
            msg,
        );
    }

    fn trace_face(&self, f: FaceIndex, msg: Option<&str>) {
        if !f.is_valid() {
            return;
        }

        let mut tr = self.context.trace_render();
        let coloring = self.context.trace_coloring();

        let verts = [self[f].vertex(rot3(0)), self[f].vertex(rot3(1)), self[f].vertex(rot3(2))];
        let positions = [
            self.trace_map_vertex(verts[0], verts[1], verts[2]),
            self.trace_map_vertex(verts[1], verts[2], verts[0]),
            self.trace_map_vertex(verts[2], verts[0], verts[1]),
        ];

        for edge in 0..3 {
            // vertex
            if positions[edge].is_visible() {
                let text_style = if positions[edge].is_virtual() {
                    &coloring.vertex_text
                } else {
                    &coloring.infinite_vertex_text
                };
                let p = positions[edge].position();
                tr.add_text(
                    &(p.x, p.y),
                    format!("{}.{} = {}", f.id(), edge, verts[edge].id()),
                    text_style.0.clone(),
                    text_style.1,
                );
            }

            // edge
            // edges are render twice for the two adjacent triangles
            let edge_start = rot3(edge as u8).decrement().id() as usize;
            let edge_end = rot3(edge as u8).increment().id() as usize;
            if !positions[edge_start].is_visible() || !positions[edge_end].is_visible() {
                continue;
            }

            let is_virtual = positions[edge_start].is_virtual() || positions[edge_end].is_virtual();
            let constraint = self[f].constraint(rot3(edge as u8));
            let (color, text_style) = match (is_virtual, constraint.is_constraint()) {
                (true, true) => (coloring.error.clone(), &coloring.error_text),
                (true, false) => (coloring.infinite_edge.clone(), &coloring.infinite_edge_text),
                (false, true) => (coloring.constraint_edge.clone(), &coloring.constraint_edge_text),
                (false, false) => (coloring.edge.clone(), &coloring.edge_text),
            };

            let n = self[f].neighbor(rot3(edge as u8));
            let a = positions[edge_start].position();
            let b = positions[edge_end].position();
            tr.add_line(&(a.x, a.y), &(b.x, b.y), color);

            let center = ((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
            tr.add_text(
                &center,
                format!("{}.{}={}\n   c:{:?}", f.id(), edge, n.id(), constraint),
                text_style.0.clone(),
                text_style.1,
            );
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
            let text_style = if self.is_finite_face(f) {
                &coloring.face_text
            } else {
                &coloring.infinite_face_text
            };
            tr.add_text(&(center.x / cnt, center.y / cnt), msg, text_style.0.clone(), text_style.1);
        }
    }

    fn trace(&self) {
        for v in self.vertex_index_iter() {
            self.trace_vertex(v, None);
        }
        for f in self.face_index_iter() {
            self.trace_face(f, None);
            //trace_circum_circle( f, None );
        }
    }

    fn trace_begin(&self) {
        let mut tr = self.context.trace_render();
        tr.begin();

        use std::f64;
        let (mut minx, mut miny) = (f64::MAX, f64::MAX);
        let (mut maxx, mut maxy) = (f64::MIN, f64::MIN);

        for v in self.vertex_index_iter() {
            let p = Posf64::from(self.p(v));
            minx = if p.x < minx { p.x } else { minx };
            maxx = if p.x > maxx { p.x } else { maxx };
            miny = if p.y < minx { p.y } else { minx };
            maxy = if p.y > maxx { p.y } else { maxx };
        }

        {
            let mapping = self.context.trace_mapping();
            for p in mapping.virtual_positions.iter() {
                minx = if p.x < minx { p.x } else { minx };
                maxx = if p.x > maxx { p.x } else { maxx };
                miny = if p.y < minx { p.y } else { minx };
                maxy = if p.y > maxx { p.y } else { maxx };
            }
        }

        let w = maxx - minx;
        let h = maxy - miny;
        minx = minx - w * 0.02;
        miny = miny - h * 0.02;
        maxx = maxx + w * 0.02;
        maxy = maxy + h * 0.02;

        tr.set_viewport(minx, miny, maxx, maxy);
    }

    fn trace_end(&self) {
        let mut tr = self.context.trace_render();
        tr.end();
    }
}
