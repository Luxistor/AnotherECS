
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

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
    resources: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            allocator: EntityAllocator::new(),
            component_storages: HashMap::new(),
            resources: HashMap::new(),
        }
    }

    pub fn insert_resource<T: 'static>(&mut self, resource: T) {
        self.resources.insert(TypeId::of::<T>(), Box::new(resource));
    }


    pub fn get_resource<T: 'static>(&self) -> &T {
        // SAFETY: 
        // Resources are a map of TypeId of T -> Box<T>. If there is an entry, it is of type T
        unsafe { 
            self.resources
            .get(&TypeId::of::<T>())
            .expect("Resource not found in world!")
            .downcast_ref_unchecked::<T>() 
        }
    }

    pub fn get_resource_mut<T: 'static>(&mut self) -> &mut T {
        // SAFETY: Analogous to World::get_resource
        unsafe { 
            self.resources
            .get_mut(&TypeId::of::<T>())
            .expect("Resource not found in world!")
            .downcast_mut_unchecked::<T>() 
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
            let _ = storage.remove(entity);
        }

        Ok(())
    }

    pub(crate) fn get_or_insert_storage<T: 'static>(&mut self) -> &mut ComponentStorage {
        self.component_storages
            .entry(TypeId::of::<T>())
            .or_insert(ComponentStorage::new::<T>(self.allocator.len()))
    }
}
