use build::{Factory, Updater};
use geometry::{CollinearTest, Orientation, Position, Predicates};
use graph::{BuilderContext, Constraint, Face, PredicatesContext, TagContext, Triangulation, Vertex};
use query::{GeometryQuery, TopologyQuery, VertexClue};
use traverse::{Crossing, CrossingIterator, TaggingLocator};
use types::{rot3, FaceEdge, FaceIndex, Location, VertexIndex};

pub trait Builder {
    type Position: Position;
    type Constraint: Constraint;

    /// Add a point to the triangulation and return the index of the generated vertex.
    /// A hint can be provided to find the triangle containing the given point.
    fn add_vertex(&mut self, p: Self::Position, hint: Option<FaceIndex>) -> VertexIndex;

    /// Add the two (new) vertices and a constraining edge between them.
    fn add_constraint_segment(&mut self, p0: Self::Position, p1: Self::Position, c: Self::Constraint);

    /// Add a constraining edge between the given vertices.
    fn add_constraint_edge(&mut self, v0: VertexIndex, v1: VertexIndex, c: Self::Constraint);
}

impl<PR, V, F, C> Triangulation<PR::Position, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
{
    fn add_vertex_at(&mut self, p: PR::Position, loc: Location) -> VertexIndex {
        match loc {
            Location::Empty => {
                let vert = self.create_vertex_with_position(p);
                self.extend_dimension(vert);
                vert
            }
            Location::Vertex(f, v) => self[f].vertex(v),
            Location::Edge(f, e) => {
                let vert = self.create_vertex_with_position(p);
                self.split_edge(f, e, vert);
                vert
            }
            Location::OutsideConvexHull(f) | Location::Face(f) => {
                let vert = self.create_vertex_with_position(p);
                self.split_face(f, vert);
                vert
            }
            Location::OutsideAffineHull => {
                let vert = self.create_vertex_with_position(p);
                self.extend_dimension(vert);
                vert
            }
        }
    }

    /// Adds the constraining edge between the two vertex when dim=1 (all faces are segments)
    fn add_constraint_dim1(&mut self, v0: VertexIndex, v1: VertexIndex, c: &F::Constraint) {
        assert!(self.is_finite_vertex(v0));
        assert!(self.is_finite_vertex(v1));
        assert_ne!(v1, v0);

        // start by the face of the first vertex
        let f0 = self[v0].face();
        let i0 = self[f0].get_vertex_index(v0).unwrap();

        // next vertex
        let vn = self[f0].vertex(i0.mirror(2));
        if vn == v1 {
            // v0-v1 edge was just found
            self[f0].merge_constraint(rot3(2), c.clone());
            return;
        }

        // find the direction to reach v1 from v0
        let reverse_dir = if self.is_finite_vertex(vn) {
            // test direction to traverse by point order
            let p0 = self.p(v0);
            let p1 = self.p(v1);
            let pn = self.p(vn);
            let pr = self.context.predicates();

            // p0,p1,pn and any other (finite) point must be collinear as dim==1,
            let direction = pr.test_collinear_points(p0, p1, pn);
            assert!(
                direction.is_before() || direction.is_between(),
                "Internal error, direction test result"
            );
            direction.is_before()
        } else {
            true
        };

        let (mut cur, mut cur_i) = if reverse_dir {
            // opposite direction
            let next = self[f0].neighbor(i0.mirror(2));
            let next_i = self[next].get_neighbor_index(f0).unwrap().mirror(2);
            (next, next_i)
        } else {
            (f0, i0)
        };

        // mark all edges constraint until the end vertex is reached
        // no geometry have to be tested, as we are on a straight line and no segment may overlap
        loop {
            self[cur].merge_constraint(rot3(2), c.clone());
            if self[cur].vertex(cur_i.mirror(2)) == v1 {
                break;
            }

            let next = self[cur].neighbor(cur_i);
            cur_i = self[next].get_neighbor_index(cur).unwrap().mirror(2);
            cur = next;
        }
    }

