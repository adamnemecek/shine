use check::{Coloring, EdgeColoring, TraceMapping, TracePosition, VertexColoring};
use geometry::{InexactPredicates, Posf64};
use geometry::{NearestPointSearch, NearestPointSearchBuilder, Position, Predicates};
use graph::{Constraint, Face, TraceContext, Triangulation, Vertex};
use query::{TopologyQuery, VertexClue};
use std::ops;
use types::{rot3, FaceEdge, FaceIndex, VertexIndex};

pub trait TraceRender {
    fn begin(&mut self);
    fn end(&mut self);

    fn set_viewport(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64);

    fn push_layer(&mut self, name: Option<String>);
    fn pop_layer(&mut self);

    fn add_point(&mut self, p: &(f64, f64), color: String);
    fn add_line(&mut self, a: &(f64, f64), b: &(f64, f64), color: String);
    fn add_text(&mut self, p: &(f64, f64), msg: String, color: String, size: f32);
}

pub trait TraceControl {
    fn coloring(&self) -> &Coloring;
    fn coloring_mut(&mut self) -> &mut Coloring;
    fn mapping(&self) -> &TraceMapping;
    fn mapping_mut(&mut self) -> &mut TraceMapping;

    fn pause(&mut self);
}

pub struct TraceDocument<'a, T>
where
    T: 'a + Trace,
{
    trace: &'a T,
}

impl<'a, T> ops::Deref for TraceDocument<'a, T>
where
    T: 'a + Trace,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.trace
    }
}

impl<'a, T> Drop for TraceDocument<'a, T>
where
    T: 'a + Trace,
{
    fn drop(&mut self) {
        self.trace.trace_end();
    }
}

pub struct TraceLayer<'a, T>
where
    T: 'a + Trace,
{
    trace: &'a T,
}

impl<'a, T> ops::Deref for TraceLayer<'a, T>
where
    T: 'a + Trace,
{
    type Target = T;

    fn deref(&self) -> &T {
        self.trace
    }
}

impl<'a, T> Drop for TraceLayer<'a, T>
where
    T: 'a + Trace,
{
    fn drop(&mut self) {
        self.trace.trace_pop_layer();
    }
}

pub trait Trace {
    fn trace_map_vertex(&self, v: VertexIndex, vcw: VertexIndex, vccw: VertexIndex) -> TracePosition;

    fn trace_vertex(&self, v: VertexIndex, msg: Option<&str>, color: Option<&VertexColoring>);
    fn trace_edge(&self, a: VertexIndex, b: VertexIndex, msg: Option<&str>, color: Option<&EdgeColoring>);
    fn trace_face_edge<E: Into<FaceEdge>>(&self, edge: E, msg: Option<&str>, color: Option<&EdgeColoring>);
    fn trace_face(&self, f: FaceIndex, msg: Option<&str>, color: Option<&Coloring>);
    fn trace_graph(&self, color: Option<&Coloring>);

    fn trace_begin(&self);
    fn trace_end(&self);
    fn trace_push_layer<S: Into<String>>(&self, name: Option<S>);
    fn trace_pop_layer(&self);
    fn trace_pause(&self);

    fn trace_document(&self) -> TraceDocument<Self>
    where
        Self: Sized,
    {
        self.trace_begin();
        TraceDocument { trace: self }
    }

    fn trace_layer<S: Into<String>>(&self, name: Option<S>) -> TraceLayer<Self>
    where
        Self: Sized,
    {
        self.trace_push_layer(name);
        TraceLayer { trace: self }
    }

    fn trace_face_edges<'a, I>(&self, iter: I, color: Option<&EdgeColoring>)
    where
        I: 'a + Iterator<Item = &'a FaceEdge>,
    {
        for edge in iter {
            self.trace_face_edge(edge.clone(), None, color);
        }
    }

    fn trace(&self) {
        self.trace_begin();
        self.trace_graph(None);
        self.trace_end();
    }
}

