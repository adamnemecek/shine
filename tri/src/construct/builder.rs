use construct::{Factory, Updater};
use context::{BuilderContext, PredicatesContext, TagContext};
use geometry::{CollinearTest, Position, Predicates};
use graph::{Constraint, Face, Vertex, VertexQuery};
use traverse::TaggingLocator;
use triangulation::Triangulation;
use types::{rot3, FaceIndex, Location, VertexIndex};

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
            Location::Vertex(f, v) => {
                let vert = self.graph[f].vertex(v);
                vert
            }
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
        assert!(self.graph.is_finite_vertex(v0));
        assert!(self.graph.is_finite_vertex(v1));
        assert_ne!(v1, v0);

        // start by the face of the first vertex
        let f0 = self.graph[v0].face();
        let i0 = self.graph[f0].get_vertex_index(v0).unwrap();

        // next vertex
        let vn = self.graph[f0].vertex(i0.mirror(2));
        if vn == v1 {
            // v0-v1 edge was just found
            self.graph[f0].merge_constraint(rot3(2), c.clone());
            return;
        }

        // find the direction to reach v1 from v0
        let reverse_dir = if self.graph.is_finite_vertex(vn) {
            // test direction to traverse by point order
            let p0 = self.graph.pos(v0);
            let p1 = self.graph.pos(v1);
            let pn = self.graph.pos(vn);
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
            let next = self.graph[f0].neighbor(i0.mirror(2));
            let next_i = self.graph[next].get_neighbor_index(f0).unwrap().mirror(2);
            (next, next_i)
        } else {
            (f0, i0)
        };

        // mark all edges constraint until the end vertex is reached
        // no geometry have to be tested, as we are on a straight line and no segment may overlap
        loop {
            self.graph[cur].merge_constraint(rot3(2), c.clone());
            if self.graph[cur].vertex(cur_i.mirror(2)) == v1 {
                break;
            }

            let next = self.graph[cur].neighbor(cur_i);
            cur_i = self.graph[next].get_neighbor_index(cur).unwrap().mirror(2);
            cur = next;
        }
    }

    /// Adds the constraining edge between the two vertex when dim=2
    fn add_constraint_dim2(&mut self, v0: VertexIndex, _v1: VertexIndex, _c: &F::Constraint) {
        //let chain_store = &mut context.chain_store;
        let mut _chain_store = self.context.chain_store();
        let mut _start = v0;
        /*while start != v1 {
            println!("add constrainat: {:?}->{:?}", start, v1);
            // collect intersecting faces and generate the two (top/bottom) chains
            // The edge-chain is not a whole polygon the new constraining edge is the missing closing edge

            let (face, edge) = 'itertion_block: {
                let mut top_chain;
                let mut bottom_chain;
                {
                    let mut crossing_iter = CrossingIterator::new(self, start, v1);
                    let cross = crossing_iter.next().unwrap();
                    println!("crossing edge: {:?}", cross);

                    if cross.side == CrossingSide::Coincident {
                        // single coincident edge detected, no hole filling is required
                        break 'itertion_block (cross.face, cross.edge);
                    }

                    top_chain = chain_store.new_chain(cross.face, cross.edge.increment(), true);
                    bottom_chain = chain_store.new_chain(cross.face, cross.edge.decrement(), true);

                    loop {
                        let cross = crossing_iter.next();
                        println!("crossing edge: {:?}", cross);

                        if cross.is_none() {
                            break;
                        }

                        let cross = cross.unwrap();

                        //test if crossed edge is constrainet
                        //todo

                        match cross.side {
                            CrossingSide::Coincident => {
                                break;
                            }
                            CrossingSide::CW => {}
                            CrossingSide::CCW => {}
                        }
                    }
                }
                self.triangulate_hole(chain_store, top_chain, bottom_chain)
            };
            chain_store.clear();

            self.make_constraint(face, edge, c.clone());
            start = self.graph.index_get(VertexQuery::EdgeEnd(face, edge));
        }*/
    }

    /*fn triangulate_hole(&mut self, chains: &mut ChainStore, top: ChainIndex, bottom: ChainIndex) -> (FaceIndex, Rot3) {
        chains.dump(top, &mut std::io::stdout()).unwrap();
        unimplemented!()
    }*/
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
        let start_face = self.graph[v0].face();
        let v1 = self.add_vertex(p1, Some(start_face));
        self.add_constraint_edge(v0, v1, c);
    }

    fn add_constraint_edge(&mut self, v0: VertexIndex, v1: VertexIndex, c: F::Constraint) {
        assert!(c.is_constraint());
        assert!(v0.is_valid());
        assert!(v1.is_valid());
        assert!(self.graph.is_finite_vertex(v0));
        assert!(self.graph.is_finite_vertex(v1));
        if v0 == v1 {
            return;
        }

        match self.graph.dimension() {
            1 => self.add_constraint_dim1(v0, v1, &c),
            2 => self.add_constraint_dim2(v0, v1, &c),
            _ => unreachable!("Inconsistent triangulation"),
        }
    }
}
