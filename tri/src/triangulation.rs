use geometry::Predicates;
use graph::{Face, Graph, Vertex};

pub struct Triangulation<PR, V, F, C = ()>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
{
    pub graph: Graph<PR::Position, V, F>,
    pub predicates: PR,
    //pub tag: RefCell<usize>,
    pub context: C,
}

impl<PR, V, F, C> Triangulation<PR, V, F, C>
where
    PR: Predicates,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: Default,
{
    pub fn new_with_predicates(predicates: PR) -> Triangulation<PR, V, F, C> {
        Triangulation {
            predicates,
            graph: Default::default(),
            //tag: RefCell::new(0),
            context: Default::default(),
        }
    }
}

impl<PR, V, F, C> Triangulation<PR, V, F, C>
where
    PR: Predicates + Default,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: Default,
{
    pub fn new() -> Triangulation<PR, V, F, C> {
        Triangulation::new_with_predicates(PR::default())
    }
}

impl<PR, V, F, C> Default for Triangulation<PR, V, F, C>
where
    PR: Predicates + Default,
    V: Vertex<Position = PR::Position>,
    F: Face,
    C: Default,
{
    fn default() -> Triangulation<PR, V, F, C> {
        Triangulation::new()
    }
}

use builder::BuilderContext;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use tagginglocator::TagContext;
use vertexchain::ChainStore;

pub struct TagCtx(RefCell<usize>);

impl TagCtx {
    fn new() -> TagCtx {
        TagCtx(RefCell::new(0))
    }
}

impl TagContext for TagCtx {
    fn tag(&self) -> RefMut<usize> {
        self.0.borrow_mut()
    }
}

pub struct BuilderCtx(RefCell<ChainStore>);

impl BuilderCtx {
    fn new() -> BuilderCtx {
        BuilderCtx(RefCell::new(ChainStore::new()))
    }
}

impl BuilderContext for BuilderCtx {
    fn chain_store(&self) -> RefMut<ChainStore> {
        self.0.borrow_mut()
    }
}

pub trait HList: Sized {
    fn with<H>(self, head: H) -> HCons<H, Self> {
        HCons { head, tail: self }
    }
}

pub struct HNil;
pub struct No;
pub struct Yes;

impl HList for HNil {}

pub struct HCons<H, T> {
    head: H,
    tail: T,
}

impl<H, T: HList> HList for HCons<H, T> {}

impl<T: HList> TagContext for HCons<TagCtx, T> {
    fn tag(&self) -> RefMut<usize> {
        self.head.tag()
    }
}

impl<H: HList + TagContext> TagContext for H {
    fn tag(&self) -> RefMut<usize> {
        self.tag()
    }
}

impl<T: HList> BuilderContext for HCons<BuilderCtx, T> {
    fn chain_store(&self) -> RefMut<ChainStore> {
        self.head.chain_store()
    }
}

impl<H: BuilderContext> BuilderContext for H {
    fn chain_store(&self) -> RefMut<ChainStore> {
        self.chain_store()
    }
}

fn foo() {
    let a = HNil.with(TagCtx::new()).with(BuilderCtx::new());
    {
        let _ = a.chain_store();
    }
    {
        let _ = a.tag();
    }
}
