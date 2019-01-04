use geometry2::{Orientation, Position, Predicates, Real};
use log::trace;
use triangulation::graph::{Face, PredicatesContext, Triangulation, Vertex};
use triangulation::query::{GeometryQuery, TopologyQuery};
use triangulation::types::{rot3, FaceVertex};

pub trait GeometryChecker {
    /// Check geometry predicates.
    fn check_orientation(&self) -> Result<(), String>;

    /// Check if the area of the convex hull and the sum of area of the triangles are within the given tolerance.
    fn check_area(&self, eps: Option<f64>) -> Result<(), String>;
}

impl<PR, V, F, C> GeometryChecker for Triangulation<PR::Position, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: PredicatesContext<Predicates = PR>,
{
    fn check_orientation(&self) -> Result<(), String> {
        if self.dimension() < 2 {
            return Ok(());
        }

        for f in self.face_index_iter() {
            if self.is_infinite_face(f) {
                continue;
            }

            let v0 = self[f].vertex(rot3(0));
            let v1 = self[f].vertex(rot3(1));
            let v2 = self[f].vertex(rot3(2));

            if !self.get_vertices_orientation(v0, v1, v2).is_ccw() {
                return Err(format!("Count-clockwise property is violated for {:?}", f));
            }
        }

        Ok(())
    }

    fn check_area(&self, eps: Option<f64>) -> Result<(), String> {
        if self.dimension() != 2 {
            return Ok(());
        }

        // calculate the area of the triangles
        let mut tri_area = 0.;
        for f in self.face_index_iter() {
            if self.is_infinite_face(f) {
                continue;
            }

            let a = self.p(FaceVertex::from(f, rot3(0)));
            let b = self.p(FaceVertex::from(f, rot3(1)));
            let c = self.p(FaceVertex::from(f, rot3(2)));

            let ax: f64 = a.x().approximate();
            let ay: f64 = a.y().approximate();
            let bx: f64 = b.x().approximate();
            let by: f64 = b.y().approximate();
            let cx: f64 = c.x().approximate();
            let cy: f64 = c.y().approximate();
            let abx = bx - ax;
            let aby = by - ay;
            let acx = cx - ax;
            let acy = cy - ay;
            tri_area += abx * acy - aby * acx; // twice the area of the triangle
        }

        // calculate the area of the convex hull
        let mut convex_area = 0.;
        let end = self.infinite_face();
        let mut cur = end;
        loop {
            let iid = self[cur].get_vertex_index(self.infinite_vertex()).unwrap(); // index of infinite vertex
            let aid = iid.decrement();
            let bid = iid.increment();
            let a = self.p(FaceVertex::from(cur, aid));
            let b = self.p(FaceVertex::from(cur, bid));
            let ax: f64 = a.x().approximate();
            let ay: f64 = a.y().approximate();
            let bx: f64 = b.x().approximate();
            let by: f64 = b.y().approximate();

            convex_area += ax * by - bx * ay;
            cur = self[cur].neighbor(aid);
            if cur == end {
                break;
            }
        }

        trace!(
            "tri_area={}, convex_area={}, area_diff={}",
            tri_area,
            convex_area,
            convex_area - tri_area
        );

        let eps = eps.unwrap_or(1e-12);
        if (convex_area - tri_area).abs() > tri_area * eps {
            Err(format!(
                "Area of convex hull differs from polygon too much: tri_area={}, convex_area={}, area_diff={}",
                tri_area,
                convex_area,
                convex_area - tri_area
            ))
        } else {
            Ok(())
        }
    }
}
