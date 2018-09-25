use entity::Entity;
use graph::join;
use graph::traits::{IndexExcl, IndexLowerBound};

/// Iterator like trait that performs the merge.
pub struct Join<S>
where
    S: IndexLowerBound<usize>,
{
    inner: join::Join<S>,
}

impl<S> Join<S>
where
    S: IndexLowerBound<usize>,
{
    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> Option<(Entity, <S as IndexExcl<usize>>::Item)> {
        self.inner.next().map(|(id, e)| (Entity::from_id(id), e))
    }

    pub fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(Entity, <S as IndexExcl<usize>>::Item),
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    pub fn until<F>(&mut self, mut f: F)
    where
        F: FnMut(Entity, <S as IndexExcl<usize>>::Item) -> bool,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}

/// Trait to create Join
pub trait IntoJoin {
    type Store: IndexLowerBound<usize>;

    fn into_join(self) -> Join<<Self as IntoJoin>::Store>;
}

impl<T> IntoJoin for T
where
    T: join::IntoJoin,
{
    type Store = <T as join::IntoJoin>::Store;

    fn into_join(self) -> Join<<Self as IntoJoin>::Store>
    where
        Self: Sized,
    {
        Join {
            inner: <Self as join::IntoJoin>::into_join(self),
        }
    }
}

pub trait IntoJoinExt: IntoJoin {
    fn join_all<F>(self, f: F)
    where
        F: FnMut(Entity, <Self::Store as IndexExcl<usize>>::Item),
        Self: Sized,
    {
        self.into_join().for_each(f);
    }

    fn join_until<F>(self, f: F)
    where
        F: FnMut(Entity, <Self::Store as IndexExcl<usize>>::Item) -> bool,
        Self: Sized,
    {
        self.into_join().until(f);
    }
}

impl<T: ?Sized> IntoJoinExt for T where T: IntoJoin {}
