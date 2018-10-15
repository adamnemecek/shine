use geometry::{Position, Predicates};
use graph::{Face, Graph, Vertex};
use indexing::PositionIndex;
use inexactgeometry::{InexactPosition64, InexactPredicates64};
use types::{FaceIndex, Rot3, VertexIndex};

enum RenderPosition {
    Invisible,
    Virtual(InexactPosition64),
    Real(InexactPosition64),
}

pub struct Render<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    tri: &'a Graph<P, V, F>,
    virtual_positions: Vec<InexactPosition64>,
}

impl<'a, P, V, F> Render<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Graph<P, V, F>) -> Render<P, V, F> {
        Render {
            tri,
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

    pub fn add_vertex(&mut self, v: VertexIndex, msg: Option<&str>) {}

    pub fn add_edge(&mut self, start: VertexIndex, end: VertexIndex, msg: Option<&str>) {}

    pub fn add_face_edge(&mut self, f: FaceIndex, i: Rot3, msg: Option<&str>) {}

    pub fn add_face(&mut self, f: FaceIndex, msg: Option<&str>) {}

    pub fn add_circum_circle(&mut self, f: FaceIndex, msg: Option<&str>) {}

    pub fn trace(&mut self) {}

    fn find_render_position(&self, v: VertexIndex, vcw: VertexIndex, vccw: VertexIndex) -> RenderPosition {
        if !v.is_valid() {
            return RenderPosition::Invisible;
        }

        if self.tri.is_finite_vertex(v) {
            let p = InexactPosition64::from(&self.tri[PositionIndex::Vertex(v)]);
            return RenderPosition::Real(p);
        }

        if self.virtual_positions.is_empty() {
            return RenderPosition::Invisible;
        }

        let predicates = InexactPredicates64::new();

        // find virtual point best fitting the convex hull (2d)
        if vcw.is_valid() && self.tri.is_finite_vertex(vcw) && vccw.is_valid() && self.tri.is_finite_vertex(vccw) {
            let p = InexactPosition64::from(&self.tri[PositionIndex::Vertex(v)]);
            let pcw = InexactPosition64::from(&self.tri[PositionIndex::Vertex(vcw)]);
            let pccw = InexactPosition64::from(&self.tri[PositionIndex::Vertex(vccw)]);
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
            if candidate.is_valid() && self.tri.is_finite_vertex(candidate) {
                let p = InexactPosition64::from(&self.tri[PositionIndex::Vertex(candidate)]);
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
