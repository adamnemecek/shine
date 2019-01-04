use geometry2::{ExactPredicates, ExactReal, InexactPredicates, InexactReal, Position, Predicates};
use std::cell::RefCell;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use triangulation::check::{TraceControl, TraceRender};
use triangulation::graph::{Face, Triangulation, Vertex};
use triangulation::types::{FaceEdge, FaceVertex};

/// Context the enables/disables triangulation features and also stores the required datas
pub struct Context<P, V, F, Predicates = (), Tag = (), Builder = (), Trace = ()>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub predicates: Predicates,
    pub tag: Tag,
    pub builder: Builder,
    pub trace: Trace,
    pub phantom: PhantomData<(P, V, F)>,
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

impl<P, V, F> Default for Context<P, V, F, (), (), (), ()>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    fn default() -> Context<P, V, F> {
        Context::new()
    }
}

impl<P, V, F> Context<P, V, F, (), (), (), ()>
where
    P: Position,
    P::Real: InexactReal,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn new_inexact_common() -> Context<P, V, F, InexactPredicates<P>, TagCtx, BuilderCtx, ()> {
        Context::<P, V, F>::new().with_inexact_predicates().with_tag().with_builder()
    }
}

impl<P, V, F> Context<P, V, F, (), (), (), ()>
where
    P: Position,
    P::Real: ExactReal,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn new_exact_common() -> Context<P, V, F, ExactPredicates<P>, TagCtx, BuilderCtx, ()> {
        Context::<P, V, F>::new().with_exact_predicates().with_tag().with_builder()
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
    fn tag(&self) -> Rc<RefCell<usize>>;
}

/// Store taging information
#[derive(Default)]
pub struct TagCtx {
    pub value: Rc<RefCell<usize>>,
}

impl<P, V, F, Predicates, Builder, Trace> Context<P, V, F, Predicates, (), Builder, Trace>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_tag(self) -> Context<P, V, F, Predicates, TagCtx, Builder, Trace> {
        Context {
            predicates: self.predicates,
            tag: Default::default(),
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
    fn tag(&self) -> Rc<RefCell<usize>> {
        self.tag.value.clone()
    }
}

/// Trait to provide temporaries for trienagulation building
pub trait BuilderContext {
    fn get_face_vertex_vector(&self, name: &str) -> Rc<RefCell<Vec<FaceVertex>>>;
    fn get_face_edge_vector(&self, name: &str) -> Rc<RefCell<Vec<FaceEdge>>>;
}

#[derive(Default)]
pub struct BuilderCache {
    face_vertex_vector: HashMap<String, Rc<RefCell<Vec<FaceVertex>>>>,
    face_edge_vector: HashMap<String, Rc<RefCell<Vec<FaceEdge>>>>,
}

/// Store temporaries for build
#[derive(Default)]
pub struct BuilderCtx {
    pub cache: RefCell<BuilderCache>,
}

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
            builder: Default::default(),
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
    fn get_face_vertex_vector(&self, name: &str) -> Rc<RefCell<Vec<FaceVertex>>> {
        let mut cache = self.builder.cache.borrow_mut();
        let cache = &mut cache.face_vertex_vector;
        if let Some(entry) = cache.get(name) {
            return entry.clone();
        }

        let entry = Rc::new(RefCell::new(Vec::new()));
        cache.insert(name.to_string(), entry.clone());
        entry
    }

    fn get_face_edge_vector(&self, name: &str) -> Rc<RefCell<Vec<FaceEdge>>> {
        let mut cache = self.builder.cache.borrow_mut();
        let cache = &mut cache.face_edge_vector;
        if let Some(entry) = cache.get(name) {
            return entry.clone();
        }

        let entry = Rc::new(RefCell::new(Vec::new()));
        cache.insert(name.to_string(), entry.clone());
        entry
    }
}

/// Trait to provide tracing capabilities
pub trait TraceContext {
    fn trace_render(&self) -> Rc<RefCell<TraceRender>>;
    fn trace_control(&self) -> Rc<RefCell<TraceControl>>;
}

/// Store tracing helpers
pub struct TraceCtx<TC: TraceControl, TR: TraceRender> {
    pub control: Rc<RefCell<TC>>,
    pub render: Rc<RefCell<TR>>,
}

impl<P, V, F, Predicates, Tag, Builder> Context<P, V, F, Predicates, Tag, Builder, ()>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
{
    pub fn with_trace<TC, TR, T>(self, trace: T) -> Context<P, V, F, Predicates, Tag, Builder, TraceCtx<TC, TR>>
    where
        TC: TraceControl,
        TR: TraceRender,
        T: Into<TraceCtx<TC, TR>>,
    {
        Context {
            predicates: self.predicates,
            tag: self.tag,
            builder: self.builder,
            trace: trace.into(),
            phantom: self.phantom,
        }
    }
}

impl<P, V, F, Predicates, Tag, Builder, TC, TR> TraceContext for Context<P, V, F, Predicates, Tag, Builder, TraceCtx<TC, TR>>
where
    P: Position,
    V: Vertex<Position = P>,
    F: Face,
    TC: 'static + TraceControl,
    TR: 'static + TraceRender,
{
    fn trace_render(&self) -> Rc<RefCell<TraceRender>> {
        self.trace.render.clone()
    }

    fn trace_control(&self) -> Rc<RefCell<TraceControl>> {
        self.trace.control.clone()
    }
}
