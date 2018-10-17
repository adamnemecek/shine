use geometry::Predicates;
use graph::{Face, Graph, Vertex};
use indexing::PositionIndex;
use inexactgeometry::{InexactPosition64, InexactPredicates64};
use types::{FaceIndex, Rot3, VertexIndex};

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

/// Vertex trace visualization info
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

/// Trace helper to map vertices into virtual positions
pub struct RenderMapping {
    pub virtual_positions: Vec<InexactPosition64>,
    pub coloring: Coloring,
}

impl RenderMapping {
    pub fn new() -> RenderMapping {
        RenderMapping {
            virtual_positions: Default::default(),
            coloring: Default::default(),
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
    fn add_triangle(&mut self, tri: &Graph<P, V, F>, mapping: &RenderMapping) {
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

        self.set_viewport((minx, miny), (maxx, maxy));

        for f in tri.face_index_iter() {
            self.add_face(tri, mapping, f, None);
            //traceCircumCircle( aTri, f, stdext::nullopt, aColor );
        }

        for v in tri.vertex_index_iter() {
            self.add_vertex(tri, mapping, v, None);
        }
    }
}
impl<P, V, F, T> TraceExt<P, V, F> for T
where
    P: Predicates,
    V: Vertex<Position = P::Position>,
    F: Face,
    T: Trace<P, V, F>,
{}