    /// Adds the constraining edge between the two vertex when dim=2
    fn add_constraint_dim2(&mut self, mut v0: VertexIndex, v1: VertexIndex, c: &F::Constraint) {
        let edge_chain = self.context.get_face_edge_vector("add_constraint_dim2.edge");
        let mut edge_chain = edge_chain.borrow_mut();
        let top_chain = self.context.get_face_edge_vector("add_constraint_dim2.top");
        let mut top_chain = top_chain.borrow_mut();
        let bottom_chain = self.context.get_face_edge_vector("add_constraint_dim2.bottom");
        let mut bottom_chain = bottom_chain.borrow_mut();

        while v0 != v1 {
            // collect intersecting faces and generate the two (top/bottom) chains
            // The edge-chain is not a whole polygon the new constraining edge is the missing closing edge

            {
                let mut crossing_iter = CrossingIterator::new(self, v0, v1);
                let mut cross = crossing_iter.next();

                // loop over coincident edges
                while let Some(Crossing::CoincidentEdge { face, edge }) = cross {
                    edge_chain.push(FaceEdge { face, edge });
                    cross = crossing_iter.next();
                }

                if let Some(Crossing::Start { face, vertex }) = cross {
                    top_chain.push(FaceEdge {
                        face,
                        edge: vertex.increment(),
                    });
                    bottom_chain.push(FaceEdge {
                        face,
                        edge: vertex.decrement(),
                    });
                    loop {
                        cross = crossing_iter.next();
                        match cross {
                            Some(Crossing::PositiveEdge { face, edge }) => {
                                bottom_chain.push(FaceEdge {
                                    face,
                                    edge: edge.decrement(),
                                });
                            }
                            Some(Crossing::NegativeEdge { face, edge }) => {
                                top_chain.push(FaceEdge {
                                    face,
                                    edge: edge.increment(),
                                });
                            }
                            Some(Crossing::End { face, vertex }) => {
                                top_chain.push(FaceEdge {
                                    face,
                                    edge: vertex.decrement(),
                                });
                                bottom_chain.push(FaceEdge {
                                    face,
                                    edge: vertex.increment(),
                                });
                                break;
                            }
                            _ => unreachable!(),
                        }
                    }
                }
            }

            if !edge_chain.is_empty() {
                v0 = self.vi(VertexClue::end_of(*edge_chain.last().unwrap()));
                for edge in edge_chain.iter() {
                    self.merge_constraint(*edge, c.clone());
                }
            }

            if !top_chain.is_empty() {
                v0 = self.vi(VertexClue::end_of(*bottom_chain.last().unwrap()));
                top_chain.reverse();
                let edge = self.triangulate_hole(&mut top_chain, &mut bottom_chain);
                self.merge_constraint(edge, c.clone());
            }
            edge_chain.clear();
            top_chain.clear();
            bottom_chain.clear();
        }
    }

