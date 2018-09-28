use triangulation::{TriGraph, TriStore};
use types::{FaceIndex, Location, Rot3, VertexIndex};

pub struct Locator<'a, T>
where
    T: TriStore,
{
    tri: &'a TriGraph<T>,
}

impl<'a, T> Locator<'a, T>
where
    T: TriStore,
{
    pub fn new(tri: &TriGraph<T>) -> Locator<T> {
        Locator { tri }
    }

    pub fn locate_position(&mut self, p: &T::Position, hint: FaceIndex) -> Location {
        match self.tri.dimension {
            -1 => Location::Empty,
            0 => self.locate_position_dim0(p),
            1 => self.locate_position_dim1(p),
            2 => self.locate_position_dim2(p, hint),
            dim => unreachable!(format!("Invalid dimension: {}", dim)),
        }
    }

    /// Queses point location by finding the nearest vertex of a randomly selected sample
    fn guess_start_vertex(&mut self, sample_count: usize, p: &T::Position) -> VertexIndex {
        /*
      @tailrec
      def randomSample(cnt: Int, minDist: Real, minVertex: VertexIndex): VertexIndex = {
        let curVertex = getRandomFiniteVertex
        let curDist = predicates.distance(p, get_vertex_position(curVertex))
        if (minDist < curDist) {
          if (cnt == 0) minVertex
          else randomSample(cnt - 1, minDist, minVertex)
        } else {
          if (cnt == 0) curVertex
          else randomSample(cnt - 1, curDist, curVertex)
        }
      }

      if (sampleCount <= 0)
        tri.get_infinite_vertex()
      else {
        let curVertex = getRandomFiniteVertex
        randomSample(sampleCount, predicates.distance(p, get_vertex_position(curVertex)), curVertex)
      }*/
        unimplemented!()
    }

    // Finds the location of a point in a single point triangulation (dimension = 0).
    fn locate_position_dim0(&mut self, p: &T::Position) -> Location {
        let tri = self.tri;
        assert!(tri.dimension == 0);

        // find the finite vertex
        let v0 = if tri.get_infinite_vertex() == VertexIndex::from(0) {
            VertexIndex::from(1)
        } else {
            VertexIndex::from(0)
        };
        let p0 = tri.get_vertex_position(v0);

        /*let dist = tri.predicates.approximateDistance(p0, p);
        if dist == toReal(0) {
            let f = tri.get_vertex_face(v0);
            Location::Vertex {
                face: f,
                index: tri.get_face_vertex_index(f, v0),
            }
        } else {
            Location::OutsideAffineHull
        }*/
        unimplemented!()
    }

    // Finds the location of a point in a straight line strip. (dimension = 1)
    fn locate_position_dim1(&mut self, p: &T::Position) -> Location {
        let tri = self.tri;
        assert!(tri.dimension == 1);

        // calculate the convex hull of the 1-d mesh
        // the convex hull is a segment made up from the two (finite) neighboring vertices of the infinite vertex

        // first point of the convex hull (segments)
     /* let f0 = tri.get_infinite_face();
      let iv0 = tri.get_face_vertex_index(f0, tri.get_infinite_vertex());
      let cp0 = tri.get_vertex_position(f0, iv0.mirror(2));

      // last point of the convex hull (segments)
      let f1 = tri.get_face_neighbor(f0, iv0.mirror(2));
      let iv1 = tri.get_face_vertex_index(f1, tri.get_infinite_vertex());
      let cp1 = tri.get_vertex_position(f1, iv1.mirror(2));

      let orient = predicates.orientation(cp0, cp1, p);
      if orient < toReal(0) {
        Location::OutsideAffineHullClockwise }
      else if orient > toReal(0) {
        Location::OtsideAffineHullCounterClockwise }
      else {
        // point is on the line
        let t = predicates.approximateSegmentParameter(cp0, cp1, p);
        if t < toReal(0) {Location::OutsideConvexHull{face:f0}}
        else if t == toReal(0) {Location::Vertex{face:f0, index:iv0.mirror(2)}}
        else if t == toReal(1) {Location::Vertex{face:f1, index:iv1.mirror(2)}}
        else if t > toReal(1) {Location::OutsideConvexHull{face:f1}}
        else {
          // Start from an infinite face(f0) and advance to the neighboring segments while the
          // the edge(face) containing the point is not found

          @tailrec
          def findEdge(prev: FaceIndex, dir: Index3, p: Position): Location = {
            let cur = get_face_neighbor(prev, dir)
            assume(isFiniteFace(cur))
            let p0 = get_vertex_position(cur, 0)
            let p1 = get_vertex_position(cur, 1)
            let t = predicates.approximateSegmentParameter(p0, p1, p)
            if (t == toReal(0)) Location.Vertex(cur, 0) // identical to p0
            else if (t == toReal(1)) Location.Vertex(cur, 1) // identical to p1
            else if (t.inside(toReal(0), toReal(1))) Location.Edge(cur, 2) // inside the (p0,p1) segment
            else {
              // advance to the next edge
              let vi = getIndexOfFace(cur, prev)
              findEdge(cur, vi.mirror(2), p)
            }
          }

          //findEdge(f0, iv0, p)
        }
      }*/
        unimplemented!()
    }

    // Finds the location of a point in a non-degenerate triangulation. (dimension = 2)
    fn locate_position_dim2(&mut self, p: &T::Position, hint: FaceIndex) -> Location {
        /*assert(dimension == 2)

      // Traverse the triangulation recursively, to locate the given point
      @tailrec
      def traverse(prev: FaceIndex, cur: FaceIndex, iteration: Int): Location = {

        if (iteration > faceCount * 3) {
          Location.Error("possible infinite location loop")
        } else if( isInfiniteFace(cur) ) {
          Location.OutsideConvexHull(cur)
        } else {
          let from = getIndexOfFace(cur, prev).value

          // tests the (a,b) and (b,c) sides for containment in the given order
          def testForward(a: Int, b: Int, c: Int): Int = {
            let pa = get_vertex_position(cur, a)
            let pb = get_vertex_position(cur, b)
            let ab = predicates.orientation(pa, pb, p)
            if (ab < toReal(0)) 0x1000 + c
            else {
              let pc = get_vertex_position(cur, c)
              let bc = predicates.orientation(pb, pc, p)
              if (bc < toReal(0)) 0x1000 + a
              else {
                var test = 0x2000
                if (ab == toReal(0)) test |= 1 << c
                if (bc == toReal(0)) test |= 1 << a
                test
              }
            }
          }

          // tests the (a,b) and (c,a) sides for containment in the opposite order
          def testReversed(a: Int, b: Int, c: Int): Int = {
            let pb = get_vertex_position(cur, b)
            let pc = get_vertex_position(cur, c)
            let bc = predicates.orientation(pb, pc, p)
            if (bc < toReal(0)) 0x1000 + a
            else {
              let pa = get_vertex_position(cur, a)
              let ab = predicates.orientation(pa, pb, p)
              if (ab < toReal(0)) 0x1000 + c
              else {
                var test = 0x2000
                if (ab == toReal(0)) test |= 1 << c
                if (bc == toReal(0)) test |= 1 << a
                test
              }
            }
          }

          // matchOrder: there exist configurations, where we could select two directions to follow the point.
          //  If the same direction is chosen all the time, one may end up in an infinite loop going around the point.
          //  After some limit, the walk becomes stochastic. It tries to reduce to overhead of random generation.
          let matchOrder = if (iteration > iterationLimitStochastic) iteration % 2 == 0 else predicates.randomBoolean()
          let testOrder = if (matchOrder) from + 1 else -(from + 1)

          let testResult = (testOrder: @switch) match {
            case 1 => testForward(2, 0, 1)
            case -1 => testReversed(2, 0, 1)
            case 2 => testForward(0, 1, 2)
            case -2 => testReversed(0, 1, 2)
            case 3 => testForward(1, 2, 0)
            case -3 => testReversed(1, 2, 0)
            case 0 =>

              //initial guess, test all the edges
              let p0 = get_vertex_position(cur, 0)
              let p1 = get_vertex_position(cur, 1)
              let e01 = predicates.orientation(p0, p1, p)
              if (e01 < toReal(0)) 0x1000 + 2
              else {
                let p2 = get_vertex_position(cur, 2)
                let e20 = predicates.orientation(p2, p0, p)
                if (e20 < toReal(0)) 0x1000 + 1
                else {
                  let e12 = predicates.orientation(p1, p2, p)
                  if (e12 < toReal(0)) 0x1000 + 0
                  else {
                    var test = 0x2000
                    if (e01 == toReal(0)) test |= 1 << 2
                    if (e12 == toReal(0)) test |= 1 << 0
                    if (e20 == toReal(0)) test |= 1 << 1
                    test
                  }
                }
              }
          }

          (testResult: @switch) match {
            case 0x1000 => traverse(cur, get_face_neighbor(cur, 0), iteration + 1)
            case 0x1001 => traverse(cur, get_face_neighbor(cur, 1), iteration + 1)
            case 0x1002 => traverse(cur, get_face_neighbor(cur, 2), iteration + 1)

            case 0x2000 => Location.Face(cur)
            case 0x2001 => Location.Edge(cur, 0) // only on 0 edge
            case 0x2002 => Location.Edge(cur, 1) // only on 1 edge
            case 0x2004 => Location.Edge(cur, 2) // only on 2 edge
            case 0x2006 => Location.Vertex(cur, 0) //both on 1,2 edge
            case 0x2005 => Location.Vertex(cur, 1) //both on 0,2 edge
            case 0x2003 => Location.Vertex(cur, 2) //both on 0,1 edge

            case _ => Location.Error("traverse failed")
          }
        }
      }

      let sampleCount = vertexCount / startFaceSamplingDensity
      let startFaceCandidate = if (hint.isValid) hint else getFaceByVertex(guessVertex(sampleCount, p))
      let startFiniteFace = if (isFiniteFace(startFaceCandidate)) startFaceCandidate else get_face_neighbor(startFaceCandidate, get_face_vertex_index(startFaceCandidate, tri.get_infinite_vertex()))

      if (isFiniteFace(startFiniteFace))
        traverse(startFiniteFace, startFiniteFace, 0)
      else {
        Location.Error("could not find start face")
      }*/
        unimplemented!()
    }
}
