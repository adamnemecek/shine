use geometry::Predicates;
use std::cell::{RefCell, RefMut};
use vertexchain::ChainStore;

/// Context the enables/disables triangulation features and also stores the required datas
pub struct Context<Predicates = (), Tag = (), Builder = ()> {
    predicates: Predicates,
    tag: Tag,
    builder: Builder,
}

impl Context<(), (), ()> {
    fn new() -> Context<(), ()> {
        Context {
            predicates: (),
            tag: (),
            builder: (),
        }
    }
}

/// Trait to provide geometry predicates
pub trait PredicatesContext {
    type Predicates: Predicates;

    fn predicates(&self) -> &Self::Predicates;
}

impl<Tag, Builder> Context<(), Tag, Builder> {
    fn with_predicates<PR: Predicates>(self, predicates: PR) -> Context<PR, Tag, Builder> {
        Context {
            predicates,
            tag: self.tag,
            builder: self.builder,
        }
    }
}

impl<PR, Tag, Builder> PredicatesContext for Context<PR, Tag, Builder>
where
    PR: Predicates,
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

impl<Predicates, Builder> Context<Predicates, (), Builder> {
    fn with_tag(self) -> Context<Predicates, TagCtx, Builder> {
        Context {
            predicates: self.predicates,
            tag: TagCtx(RefCell::new(0)),
            builder: self.builder,
        }
    }
}

impl<Predicates, Builder> TagContext for Context<Predicates, TagCtx, Builder> {
    fn tag(&self) -> RefMut<usize> {
        self.tag.tag()
    }
}

/// Trait to provide temporaries for trienagulation building
pub trait BuilderContext {
    fn chain_store(&self) -> RefMut<ChainStore>;
}

/// Store temporaries for build
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

impl<Predicates, Tag> Context<Predicates, Tag, ()> {
    fn with_builder(self) -> Context<Predicates, Tag, BuilderCtx> {
        Context {
            predicates: self.predicates,
            tag: self.tag,
            builder: BuilderCtx(RefCell::new(ChainStore::new())),
        }
    }
}

impl<Predicates, Tag> BuilderContext for Context<Predicates, Tag, BuilderCtx> {
    fn chain_store(&self) -> RefMut<ChainStore> {
        self.builder.chain_store()
    }
}
