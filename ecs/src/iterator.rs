use std::ops;
use hibitset::{BitSetLike, BitSetAnd/*, BitIter*/};
use entity::Entity;


/// Container that can immutably access component by Entity
pub trait IndexedContainer: ops::Index<Entity> {
    fn try_index(&self, idx: Entity) -> Option<&Self::Output>;
}

/// Container that can mutably access component by Entity
pub trait IndexedMutContainer: IndexedContainer + ops::IndexMut<Entity> {
    fn try_index_mut(&mut self, idx: Entity) -> Option<&mut Self::Output>;
}

/// Container with mask capability
pub trait MaskedContainer {
    type Mask: BitSetLike;

    fn mask(&self) -> &Self::Mask;
}

/// Trait to iterate over the entities of a container
pub trait EntityIterator {
    fn next_entity(&mut self) -> Option<Entity>;
}


/// Trait to iterate over components.
pub trait RIterator: EntityIterator {
    type Item0;

    fn next(&mut self) -> Option<(Entity, &Self::Item0)>;
}

/// Trait to iterate over components.
pub trait WIterator: EntityIterator {
    type Item0;

    fn next(&mut self) -> Option<(Entity, &mut Self::Item0)>;
}

/// Trait to iterate over components.
pub trait RRIterator: EntityIterator {
    type Item0;
    type Item1;

    fn next(&mut self) -> Option<(Entity, &Self::Item0, &Self::Item1)>;
}

/// Trait to iterate over components.
pub trait RWIterator: EntityIterator {
    type Item0;
    type Item1;

    fn next(&mut self) -> Option<(Entity, &Self::Item0, &mut Self::Item1)>;
}

/// Trait to iterate over components.
pub trait WWIterator: EntityIterator {
    type Item0;
    type Item1;

    fn next(&mut self) -> Option<(Entity, &mut Self::Item0, &mut Self::Item1)>;
}


pub struct RWJoin<'a, I0, I1>
    where
        I0: 'a + IndexedContainer + MaskedContainer,
        I1: 'a + IndexedMutContainer + MaskedContainer
{
    mask: BitSetAnd<&'a I0::Mask, &'a I1::Mask>,
    i0: *const I0,
    i1: *mut I1,
}

impl<'a, I0, I1> RWJoin<'a, I0, I1>
    where
        I0: 'a + IndexedContainer + MaskedContainer,
        I1: 'a + IndexedMutContainer + MaskedContainer
{
    fn new<'b>(i0: &'b I0, i1: &'b mut I1) -> RWJoin<'b, I0, I1> {
        let pi0 = i0 as *const I0;
        let pi1 = i1 as *mut I1;
        RWJoin {
            mask: BitSetAnd(i0.mask(), i1.mask()),
            i0: pi0,
            i1: pi1,
        }
    }
}

impl<'a, I0, I1> MaskedContainer for RWJoin<'a, I0, I1>
    where
        I0: 'a + IndexedContainer + MaskedContainer,
        I1: 'a + IndexedMutContainer + MaskedContainer
{
    type Mask = BitSetAnd<&'a I0::Mask, &'a I1::Mask>;

    fn mask(&self) -> &Self::Mask {
        &self.mask
    }
}
