use std::{any::TypeId, collections::HashMap};

use crate::{
    component_storage::ComponentStorage,
    entity::Entity,
    entity_allocator::{AllocState, EntityAllocator},
    entity_builder::EntityBuilder,
    error::ECSError,
};

pub struct World {
    allocator: EntityAllocator,
    component_storages: HashMap<TypeId, ComponentStorage>,
}

impl World {
    pub fn new() -> Self {
        Self {
            allocator: EntityAllocator::new(),
            component_storages: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> EntityBuilder {
        let (alloc_state, entity) = self.allocator.allocate();

        match alloc_state {
            AllocState::Pushed => {
                for storage in self.component_storages.values_mut() {
                    storage.allocate();
                }
            }
            _ => (),
        }

        EntityBuilder::new(entity, self)
    }

    pub fn despawn(&mut self, entity: Entity) -> Result<(), ECSError> {
        self.allocator.deallocate(entity)?;

        for storage in self.component_storages.values_mut() {
            storage.remove(entity);
        }

        Ok(())
    }

    pub fn get_or_insert_storage<T: 'static>(&mut self) -> &mut ComponentStorage {
        let type_id = TypeId::of::<T>();

        match self.component_storages.get_mut(&type_id) {
            Some(storage) => return storage,
            None => {
                self.component_storages
                    .insert(type_id, ComponentStorage::new::<T>(self.allocator.len()));

                return self.component_storages.get_mut(&type_id).unwrap();
            }
        }
    }
}
