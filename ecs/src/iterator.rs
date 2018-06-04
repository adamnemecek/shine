use hibitset::{BitSetLike, BitSetAnd, BitIter};

use entity::Entity;
use edge::Edge;

/// Trait to access component by entity.
pub trait ComponentMap {
    type Item: 'static + Sync + Send;

    unsafe fn get_unchecked(&self, entity: Entity) -> &Self::Item;
    unsafe fn get_unchecked_mut(&mut self, entity: Entity) -> &mut Self::Item;

    fn get(&self, entity: Entity) -> Option<&Self::Item>;
    fn get_mut(&mut self, entity: Entity) -> Option<&mut Self::Item>;
}


/// Trait to access link by edge.
pub trait LinkMap {
    type Item: 'static + Sync + Send;

    unsafe fn get_unchecked(&self, edge: Edge) -> &Self::Item;
    unsafe fn get_unchecked_mut(&mut self, edge: Edge) -> &mut Self::Item;

    fn get(&self, edge: Edge) -> Option<&Self::Item>;
    fn get_mut(&mut self, edge: Edge) -> Option<&mut Self::Item>;
}


/// Container with mask capability
pub trait MaskedContainer {
    type Mask: BitSetLike;
    type Store: ComponentMap;

    fn store(&self) -> (&Self::Mask, &Self::Store);
    fn store_mut(&mut self) -> (&Self::Mask, &mut Self::Store);
}


macro_rules! define_join {
    (__inner and_type($a:ty, $b:ty)) => {BitSetAnd<$a,$b>}
    (__inner and($a:ident, $b:ident)) => {BitSetAnd($a,$b)}

    (__inner and_type($a:ty, $b:ty, $c:ty)) => {BitSetAnd<$a,BitSetAnd<$b,$c>>}
    (__inner and($a:ident, $b:ident, $c:ident)) => {BitSetAnd($a,BitSetAnd($b,$c))}

    ($join:ident => impl<($($arg:ident),*), mut ($($arg_mut:ident),*)>) => {
        pub struct $join<'a, $($arg,)* $($arg_mut,)*>
            where
                $($arg : 'a + MaskedContainer,)*
                $($arg_mut : 'a + MaskedContainer,)*
        {
            //iter: BitIter<BitSetAnd<&'a S0::Mask, BitSetAnd<&'a S1::Mask, &'a S2::Mask>>>,
            $($arg : &'a $arg::Store,)*
            $($arg_mut : &'a mut $arg_mut::Store,)*
        }

        impl<'a, $($arg,)* $($arg_mut,)*> $join<'a, $($arg,)* $($arg_mut,)*>
            where
                $($arg : 'a + MaskedContainer,)*
                $($arg_mut : 'a + MaskedContainer,)*
        {
            pub fn new<'b>($($arg : &'b $arg,)* $($arg_mut : &'b mut $arg_mut,)*) -> $join<'b, $($arg,)* $($arg_mut,)*> {
                $(let $arg = $arg.store();)*
                $(let $arg_mut = $arg_mut.store_mut();)*
                $join {
                    //iter: BitSetAnd(m0, BitSetAnd(m1, m2)).iter(),
                    $($arg: $arg.1,)*
                    $($arg_mut: $arg_mut.1,)*
                }
            }
        }
    };
}

define_join! { RWWJoin2 => impl<(S1, S2), mut(S3)> }

/// Join component stores
pub struct RWWJoin<'a, S0, S1, S2>
    where
        S0: 'a + MaskedContainer,
        S1: 'a + MaskedContainer,
        S2: 'a + MaskedContainer,
{
    iter: BitIter<BitSetAnd<&'a S0::Mask, BitSetAnd<&'a S1::Mask, &'a S2::Mask>>>,
    store0: &'a S0::Store,
    store1: &'a mut S1::Store,
    store2: &'a mut S2::Store,
}

impl<'a, S0, S1, S2> RWWJoin<'a, S0, S1, S2>
    where
        S0: 'a + MaskedContainer,
        S1: 'a + MaskedContainer,
        S2: 'a + MaskedContainer,
{
    pub fn new<'b>(store0: &'b S0, store1: &'b mut S1, store2: &'b mut S2) -> RWWJoin<'b, S0, S1, S2> {
        let (m0, s0) = store0.store();
        let (m1, s1) = store1.store_mut();
        let (m2, s2) = store2.store_mut();
        RWWJoin {
            iter: BitSetAnd(m0, BitSetAnd(m1, m2)).iter(),
            store0: s0,
            store1: s1,
            store2: s2,
        }
    }

    pub fn next<'b>(&'b mut self) -> Option<(Entity, &'b <S0::Store as ComponentMap>::Item, &'b mut <S1::Store as ComponentMap>::Item, &'b mut <S2::Store as ComponentMap>::Item)> {
        match self.iter.next() {
            Some(id) => {
                let entity = Entity::from_id(id);
                Some((entity,
                      unsafe { self.store0.get_unchecked(entity) },
                      unsafe { self.store1.get_unchecked_mut(entity) },
                      unsafe { self.store2.get_unchecked_mut(entity) }))
            }
            None => None,
        }
    }
}

