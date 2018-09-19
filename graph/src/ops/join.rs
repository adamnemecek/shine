pub struct MaskedTag;
pub struct MergedTag;

/// Iterator like behavior for join operations.
pub trait Join {
    type Item;

    #[allow(clippy::should_implement_trait)]
    fn next(&mut self) -> Option<(usize, Self::Item)>;
}

/// Extension methods to preform common iteration tasks.
pub trait JoinExt: Join {
    fn for_each<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, Self::Item),
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            f(id, e);
        }
    }

    fn until<F>(&mut self, mut f: F)
    where
        F: FnMut(usize, Self::Item) -> bool,
        Self: Sized,
    {
        while let Some((id, e)) = self.next() {
            if !f(id, e) {
                break;
            }
        }
    }
}
impl<T: ?Sized> JoinExt for T where T: Join {}

/*
/// Helper to select the most appropriate merging method based on the Tag
pub trait IntoJoin {
    fn into_join<Tag>(self) -> impl Join;
}

/// Extension method simplify the most common join operations
pub trait IntoJoinExt: IntoJoin {
    fn join_all<Tag, F>(self, f: F)
    where
        F: FnMut(usize, <Self::Join as Join>::Item),
        Self: Sized,
    {
        self.into_join::<Tag>().for_each(f);
    }

    fn join_until<Tag, F>(self, f: F)
    where
        F: FnMut(usize, <Self::Join as Join>::Item) -> bool,
        Self: Sized,
    {
        self.into_join::<Tag>().until(f);
    }
}
impl<Tag, T: ?Sized> IntoJoinExt for T where T: IntoJoin {}*/
