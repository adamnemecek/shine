use geometry::Predicates;
use graph::{Face, Vertex};
use indexing::PositionQuery;
use triangulation::Triangulation;
use types::{FaceIndex, Rot3, VertexIndex};

pub enum Side {
    /// Coincident to the edge or indicates the iteration start, when no information
    Concident,
    CounterClokwise,
    Clockwise,
}

pub struct IntersectingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    tri: &'a Triangulation<PR, V, F>,

    v0: VertexIndex,
    v1: VertexIndex,

    has_next: bool,
    current_face: FaceIndex,
    current_index: Rot3,
    current_side: Side,
}

impl<'a, PR, V, F> IntersectingIterator<'a, PR, V, F>
where
    PR: 'a + Predicates,
    V: 'a + Vertex<Position = PR::Position>,
    F: 'a + Face,
{
    pub fn new(&tri: Triangulation<PR, V, F>, v0: VertexIndex, v1: VertexIndex) -> IntersectingIterator<PR, V, F> {
        /*
                const Position& p0 = mTri.getPosition( mV0 );
          const Position& p1 = mTri.getPosition( mV1 );
         
          Tri2EdgeCirculator<TTri> circulator( mTri, mV0 );
          Real rotation = 0;
          for(;; ) {
            VertexIndex ve = circulator.getOtherVertex();
            if( ve == mV1 ) {
              mFace = circulator.getFace();
              mEdge = circulator.getIndex();
              mSide = Side_Concident;
              mHasNext = false;
              break;
            }
        
            if( !mTri.isFinite( ve ) ) {
              // iterate over infinite edges
              circulator.nextCW();
              continue;
            }
        
            const Position& pe = mTri.getPosition( ve );
            Real orient = mTri.getPredicates().orientation( p0, p1, pe );
        
            if( orient == 0 ) {
              Real t = mTri.getPredicates().approximateSegmentParameter( p0, p1, pe );
        
              drAssert( t != 0, "invalid triangulation, p0 == pe, but p0 == getStartVertex(aEdge) == getEndVertex(aEdge) == pe" );
              drAssert( t != 1, "invalid triangulation, p1 == pe, but v1 != ve " );
              drAssert( t <= 1, "invalid triangulation, collinear, pe is not in (p0,p1), overlapping triangle ?" );
        
              if( t > 0 ) {
                // pe is between p0 and p1
                mFace = circulator.getFace();
                mEdge = circulator.getIndex();
                mSide = Side_Concident;
                mHasNext = false;
                break;
              }
              // just on the other side
              orient = 1;
            }
        
            drAssert( orient != 0 );
            if( rotation == 0 ) {
              // "first" call, find circulating direction
              rotation = orient;
            }
        
            if( rotation * orient < 0 ) {
              // orientation has changed -> we have just found the triangle, set aEdge to the intersecting edge
              if( rotation < 0 ) {
                circulator.nextCW();
              }
        
              // this intersection is special, we are coming from a vertex, no side is defined
              mFace = circulator.getFace();
              mEdge = circulator.getIndex().incremented();
              mSide = Side_Concident;
              mHasNext = true;
              break;
            } else if( rotation > 0 ) {
              circulator.nextCW();
            } else {
              circulator.nextCCW();
            }
          }
        */
        //unimplemented!()
    }

    pub fn advance(&mut self) {
        if !self.has_next {
            return;
        }

        let p0 = &self.tri.graph[PositionQuery::Vertex(self.v0)];
        let p1 = &self.tri.graph[PositionQuery::Vertex(self.v1)];

        let nf = self.tri.graph[self.current_face].neighbor(self.current_index);
        let iv = self.tri.graph[nf].get_neighbor_index(self.current_face).unwrap();
        let nv = self.tri.graph[nf].vertex(iv);

        self.current_face = nf;
        if (nv == self.v1) {
            self.current_index = iv.increment();
            self.current_side = Side::Concident;
            self.has_next = false;
        } else {
            let np = &self.tri.graph[PositionQuery::Vertex(nv)];
            let orientation = self.tri.predicates.orientation(p0, p1, np);
            assert(!orientation.is_collinear(), "next != v1, but v0,v1,next are collinear");

            self.current_index = if orientation.is_ccw() {
                iv.increment()
            } else {
                iv.decrement()
            };
            self.current_side = if orientation.is_ccw() {
                Side::CounterClockwise
            } else {
                Side::Clockwise
            };
            self.has_next = true;
        }
    }

    pub fn current() -> Option<(Face, Rot3, Side)> {
        Some(self.face, self.index, self.side)
    }
}
