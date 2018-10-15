use geometry::{CollinearTest, Orientation, Position, Predicates};
use graph::{Face, Graph, Vertex};
use indexing::PositionIndex;
use types::{FaceIndex, Rot3, VertexIndex};

enum RenderPosition {
    Invisible,
    Virtual(f64, f64),
    Real(f64, f64),
}

pub struct Trace<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    tri: &'a Graph<P, V, F>,
    virtual_positions: Vec<(f64, f64)>,
}

impl<'a, P, V, F> Trace<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Graph<P, V, F>) -> Trace<P, V, F> {
        Trace {
            tri,
            virtual_positions: Default::default(),
        }
    }

    pub fn clear_virtual_position(&mut self, p: (f64, f64)) {
        self.virtual_positions.clear();
    }

    pub fn add_virtual_position(&mut self, p: (f64, f64)) {
        self.virtual_positions.push(p);
    }

    pub fn set_virtual_position<VP: Into<Vec<(f64, f64)>>>(&mut self, p: VP) {
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
            let p = self.tri[PositionIndex::Vertex(v)];
            return RenderPosition::Real(p.x().into(), p.y().into());
        }

        if self.virtual_positions.is_empty() {
            return RenderPosition::Invisible;
        }

        // find edge
        /*if vcw.is_valid() && self.tri.is_finite_vertex(vcw) && vccw.is_valid() && self.tri.is_finite_vertex(vccw) {
            let pcw = self.tri[PositionIndex::Vertex(vcw)];
            let pccw = self.tri[PositionIndex::Vertex(vccw)];
            let mut best_value = 0;
            let best = None;

            for virt_pos in self.virtual_positions.iter() {
                let value = tri_area(a, b, p);
                if value > 0 {
                    best_value = value;
                    best = Some(virt_pos);
                }
            }

            return virt_pos.unwrap_or(RenderPosition::Invisible);
        }*/

        /* for condidate in [ vcw, vccw ] {
                if( vertexCandidate[ i ].isValid() && aTri.isFinite( vertexCandidate[ i ] ) ) {
                glm::vec2 a = posToVec2f( aTri.getPosition( vertexCandidate[ i ] ) );
                Tri2GlmPredicates::Real max = -1;
                glm::vec2 vpos;

                for( auto p: mVirtualPositions ) {
                    Tri2GlmPredicates::Real o = glm::length( a - p );
                    if( max < 0 || o < max ) {
                    max = o;
                    vpos = p;
                    }
                }

                return max > 0 ? RenderPosition::makeVirtual( vpos ) : RenderPosition::makeInvisible();
                }
            }*/

        RenderPosition::Invisible
    }
}