    fn triangulate_half_hole(&mut self, chain: &mut Vec<FaceEdge>) -> FaceEdge {
        assert!(chain.len() >= 2);
        let mut cur = 0;
        while chain.len() > 2 {
            let next = cur + 1;
            let cur_edge = chain[cur];
            let next_edge = chain[next];

            let p0 = self.vi(VertexClue::start_of(cur_edge));
            let p1 = self.vi(VertexClue::end_of(cur_edge));
            assert_eq!(p1, self.vi(VertexClue::start_of(next_edge)), "Edges are not continouous");
            let p2 = self.vi(VertexClue::end_of(next_edge));

            if !self.get_vertices_orientation(p0, p1, p2).is_ccw() {
                // cannot clip, not an ear
                cur += 1;
                continue;
            }

            // found an ear, clip it
            // Remove the edge only if it is not part of the first or last crossed triangle.
            // These edges are shared by both the upper and lower polygon parts and handled outside

            if next + 1 < chain.len() {
                // remove next from the list and make it the clipped ear
                chain.remove(next);

                self[cur_edge.face].set_vertex(cur_edge.edge.decrement(), p2);
                self[next_edge.face].set_vertex(next_edge.edge, p0);

                let ne = self.opposite_edge(cur_edge);
                self.set_adjacent((ne.face, ne.edge), (next_edge.face, next_edge.edge.decrement()));
                self.set_adjacent((cur_edge.face, cur_edge.edge), (next_edge.face, next_edge.edge.increment()));
                self[p0].set_face(next_edge.face);
                self[p1].set_face(next_edge.face);
                self[p2].set_face(next_edge.face);

                let c = self[cur_edge.face].constraint(cur_edge.edge);
                self[next_edge.face].set_constraint(next_edge.edge.decrement(), c);
                self[cur_edge.face].clear_constraint(cur_edge.edge);
                self[next_edge.face].clear_constraint(next_edge.edge.increment());

                if cur > 0 {
                    // step back
                    cur -= 1;
                }
            } else {
                // remove cur from the list and make it the clipped ear
                assert!(cur > 0);
                chain.remove(cur);

                self[cur_edge.face].set_vertex(cur_edge.edge, p2);
                self[next_edge.face].set_vertex(next_edge.edge.increment(), p0);

                let ne = self.opposite_edge(next_edge);
                self.set_adjacent((ne.face, ne.edge), (cur_edge.face, cur_edge.edge.increment()));
                self.set_adjacent((next_edge.face, next_edge.edge), (cur_edge.face, cur_edge.edge.decrement()));
                self[p0].set_face(cur_edge.face);
                self[p1].set_face(cur_edge.face);
                self[p2].set_face(cur_edge.face);

                let c = self[next_edge.face].constraint(next_edge.edge);
                self[cur_edge.face].set_constraint(cur_edge.edge.increment(), c);
                self[cur_edge.face].clear_constraint(cur_edge.edge.decrement());
                self[next_edge.face].clear_constraint(next_edge.edge);
                // step back
                cur -= 1;
            }
        }

        chain.pop().unwrap()
    }

    /// Triangulates an edge-visible hole given by the edge chain of the upper(lower) polygon.
    /// On completion it returns the edge that separates the upper and lower half of the polygon.
    fn triangulate_hole(&mut self, top: &mut Vec<FaceEdge>, bottom: &mut Vec<FaceEdge>) -> FaceEdge {
        assert!(top.len() >= 2 && bottom.len() >= 2);
        let top = self.triangulate_half_hole(top);
        let bottom = self.triangulate_half_hole(bottom);
        let top = FaceEdge::from(top.face, top.edge.decrement());
        let bottom = FaceEdge::from(bottom.face, bottom.edge.decrement());
        self.set_adjacent(top, bottom);
        self.flip(top.face, top.edge);
        FaceEdge::from(top.face, top.edge.increment())
    }
}

impl<PR, V, F, C> Builder for Triangulation<PR::Position, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: PredicatesContext<Predicates = PR> + TagContext + BuilderContext,
{
    type Position = PR::Position;
    type Constraint = F::Constraint;

    fn add_vertex(&mut self, p: PR::Position, hint: Option<FaceIndex>) -> VertexIndex {
        let location = self.locate_position(&p, hint).unwrap();
        self.add_vertex_at(p, location)
    }

    fn add_constraint_segment(&mut self, p0: PR::Position, p1: PR::Position, c: F::Constraint) {
        assert!(c.is_constraint());
        let v0 = self.add_vertex(p0, None);
        let start_face = self[v0].face();
        let v1 = self.add_vertex(p1, Some(start_face));
        self.add_constraint_edge(v0, v1, c);
    }

    fn add_constraint_edge(&mut self, v0: VertexIndex, v1: VertexIndex, c: F::Constraint) {
        assert!(c.is_constraint());
        assert!(v0.is_valid());
        assert!(v1.is_valid());
        assert!(self.is_finite_vertex(v0));
        assert!(self.is_finite_vertex(v1));
        if v0 == v1 {
            return;
        }

        match self.dimension() {
            1 => self.add_constraint_dim1(v0, v1, &c),
            2 => self.add_constraint_dim2(v0, v1, &c),
            _ => unreachable!("Inconsistent triangulation"),
        }
    }
}
