use geometry::{CollinearTest, Orientation, Predicates};
use triangulation::{Face, TriGraph, Vertex};
use types::{FaceIndex, Rot3, VertexIndex};

pub struct Checker<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    tri: &'a TriGraph<P, V, F>,
}

impl<'a, P, V, F> Checker<'a, P, V, F>
where
    P: 'a + Predicates,
    V: 'a + Vertex<Position = P::Position>,
    F: 'a + Face,
{
    pub fn new(tri: &TriGraph<P, V, F>) -> Checker<P, V, F> {
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
        } else if self.tri.dimension() < 3 {
            let finite_vertex_count = self
                .tri
                .vertex_index_iter()
                .filter(|&v| self.tri.is_infinite_vertex(v))
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
                    let d = Rot3(r);
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

            Ok(())
        } else {
            Err(format!("Invalid dimension: {}", self.tri.dimension()))
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
                .ok_or(format!("Vertex-face link is invalid {:?} is not a neighbor of {:?}", nf, v))?;
        }
        Ok(())
    }

    fn check_face_face_link(&self) -> Result<(), String> {
        assert!(self.tri.dimension() >= 0);
        for f in self.tri.face_index_iter() {
            for d in 0..self.tri.dimension() {
                let i = Rot3(d as u8);
                let nf = self.tri[f].neighbor(i);
                if !nf.is_valid() {
                    return Err(format!(
                        "Face-face link is invalid, no neighboring face for {:?} at {:?}",
                        f, i
                    ));
                }

                let ni = self.tri[nf].get_neighbor_index(f).ok_or(format!(
                    "Face-face link is invalid, missing backward link between ({:?},{:?}) and {:?}",
                    f, i, nf
                ))?;

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

    pub fn check(&self) -> Result<(), String> {
        self.check_dimension()?;
        Ok(())
    }
}
