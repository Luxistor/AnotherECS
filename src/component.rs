use crate::{world::World, entity::Entity};

trait ComponentTuple {
    fn insert(&self, world: &mut World, entity: Entity);
} 