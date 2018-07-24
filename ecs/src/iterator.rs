use entity::Entity;
use hibitset::{BitIter, BitSetAnd, BitSetView};

pub trait Reference<'a> {
    type Item: 'static + Sync + Send;

    fn reference(&mut self, entity: Entity);
}

/// Trait to immutably reference a component of an entity
pub trait ComponentRef<'a>: Reference<'a> {
    fn get_unchecked(&self) -> &Self::Item;
    fn get(&self) -> Option<&Self::Item>;
}

/// Trait to mutably reference a component of an entity
pub trait ComponentRefMut<'a>: Reference<'a> {
    fn get_unchecked(&mut self) -> &mut Self::Item;

    fn get(&mut self) -> Option<&mut Self::Item>;

    fn set(&mut self, value: Self::Item);

    fn acquire(&mut self) -> &mut Self::Item
    where
        Self::Item: Default,
    {
        self.set(Default::default());
        self.get_unchecked()
    }
}

/// Trait to immutably reference a component of an entity
pub trait LinkRef<'a>: Reference<'a> {
    fn next(&mut self) -> Option<&Self::Item>;
}

/// Trait to mutably reference a component of an entity
pub trait LinkRefMut<'a>: Reference<'a> {
    fn next(&mut self) -> Option<&mut Self::Item>;
    fn set(&mut self, entity: Entity, value: Self::Item);

    fn acquire(&mut self, entity: Entity) -> &mut Self::Item
    where
        Self::Item: Default,
    {
        self.set(entity, Default::default());
        self.next().unwrap()
    }
}

/// Container with mask capability
pub trait MaskedContainer {
    type Item: 'static + Sync + Send;
    type Mask: BitSetView;
    type Ref: Reference<'static>;
    // TODO: GAT
    type RefMut: Reference<'static>; // TODO: GAT

    fn create_ref<'a>(&'a self) -> (&'a Self::Mask, Self::Ref);
    fn create_ref_mut<'a>(&'a mut self) -> (&'a Self::Mask, Self::RefMut);
}

pub struct RWJoin2<'a, A, B>
where
    A: 'a + MaskedContainer,
    B: 'a + MaskedContainer,
{
    iter: BitIter<BitSetAnd<&'a A::Mask, &'a B::Mask>>,
    a: A::Ref,
    b: B::RefMut,
}

impl<'a, A, B> RWJoin2<'a, A, B>
where
    A: 'a + MaskedContainer,
    B: 'a + MaskedContainer,
{
    pub fn new<'b>(a: &'b A, b: &'b mut B) -> RWJoin2<'b, A, B> {
        let a = a.create_ref();
        let b = b.create_ref_mut();
        RWJoin2 {
            iter: BitSetAnd(a.0, b.0).iter(),
            a: a.1,
            b: b.1,
        }
    }

    pub fn next(&mut self) -> Option<(Entity, &A::Ref, &mut B::RefMut)> {
        match self.iter.next() {
            Some(id) => {
                let entity = Entity::from_id(id);
                self.a.reference(entity);
                self.b.reference(entity);
                Some((entity, &self.a, &mut self.b))
            }
            None => None,
        }
    }
}

macro_rules! define_join {
(__inner_and_type $ a: ty, ) => {& 'a $ a};
(__inner_and_type $ a: ty, $ ($ b: ty, ) * ) => {BitSetAnd < & 'a $ a, define_join ! (__inner_and_type $ ( $ b, ) * ) >};

(__inner_and $ a: ident, ) => {$ a.0};
(__inner_and $ a: ident, $ ($ b: ident, ) * ) => {BitSetAnd( $ a.0, define_join ! (__inner_and $ ( $ b, ) * ))};

( $ join: ident => impl < ($ ( $ arg: ident), * )> ) => {define_join ! ( $ join => impl < ( $( $ arg), * ), mut () > );};
( $ join: ident => impl < mut ( $ ( $ arg_mut: ident), *) > ) => {define_join ! ( $ join => impl < (), mut ( $ ( $ arg_mut), *) > );};

( $ join: ident => impl < ($ ( $ arg: ident), * ), mut ( $ ( $ arg_mut: ident), * ) > ) => {
# [allow(non_snake_case)]
//#[allow(dead_code)]
pub struct $join < 'a, $ ( $ arg, )* $ ( $ arg_mut, ) * >
where
$ ( $ arg: 'a + MaskedContainer, )*
$ ( $ arg_mut: 'a + MaskedContainer,) *
{
iter: BitIter < define_join ! ( __inner_and_type $ ($ arg::Mask, ) * $ ( $arg_mut::Mask, ) * ) >,
$( $ arg: & 'a $ arg::Store, ) *
$ ( $ arg_mut: & 'a mut $ arg_mut::Store, ) *
}

impl < 'a, $ ( $ arg, )* $ ( $ arg_mut, ) * > $join < 'a, $ ( $ arg, )* $ ( $ arg_mut, ) * >
where
$ ( $ arg: 'a + MaskedContainer, )*
$ ( $ arg_mut: 'a + MaskedContainer,) *
{
# [allow(non_snake_case)]
pub fn new< 'b > ( $ ( $ arg: &'b $ arg, ) * $ ( $ arg_mut : & 'b mut $ arg_mut, ) * ) -> $ join < 'b, $ ( $ arg, ) * $ ( $ arg_mut, ) *> {
$ ( let $ arg = $ arg.store(); ) *
$ ( let $ arg_mut = $ arg_mut.store_mut(); ) *
$ join {
iter: define_join ! ( __inner_and $ ( $ arg, ) * $ ( $ arg_mut, ) * ).iter(),
$ ( $ arg: $ arg.1, ) *
$ ($ arg_mut: $ arg_mut.1, ) *
}
}

pub fn next < 'b > ( & 'b mut self ) -> Option < Entity >{
match self.iter.next() {
Some(id) => Some(Entity::from_id(id)),
None => None
}
}
}
};
}

/*
define_join! { RRJoin => impl<(S0, S1)> }
define_join! { RWJoin => impl<(S0), mut(S1)> }
define_join! { WWJoin => impl<mut(S0, S1)> }

define_join! { RRRJoin => impl<(S0, S1, S2)> }
define_join! { RRWJoin => impl<(S0, S1), mut(S2)> }
define_join! { RWWJoin => impl<(S0), mut(S1, S2)> }
define_join! { WWWJoin => impl<mut(S0, S1, S2)> }*/
