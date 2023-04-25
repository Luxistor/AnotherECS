use crate::world::World;
use crate::entity::Entity;

#[must_use = "EntityBuilder must finish building"]
pub struct EntityBuilder<'world> {
    entity: Entity,
    world: &'world mut World 

}

impl<'world> EntityBuilder<'world> {
    pub fn new(entity: Entity, world: &'world mut World) -> Self {
        Self {
            entity,
            world
        }
    }

    pub fn with<T: 'static>(self, component: T) -> Self {
        let storage = self.world.get_or_insert_storage::<T>();
        storage.insert(self.entity, component);

        self
    }

    pub fn build(self) -> Entity {
        self.entity
    }
}