impl<P, V, F, C> Trace for Triangulation<P, V, F, C>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    default fn trace_map_vertex(&self, v: VertexIndex, _vcw: VertexIndex, _vccw: VertexIndex) -> TracePosition {
        if self.is_finite_vertex(v) {
            let p = Posf64::from(self.p(v));
            TracePosition::Real(p)
        } else {
            TracePosition::Invisible
        }
    }

    default fn trace_vertex(&self, _v: VertexIndex, _msg: Option<&str>, _color: Option<&VertexColoring>) {}
    default fn trace_edge(&self, _a: VertexIndex, _b: VertexIndex, _msg: Option<&str>, _color: Option<&EdgeColoring>) {}
    default fn trace_face_edge<E: Into<FaceEdge>>(&self, _edge: E, _msg: Option<&str>, _color: Option<&EdgeColoring>) {}
    default fn trace_face(&self, _f: FaceIndex, _msg: Option<&str>, _color: Option<&Coloring>) {}
    default fn trace_graph(&self, _color: Option<&Coloring>) {}

    default fn trace_begin(&self) {}
    default fn trace_end(&self) {}
    default fn trace_push_layer<S: Into<String>>(&self, _name: Option<S>) {}
    default fn trace_pop_layer(&self) {}
    default fn trace_pause(&self) {}
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

        let control = self.context.trace_control();
        let control = control.borrow();
        let mapping = control.mapping();

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

    fn trace_vertex(&self, v: VertexIndex, msg: Option<&str>, color: Option<&VertexColoring>)
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        if !v.is_valid() {
            return;
        }

        let render = self.context.trace_render();
        let mut render = render.borrow_mut();

        let control = self.context.trace_control();
        let control = control.borrow();
        let mapping = control.mapping();
        let coloring = color.unwrap_or(&control.coloring().vertex);

        let msg = msg.map(|m| format!("V: {}", m)).unwrap_or_else(|| format!("V: {}", v.id()));

        if self.is_finite_vertex(v) {
            let p = Posf64::from(self.p(v));
            render.add_point(&(p.x, p.y), coloring.color.clone());
            render.add_text(&(p.x, p.y), msg, coloring.text.0.clone(), coloring.text.1);
        } else {
            for p in mapping.virtual_positions.iter() {
                render.add_point(&(p.x, p.y), coloring.infinite.clone());
                render.add_text(
                    &(p.x, p.y),
                    msg.clone(),
                    coloring.infinite_text.0.clone(),
                    coloring.infinite_text.1,
                );
            }
        }
    }

    fn trace_edge(&self, a: VertexIndex, b: VertexIndex, msg: Option<&str>, color: Option<&EdgeColoring>)
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        if self.is_infinite_vertex(a) || self.is_infinite_vertex(b) {
            return;
        }

        let render = self.context.trace_render();
        let mut render = render.borrow_mut();

        let control = self.context.trace_control();
        let control = control.borrow();
        let coloring = color.unwrap_or(&control.coloring().edge);

        let msg = msg
            .map(|m| format!("E: {}", m))
            .unwrap_or_else(|| format!("E: ({}-{})", a.id(), b.id()));

        let pa = Posf64::from(self.p(a));
        let pb = Posf64::from(self.p(b));
        render.add_line(&(pa.x, pa.y), &(pb.x, pb.y), coloring.color.clone());
        let x = (pa.x + pb.x) * 0.5;
        let y = (pa.y + pb.y) * 0.5;
        render.add_text(&(x, y), msg, coloring.text.0.clone(), coloring.text.1);
    }

    fn trace_face_edge<E: Into<FaceEdge>>(&self, edge: E, msg: Option<&str>, color: Option<&EdgeColoring>) {
        let edge: FaceEdge = edge.into();
        let msg = msg
            .map(|m| m.to_string())
            .unwrap_or_else(|| format!("({}.{})", edge.face.id(), edge.edge.id()));
        self.trace_edge(
            self.vi(VertexClue::start_of(edge)),
            self.vi(VertexClue::end_of(edge)),
            Some(&msg),
            color,
        );
    }

    fn trace_face(&self, f: FaceIndex, msg: Option<&str>, color: Option<&Coloring>) {
        if !f.is_valid() {
            return;
        }

        let verts = [self[f].vertex(rot3(0)), self[f].vertex(rot3(1)), self[f].vertex(rot3(2))];
        let positions = [
            self.trace_map_vertex(verts[0], verts[1], verts[2]),
            self.trace_map_vertex(verts[1], verts[2], verts[0]),
            self.trace_map_vertex(verts[2], verts[0], verts[1]),
        ];

        let render = self.context.trace_render();
        let mut render = render.borrow_mut();

        let control = self.context.trace_control();
        let control = control.borrow();
        let coloring = color.unwrap_or(&control.coloring());

        for edge in 0..3 {
            // vertex
            if positions[edge].is_visible() {
                let text_style = if positions[edge].is_virtual() {
                    &coloring.vertex.text
                } else {
                    &coloring.vertex.infinite_text
                };
                let p = positions[edge].position();
                render.add_text(
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
                (true, false) => (coloring.edge.infinite.clone(), &coloring.edge.infinite_text),
                (false, true) => (coloring.edge.constraint.clone(), &coloring.edge.constraint_text),
                (false, false) => (coloring.edge.color.clone(), &coloring.edge.text),
            };

            let n = self[f].neighbor(rot3(edge as u8));
            let a = positions[edge_start].position();
            let b = positions[edge_end].position();
            render.add_line(&(a.x, a.y), &(b.x, b.y), color);

            let center = ((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
            render.add_text(
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
                &coloring.face.text
            } else {
                &coloring.face.infinite_text
            };
            render.add_text(&(center.x / cnt, center.y / cnt), msg, text_style.0.clone(), text_style.1);
        }
    }

    fn trace_graph(&self, color: Option<&Coloring>) {
        for v in self.vertex_index_iter() {
            self.trace_vertex(v, None, color.map(|v| &v.vertex));
        }
        for f in self.face_index_iter() {
            self.trace_face(f, None, color);
            //trace_circum_circle( f, None );
        }
    }

    fn trace_begin(&self) {
        let render = self.context.trace_render();
        let mut render = render.borrow_mut();

        let control = self.context.trace_control();
        let mut control = control.borrow_mut();
        let mapping = control.mapping_mut();

        render.begin();

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
            //todo: add some default virtual positions if virtual_positions is empty

            for p in mapping.virtual_positions.iter() {
                minx = if p.x < minx { p.x } else { minx };
                maxx = if p.x > maxx { p.x } else { maxx };
                miny = if p.y < minx { p.y } else { minx };
                maxy = if p.y > maxx { p.y } else { maxx };
            }
        }

        let w = maxx - minx;
        let h = maxy - miny;
        minx -= w * 0.02;
        miny -= h * 0.02;
        maxx += w * 0.02;
        maxy += h * 0.02;

        render.set_viewport(minx, miny, maxx, maxy);
    }

    fn trace_end(&self) {
        let render = self.context.trace_render();
        let mut render = render.borrow_mut();
        render.end();
    }

    fn trace_push_layer<S: Into<String>>(&self, name: Option<S>) {
        let render = self.context.trace_render();
        let mut render = render.borrow_mut();
        render.push_layer(name.map(|n| n.into()));
    }

    fn trace_pop_layer(&self) {
        let render = self.context.trace_render();
        let mut render = render.borrow_mut();
        render.pop_layer();
    }

    fn trace_pause(&self) {
        let control = self.context.trace_control();
        let mut control = control.borrow_mut();
        control.pause();
    }
}
