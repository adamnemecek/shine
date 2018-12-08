use construct::{Factory, Updater};
use context::{BuilderContext, PredicatesContext, TagContext};
use geometry::{CollinearTest, Position, Predicates};
use graph::{Constraint, Face, Vertex, VertexQuery};
use traverse::TaggingLocator;
use traverse::{Crossing, CrossingIterator};
use triangulation::Triangulation;
use types::{rot3, FaceIndex, Location, VertexClue, VertexIndex};
use vertexchain::{Chain, ChainStore};

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
    fn add_constraint_dim2(&mut self, mut v0: VertexIndex, v1: VertexIndex, _c: &F::Constraint) {
        let mut edge_chain = self.context.create_chain(false);
        let mut top_chain = self.context.create_chain(true);
        let mut bottom_chain = self.context.create_chain(true);

        while v0 != v1 {
            println!("add constrainat: {:?}->{:?}", v0, v1);
            // collect intersecting faces and generate the two (top/bottom) chains
            // The edge-chain is not a whole polygon the new constraining edge is the missing closing edge
            
            let mut crossing_iter = CrossingIterator::new(self, v0, v1);
            let mut cross = crossing_iter.next();
            println!("cross edge: {:?}", cross);

            // loop over coincident edges
            while let Some(Crossing::CoincidentEdge { face, edge }) = cross {
                edge_chain.push_back(face, edge);
                v0 = self.graph.vi(VertexClue::edge_end(face, edge));
                cross = crossing_iter.next();
                println!("cross edge: {:?}, v0: {:?}", cross, v0);
            }

            if let Some(Crossing::Start { face, vertex }) = cross {
                top_chain.push_back(face, vertex);
                bottom_chain.push_back(face, vertex);
                loop {
                    cross = crossing_iter.next();
                    println!("cross edge: {:?}", cross);
                    if let Some(Crossing::End { face, vertex }) = cross {
                        v0 = self.graph.vi(VertexClue::face_vertex(face, vertex));
                        println!("v0: {:?}", v0);
                        break;
                    }
                }
            }

            if !edge_chain.is_empty() {
                println!("edge constraint:{:?}", edge_chain);
            }

            if top_chain.is_empty() {
                println!("fill hole\ntop:{:?}\nbottom:{:?}", top_chain, bottom_chain);
                //self.triangulate_hole(&mut top_chain, bottom_chain);
            }
            edge_chain.release();
            top_chain.release();
            bottom_chain.release();
        }
    }

    fn triangulate_hole(&mut self, chains: &mut ChainStore, top: &mut Chain, bottom: &mut Chain) {
        //chains.dump(top, &mut std::io::stdout()).unwrap();
        //unimplemented!()
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
