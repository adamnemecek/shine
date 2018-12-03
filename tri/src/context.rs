use checker::{Coloring, TraceMapping, TraceRender};
use geometry::{ExactPredicates, ExactReal, InexactPredicates, InexactReal, Position, Predicates};
use graph::{Face, Vertex};
use std::cell::{Ref, RefCell, RefMut};
use std::marker::PhantomData;
use triangulation::Triangulation;
use vertexchain::ChainStore;

/// Context the enables/disables triangulation features and also stores the required datas
pub struct Context<P, V, F, Predicates = (), Tag = (), Builder = (), Trace = ()>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    predicates: Predicates,
    tag: Tag,
    builder: Builder,
    trace: Trace,
    phantom: PhantomData<(P, V, F)>,
}

impl<P, V, F> Context<P, V, F, (), (), (), ()>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn new() -> Context<P, V, F> {
        Context {
            predicates: (),
            tag: (),
            builder: (),
            trace: (),
            phantom: PhantomData,
        }
    }
}

impl<P, V, F, Predicates, Tag, Builder, Trace> Context<P, V, F, Predicates, Tag, Builder, Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn create(self) -> Triangulation<P, V, F, Self> {
        Triangulation::new(self)
    }
}

/// Trait to provide geometry predicates
pub trait PredicatesContext {
    type Predicates: Predicates;

    fn predicates(&self) -> &Self::Predicates;
}

impl<P, V, F, Tag, Builder, Trace> Context<P, V, F, (), Tag, Builder, Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_predicates<PR>(self, predicates: PR) -> Context<P, V, F, PR, Tag, Builder, Trace>
    where
        PR: Predicates<Position = P>,
    {
        Context {
            predicates,
            tag: self.tag,
            builder: self.builder,
            trace: self.trace,
            phantom: self.phantom,
        }
    }
}

impl<P, V, F, Tag, Builder, Trace> Context<P, V, F, (), Tag, Builder, Trace>
where
    P: Position,
    P::Real: InexactReal,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_inexact_predicates(self) -> Context<P, V, F, InexactPredicates<P>, Tag, Builder, Trace> {
        Context {
            predicates: InexactPredicates::new(),
            tag: self.tag,
            builder: self.builder,
            trace: self.trace,
            phantom: self.phantom,
        }
    }

    pub fn with_inexact_predicates_eps(self, eps: P::Real) -> Context<P, V, F, InexactPredicates<P>, Tag, Builder, Trace> {
        Context {
            predicates: InexactPredicates::with_eps(eps),
            tag: self.tag,
            builder: self.builder,
            trace: self.trace,
            phantom: self.phantom,
        }
    }
}

impl<P, V, F, Tag, Builder, Trace> Context<P, V, F, (), Tag, Builder, Trace>
where
    P: Position,
    P::Real: ExactReal,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_exact_predicates(self) -> Context<P, V, F, ExactPredicates<P>, Tag, Builder, Trace> {
        Context {
            predicates: ExactPredicates::new(),
            tag: self.tag,
            builder: self.builder,
            trace: self.trace,
            phantom: self.phantom,
        }
    }
}

impl<P, V, F, PR, Tag, Builder, Trace> PredicatesContext for Context<P, V, F, PR, Tag, Builder, Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
    PR: Predicates<Position = P>,
{
    type Predicates = PR;

    fn predicates(&self) -> &Self::Predicates {
        &self.predicates
    }
}

/// Trait to provide tagging information
pub trait TagContext {
    fn tag(&self) -> RefMut<usize>;
}

/// Store taging information
pub struct TagCtx(RefCell<usize>);

impl<P, V, F, Predicates, Builder, Trace> Context<P, V, F, Predicates, (), Builder, Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_tag(self) -> Context<P, V, F, Predicates, TagCtx, Builder, Trace> {
        Context {
            predicates: self.predicates,
            tag: TagCtx(RefCell::new(0)),
            builder: self.builder,
            trace: self.trace,
            phantom: self.phantom,
        }
    }
}

impl<P, V, F, Predicates, Builder, Trace> TagContext for Context<P, V, F, Predicates, TagCtx, Builder, Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn tag(&self) -> RefMut<usize> {
        self.tag.0.borrow_mut()
    }
}

/// Trait to provide temporaries for trienagulation building
pub trait BuilderContext {
    fn chain_store(&self) -> RefMut<ChainStore>;
}

/// Store temporaries for build
pub struct BuilderCtx(RefCell<ChainStore>);

impl<P, V, F, Predicates, Tag, Trace> Context<P, V, F, Predicates, Tag, (), Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_builder(self) -> Context<P, V, F, Predicates, Tag, BuilderCtx, Trace> {
        Context {
            predicates: self.predicates,
            tag: self.tag,
            builder: BuilderCtx(RefCell::new(ChainStore::new())),
            trace: self.trace,
            phantom: self.phantom,
        }
    }
}

impl<P, V, F, Predicates, Tag, Trace> BuilderContext for Context<P, V, F, Predicates, Tag, BuilderCtx, Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn chain_store(&self) -> RefMut<ChainStore> {
        self.builder.0.borrow_mut()
    }
}

/// Trait to provide tracing capabilities
pub trait TraceContext {
    fn trace_render(&self) -> RefMut<TraceRender>;
    fn trace_coloring(&self) -> Ref<Coloring>;
    fn trace_coloring_mut(&self) -> RefMut<Coloring>;
    fn trace_mapping(&self) -> Ref<TraceMapping>;
    fn trace_mapping_mut(&self) -> RefMut<TraceMapping>;
}

/// Store tracing helpers
pub struct TraceCtx<TP: TraceRender>(RefCell<TP>, RefCell<Coloring>, RefCell<TraceMapping>);

impl<P, V, F, Predicates, Tag, Builder> Context<P, V, F, Predicates, Tag, Builder, ()>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_trace<TP: TraceRender>(self, tracer: TP) -> Context<P, V, F, Predicates, Tag, Builder, TraceCtx<TP>> {
        Context {
            predicates: self.predicates,
            tag: self.tag,
            builder: self.builder,
            trace: TraceCtx(
                RefCell::new(tracer),
                RefCell::new(Coloring::new()),
                RefCell::new(TraceMapping::new()),
            ),
            phantom: self.phantom,
        }
    }
}

impl<P, V, F, Predicates, Tag, Builder, TP> TraceContext for Context<P, V, F, Predicates, Tag, Builder, TraceCtx<TP>>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
    TP: TraceRender,
{
    fn trace_render(&self) -> RefMut<TraceRender> {
        self.trace.0.borrow_mut()
    }

    fn trace_coloring(&self) -> Ref<Coloring> {
        self.trace.1.borrow()
    }

    fn trace_coloring_mut(&self) -> RefMut<Coloring> {
        self.trace.1.borrow_mut()
    }

    fn trace_mapping(&self) -> Ref<TraceMapping> {
        self.trace.2.borrow()
    }

    fn trace_mapping_mut(&self) -> RefMut<TraceMapping> {
        self.trace.2.borrow_mut()
    }
}
