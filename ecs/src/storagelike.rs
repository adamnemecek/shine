use entity::Entity;

pub trait StorageLike {
    type Item;

    fn get(&mut self, entity: Entity) -> Self::Item;

    fn next_entity(&mut self) -> Option<Entity>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(entity) = self.next_entity() {
            Some(self.get(entity))
        } else {
            None
        }
    }
}
/*
impl<T: StorageLike> Iterator for T
{
    type Item = <Self as StorageLike>::Item;

    fn next(&mut self) -> Option<Self::Item> {
        StorageLike::next(self)
    }
}*/
