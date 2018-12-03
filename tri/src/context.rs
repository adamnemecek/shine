use checker::{Coloring, TraceMapping, TraceRender};
use geometry::{Position, Predicates};
use graph::{Face, Vertex};
use std::cell::{Ref, RefCell, RefMut};
use triangulation::Triangulation;
use vertexchain::ChainStore;

/// Context the enables/disables triangulation features and also stores the required datas
pub struct Context<Predicates = (), Tag = (), Builder = (), Trace = ()> {
    predicates: Predicates,
    tag: Tag,
    builder: Builder,
    trace: Trace,
}

impl Context<(), (), (), ()> {
    pub fn new() -> Context {
        Context {
            predicates: (),
            tag: (),
            builder: (),
            trace: (),
        }
    }
}

impl<Predicates, Tag, Builder, Trace> Context<Predicates, Tag, Builder, Trace> {
    pub fn create<P, V, F>(self) -> Triangulation<P, V, F, Self>
    where
        P: Position,
        V: Vertex<Position = P>,
        F: Face,
    {
        Triangulation::new(self)
    }
}

/// Trait to provide geometry predicates
pub trait PredicatesContext {
    type Predicates: Predicates;

    fn predicates(&self) -> &Self::Predicates;
}

impl<Tag, Builder, Trace> Context<(), Tag, Builder, Trace> {
    pub fn with_predicates<PredicateCtx: Predicates>(
        self,
        predicates: PredicateCtx,
    ) -> Context<PredicateCtx, Tag, Builder, Trace> {
        Context {
            predicates,
            tag: self.tag,
            builder: self.builder,
            trace: self.trace,
        }
    }
}

impl<PredicateCtx, Tag, Builder, Trace> PredicatesContext for Context<PredicateCtx, Tag, Builder, Trace>
where
    PredicateCtx: Predicates,
{
    type Predicates = PredicateCtx;

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

impl<Predicates, Builder, Trace> Context<Predicates, (), Builder, Trace> {
    pub fn with_tag(self) -> Context<Predicates, TagCtx, Builder, Trace> {
        Context {
            predicates: self.predicates,
            tag: TagCtx(RefCell::new(0)),
            builder: self.builder,
            trace: self.trace,
        }
    }
}

impl<Predicates, Builder, Trace> TagContext for Context<Predicates, TagCtx, Builder, Trace> {
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

impl<Predicates, Tag, Trace> Context<Predicates, Tag, (), Trace> {
    pub fn with_builder(self) -> Context<Predicates, Tag, BuilderCtx, Trace> {
        Context {
            predicates: self.predicates,
            tag: self.tag,
            builder: BuilderCtx(RefCell::new(ChainStore::new())),
            trace: self.trace,
        }
    }
}

impl<Predicates, Tag, Trace> BuilderContext for Context<Predicates, Tag, BuilderCtx, Trace> {
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

impl<Predicates, Tag, Builder> Context<Predicates, Tag, Builder, ()> {
    pub fn with_trace<TP: TraceRender>(self, tracer: TP) -> Context<Predicates, Tag, Builder, TraceCtx<TP>> {
        Context {
            predicates: self.predicates,
            tag: self.tag,
            builder: self.builder,
            trace: TraceCtx(
                RefCell::new(tracer),
                RefCell::new(Coloring::new()),
                RefCell::new(TraceMapping::new()),
            ),
        }
    }
}

impl<Predicates, Tag, Builder, TP> TraceContext for Context<Predicates, Tag, Builder, TraceCtx<TP>>
where
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
