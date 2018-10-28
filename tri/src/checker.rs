use geometry::{Orientation, Real, Position, Predicates};
use graph::{Face, Graph, Vertex};
use indexing::PositionQuery;
use types::rot3;

pub struct Checker<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    tri: &'a Graph<P, V, F>,
}

impl<'a, P, V, F> Checker<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &Graph<P, V, F>) -> Checker<P, V, F> {
        Checker { tri }
    }

    pub fn check_dimension(&self) -> Result<(), String> {
        if self.tri.dimension() == -1 {
            if self.tri.vertex_count() != 0 {
                Err(format!("Empty triangulation has vertices: {}", self.tri.vertex_count()))
            } else if self.tri.face_count() != 0 {
                Err(format!("Empty triangulation has faces: {}", self.tri.face_count()))
            } else if self.tri.infinite_vertex().is_valid() {
                Err(format!(
                    "Empty triangulation has a valid infinite vertex: {:?}",
                    self.tri.infinite_vertex()
                ))
            } else {
                Ok(())
            }
        } else {
            let finite_vertex_count = self
                .tri
                .vertex_index_iter()
                .filter(|&v| !self.tri.is_infinite_vertex(v))
                .count();
            if finite_vertex_count != self.tri.vertex_count() - 1 {
                return Err(format!(
                    "Number of finite vertices is invalid, got {}, expected: {}",
                    finite_vertex_count,
                    self.tri.vertex_count() - 1
                ));
            }

            let mut finite_face_count = 0;
            let mut infinite_face_count = 0;
            for f in self.tri.face_index_iter() {
                for r in 0..3 {
                    let d = rot3(r);
                    if self.tri[f].vertex(d).is_valid() != (r <= self.tri.dimension() as u8) {
                        return Err(format!(
                            "A face({:?}) has invalid dimension at {:?} (dim:{})",
                            f,
                            d,
                            self.tri.dimension()
                        ));
                    }
                }
                if self.tri.is_infinite_face(f) {
                    infinite_face_count += 1;
                } else {
                    finite_face_count += 1;
                }
            }

            if self.tri.dimension() == 0 {
                if finite_face_count != 1 {
                    Err(format!(
                        "Face count does not match for dim0, (f = 1), f={}",
                        finite_face_count
                    ))
                } else if infinite_face_count != 1 {
                    Err(format!(
                        "Infinit face count does not match for dim0: (h = 1), h={}",
                        infinite_face_count
                    ))
                } else if finite_vertex_count != 1 {
                    Err(format!(
                        "Vertex count does not match for dim0: (v = 1), v={}",
                        finite_vertex_count
                    ))
                } else {
                    Ok(())
                }
            } else if self.tri.dimension() == 1 {
                if infinite_face_count != 2 {
                    Err(format!(
                        "Infinite face count does not match hull count for dim1: (if = h), if={},h=2",
                        infinite_face_count
                    ))
                } else if finite_face_count + 1 != finite_vertex_count {
                    Err(format!(
                        "Vertex, face count does not match for dim1: (v = f+1), v={},f={}",
                        finite_vertex_count, finite_face_count
                    ))
                } else {
                    Ok(())
                }
            } else if self.tri.dimension() == 2 {
                let mut hull_count = 0;
                let end = self.tri.infinite_face();
                let mut cur = end;
                loop {
                    hull_count += 1;
                    let iid = self.tri[cur].get_vertex_index(self.tri.infinite_vertex()).unwrap(); // index of infinite vertex
                    let aid = iid.decrement();
                    cur = self.tri[cur].neighbor(aid);
                    if cur == end {
                        break;
                    }
                }

                if hull_count != infinite_face_count {
                    Err(format!(
                        "Infinite face count does not match hull count for dim2: (if = h), if={},h={}",
                        infinite_face_count, hull_count
                    ))
                } else if 2 * finite_vertex_count != finite_face_count + hull_count + 2 {
                    // https://en.wikipedia.org/wiki/Point_set_triangulation#Combinatorics_in_the_plane
                    Err(format!(
                        "Vertex, face count does not match for dim2: (2v-h-2 = f), v={},f={},h={}",
                        finite_vertex_count, finite_face_count, hull_count
                    ))
                } else {
                    Ok(())
                }
            } else {
                Err(format!("Invalid dimension: {}", self.tri.dimension()))
            }
        }
    }

    fn check_vertex_face_link(&self) -> Result<(), String> {
        for v in self.tri.vertex_index_iter() {
            if !self.tri[v].face().is_valid() {
                return Err(format!("Vertex-face link is invalid, no face for {:?} ", v));
            }

            let nf = self.tri[v].face();
            let _vi = self.tri[nf]
                .get_vertex_index(v)
                .ok_or_else(|| format!("Vertex-face link is invalid {:?} is not a neighbor of {:?}", nf, v))?;
        }
        Ok(())
    }

    fn check_face_face_link(&self) -> Result<(), String> {
        for f in self.tri.face_index_iter() {
            for d in 0..self.tri.dimension() {
                let i = rot3(d as u8);
                let nf = self.tri[f].neighbor(i);
                if !nf.is_valid() {
                    return Err(format!(
                        "Face-face link is invalid, no neighboring face for {:?} at {:?}",
                        f, i
                    ));
                }

                let ni = self.tri[nf].get_neighbor_index(f).ok_or_else(|| {
                    format!(
                        "Face-face link is invalid, missing backward link between ({:?},{:?}) and {:?}",
                        f, i, nf
                    )
                })?;

                match self.tri.dimension() {
                    1 => {
                        if self.tri[f].vertex(i.mirror(2)) != self.tri[nf].vertex(ni.mirror(2)) {
                            return Err(format!(
                                "Face-face link is invalid, vertex relation in dim1 ({:?},{:?}) <-> ({:?},{:?})",
                                f, i, nf, ni
                            ));
                        }
                    }
                    2 => {
                        if self.tri[f].vertex(i.decrement()) != self.tri[nf].vertex(ni.increment())
                            || self.tri[f].vertex(i.increment()) != self.tri[nf].vertex(ni.decrement())
                        {
                            return Err(format!(
                                "Face-face link is invalid, vertex relation in dim2 ({:?},{:?}) <-> ({:?},{:?})",
                                f, d, nf, ni
                            ));
                        }
                        if self.tri[f].constraint(i) != self.tri[nf].constraint(ni) {
                            return Err(format!(
                                "Face-face link is invalid, non-matching constraints in dim2 ({:?},{:?}) <-> ({:?},{:?})",
                                f, d, nf, ni
                            ));
                        }
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn check_topology(&self) -> Result<(), String> {
        self.check_vertex_face_link()?;
        self.check_face_face_link()?;
        Ok(())
    }

    pub fn check_orientation(&self) -> Result<(), String> {
        if self.tri.dimension() < 2 {
            return Ok(());
        }

        for f in self.tri.face_index_iter() {
            if self.tri.is_infinite_face(f) {
                continue;
            }

            let v0 = self.tri[f].vertex(rot3(0));
            let v1 = self.tri[f].vertex(rot3(1));
            let v2 = self.tri[f].vertex(rot3(2));

            if !self.tri.get_vertices_orientation(v0, v1, v2).is_ccw() {
                return Err(format!("Count-clockwise property is violated for {:?}", f));
            }
        }

        Ok(())
    }

    pub fn check_area(&self, eps: Option<f64>) -> Result<(), String> {
        if self.tri.dimension() != 2 {
            return Ok(());
        }

        // calculate the area of the triangles
        let mut tri_area = 0.;
        for f in self.tri.face_index_iter() {
            if self.tri.is_infinite_face(f) {
                continue;
            }

            let a = &self.tri[PositionQuery::Face(f, rot3(0))];
            let b = &self.tri[PositionQuery::Face(f, rot3(1))];
            let c = &self.tri[PositionQuery::Face(f, rot3(2))];

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
        let end = self.tri.infinite_face();
        let mut cur = end;
        loop {
            let iid = self.tri[cur].get_vertex_index(self.tri.infinite_vertex()).unwrap(); // index of infinite vertex
            let aid = iid.decrement();
            let bid = iid.increment();
            let a = &self.tri[PositionQuery::Face(cur, aid)];
            let b = &self.tri[PositionQuery::Face(cur, bid)];
            let ax: f64 = a.x().approximate();
            let ay: f64 = a.y().approximate();
            let bx: f64 = b.x().approximate();
            let by: f64 = b.y().approximate();

            convex_area += ax * by - bx * ay;
            cur = self.tri[cur].neighbor(aid);
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

    pub fn check(&self, eps_area: Option<f64>) -> Result<(), String> {
        self.check_dimension()?;
        self.check_topology()?;
        self.check_orientation()?;
        self.check_area(eps_area)?;
        Ok(())
    }
}
