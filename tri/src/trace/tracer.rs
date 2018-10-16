use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use indexing::PositionIndex;
use inexactgeometry::{InexactPosition64, InexactPredicates64};
use std::collections::HashMap;
use svg::node::{element, Text};
use svg::{Document, Node};
use types::{rot3, FaceIndex, Rot3, VertexIndex};

/// Color settings for the Render
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
    fn new() -> Coloring {
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

/// Point render type based on virtual positions
#[derive(Debug)]
pub enum RenderPosition {
    Invisible,
    Virtual(InexactPosition64),
    Real(InexactPosition64),
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

    pub fn position(&self) -> &InexactPosition64 {
        match *self {
            RenderPosition::Virtual(ref p) => p,
            RenderPosition::Real(ref p) => p,
            _ => panic!("No position for {:?}", self),
        }
    }
}

/// Map virtual and ideal points into visualizable objects.
pub struct RenderMapping {
    pub virtual_positions: Vec<InexactPosition64>,
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

    pub fn add_virtual_position<VP: Into<InexactPosition64>>(&mut self, p: VP) {
        self.virtual_positions.push(p.into());
    }

    pub fn set_virtual_position<VP: Into<Vec<InexactPosition64>>>(&mut self, p: VP) {
        self.virtual_positions = p.into();
    }

    pub fn map_vertex<P, V, F>(&self, tri: &Graph<P, V, F>, v: VertexIndex, vcw: VertexIndex, vccw: VertexIndex) -> RenderPosition
    where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
        if !v.is_valid() {
            return RenderPosition::Invisible;
        }

        if tri.is_finite_vertex(v) {
            let p = InexactPosition64::from(&tri[PositionIndex::Vertex(v)]);
            return RenderPosition::Real(p);
        }

        if self.virtual_positions.is_empty() {
            return RenderPosition::Invisible;
        }

        let predicates = InexactPredicates64::new();

        // find virtual point best fitting the convex hull (2d)
        if vcw.is_valid() && tri.is_finite_vertex(vcw) && vccw.is_valid() && tri.is_finite_vertex(vccw) {
            let p = InexactPosition64::from(&tri[PositionIndex::Vertex(v)]);
            let pcw = InexactPosition64::from(&tri[PositionIndex::Vertex(vcw)]);
            let pccw = InexactPosition64::from(&tri[PositionIndex::Vertex(vccw)]);
            let mut best_value = 0.;
            let mut best = None;

            for &virt_pos in self.virtual_positions.iter() {
                let value = predicates.orientation_triangle(&pcw, &p, &pccw);
                if value > best_value {
                    best_value = value;
                    best = Some(virt_pos);
                }
            }

            return best.map(|p| RenderPosition::Virtual(p)).unwrap_or(RenderPosition::Invisible);
        }

        // find virtual point best fitting the edge (1d)
        for &candidate in [vcw, vccw].iter() {
            if candidate.is_valid() && tri.is_finite_vertex(candidate) {
                let p = InexactPosition64::from(&tri[PositionIndex::Vertex(candidate)]);
                let mut best_value = 0.;
                let mut best = None;

                for &virt_pos in self.virtual_positions.iter() {
                    let value = predicates.distance_point_point(&virt_pos, &p);
                    if value > best_value {
                        best_value = value;
                        best = Some(virt_pos);
                    }
                }
                return best.map(|p| RenderPosition::Virtual(p)).unwrap_or(RenderPosition::Invisible);
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

/// Render triangulation for tracing
pub struct Tracer {
    coloring: Coloring,
    document: Document,
    layers: Vec<element::Group>,
    scale: (f64, f64, f64, f64),
    text_map: HashMap<(i32, i32), usize>,
}

impl Tracer {
    pub fn new() -> Tracer {
        Tracer {
            coloring: Default::default(),
            document: Document::new(),
            layers: Default::default(),
            scale: (1., 1., 0., 0.),
            text_map: Default::default(),
        }
    }

    pub fn push_layer(&mut self) {
        self.layers.push(element::Group::new());
    }

    pub fn pop_layer(&mut self) {
        let v = self.layers.pop().unwrap();
        self.add_node(v);
    }

    pub fn add_vertex<P, V, F>(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, v: VertexIndex, msg: Option<&str>)
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
            let color = self.coloring.vertex.clone();
            self.add_point(&p, color);
            let color = self.coloring.vertex_text.clone();
            self.add_text(&p, msg, color);
        } else {
            for p in mapping.virtual_positions.iter() {
                let color = self.coloring.infinite_vertex.clone();
                self.add_point(p, color);
                let color = self.coloring.infinite_vertex_text.clone();
                self.add_text(p, msg.clone(), color);
            }
        }
    }

    pub fn add_edge<P, V, F>(
        &mut self,
        tri: &Graph<P, V, F>,
        mapping: &RenderMapping,
        start: VertexIndex,
        end: VertexIndex,
        msg: Option<&str>,
    ) where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
        if tri.is_infinite_vertex(start) || tri.is_infinite_vertex(end) {
            return;
        }

        let msg = msg.map(|m| format!("E, {}", m));

        let a = InexactPosition64::from(&tri[PositionIndex::Vertex(start)]);
        let b = InexactPosition64::from(&tri[PositionIndex::Vertex(end)]);
        let node = element::Line::new()
            .set("x1", a.x)
            .set("y1", a.y)
            .set("x2", b.x)
            .set("y2", b.y)
            .set("fill", self.coloring.edge.clone());
        self.add_node(node);
        let x = (a.x + b.x) * 0.5;
        let y = (a.y + b.y) * 0.5;
        let color = self.coloring.edge_text.clone();
        if let Some(msg) = msg {
            self.add_text(&InexactPosition64 { x, y }, msg, color);
        }
    }

    pub fn add_face_edge<P, V, F>(
        &mut self,
        tri: &Graph<P, V, F>,
        mapping: &RenderMapping,
        f: FaceIndex,
        i: Rot3,
        msg: Option<&str>,
    ) where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
        //self.add_edge( tri, mapping, tri[aTri.getStartVertex( aEdge )], aTri.getEndVertex( aEdge ), aMessage, aColor );
    }

    pub fn add_face<P, V, F>(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, msg: Option<&str>)
    where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
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
                    self.coloring.face_text.clone()
                } else {
                    self.coloring.infinite_face_text.clone()
                };
                self.add_text(
                    positions[edge].position(),
                    format!("{}.{} = {}", f.0, edge, verts[edge].0),
                    color,
                );
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
            let color = self.coloring.face.clone();
            self.add_line(a, b, color);
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

    pub fn add_circum_circle<P, V, F>(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping, f: FaceIndex, msg: Option<&str>)
    where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
    }

    pub fn add_triangle<P, V, F>(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping)
    where
        P: Predicates,
        V: Vertex<Position = P::Position>,
        F: Face,
    {
        use std::f64;
        let (mut minx, mut miny) = (f64::MAX, f64::MAX);
        let (mut maxx, mut maxy) = (f64::MIN, f64::MIN);

        for v in tri.vertex_index_iter() {
            let p = InexactPosition64::from(&tri[PositionIndex::Vertex(v)]);
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

        self.set_scale(minx, miny, maxx, maxy);

        for f in tri.face_index_iter() {
            self.add_face(tri, mapping, f, None);
            //traceCircumCircle( aTri, f, stdext::nullopt, aColor );
        }

        for v in tri.vertex_index_iter() {
            self.add_vertex(tri, mapping, v, None);
        }
    }

    fn add_node<N: Node>(&mut self, node: N) {
        if let Some(p) = self.layers.last_mut() {
            p.append(node);
        } else {
            self.document.append(node);
        }
    }

    fn scale(&self, p: &InexactPosition64) -> InexactPosition64 {
        InexactPosition64 {
            x: p.x * self.scale.0 + self.scale.2,
            y: p.y * self.scale.1 + self.scale.3,
        }
    }

    fn set_scale(&mut self, minx: f64, miny: f64, maxx: f64, maxy: f64) {
        let w = maxx - minx;
        let h = maxy - miny;
        let w = if w == 0. { 1. } else { w };
        let h = if h == 0. { 1. } else { h };

        self.scale.0 = 2. / w;
        self.scale.1 = 2. / h;
        self.scale.2 = -(minx + maxx) / w;
        self.scale.3 = -(miny + maxy) / h;
        self.document.assign("width", "640");
        self.document.assign("height", "auto");
        self.document.assign("viewbox", "-1 -1 2 2");
    }

    fn add_point(&mut self, p: &InexactPosition64, color: String) {
        let p = self.scale(p);
        let node = element::Line::new()
            .set("x1", p.x)
            .set("y1", p.y)
            .set("x2", p.x)
            .set("y2", p.y)
            .set("vector-effect", "non-scaling-stroke")
            .set("stroke-linecap", "round")
            .set("stroke", color)
            .set("stroke-width", "4");
        self.add_node(node);
    }

    fn add_line(&mut self, a: &InexactPosition64, b: &InexactPosition64, color: String) {
        let a = self.scale(a);
        let b = self.scale(b);
        let node = element::Line::new()
            .set("x1", a.x)
            .set("y1", a.y)
            .set("x2", b.x)
            .set("y2", b.y)
            .set("vector-effect", "non-scaling-stroke")
            .set("stroke-linecap", "round")
            .set("stroke", color)
            .set("stroke-width", "2");
        self.add_node(node);
    }

    fn add_text(&mut self, p: &InexactPosition64, msg: String, color: String) {
        let p = self.scale(p);

        let offset = {
            let key = ((p.x * 65536.) as i32, (p.y * 65536.) as i32);
            let count = self.text_map.entry(key).or_insert(0);
            *count += 1;
            *count as f64 * 0.05
        };

        let mut node = element::Text::new()
            .set("x", p.x)
            .set("y", p.y + offset)
            .set("font-size", "0.05")
            .set("fill", color);
        node.append(Text::new(msg));
        self.add_node(node);
    }
}

impl ToString for Tracer {
    fn to_string(&self) -> String {
        self.document.to_string()
    }
}
