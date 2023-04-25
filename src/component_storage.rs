use crate::{entity::Entity, error::ECSError};
use any_vec::AnyVec;

#[derive(PartialEq, Eq, Clone, Copy)]

struct SparseEntity {
    index: usize,
    generation: i16,
}

impl SparseEntity {
    pub fn index(&self) -> usize {
        self.index
    }

    pub fn generation(&self) -> i16 {
        self.generation
    }
}

pub(crate) struct ComponentStorage {
    components: AnyVec,
    entities: Vec<Entity>,
    sparse: Vec<Option<SparseEntity>>,
}

impl ComponentStorage {
    pub fn new<T: 'static>(entity_count: usize) -> Self {
        Self {
            components: AnyVec::new::<T>(),
            entities: Vec::new(),
            sparse: vec![None; entity_count],
        }
    }

    pub fn insert<T: 'static>(&mut self, entity: Entity, component: T) {
        let mut comps_vec = self.components.downcast_mut::<T>().unwrap();
        comps_vec.push(component);
        self.entities.push(entity);

        self.sparse[entity.index()] = Some(SparseEntity {
            index: comps_vec.len() - 1,
            generation: entity.generation(),
        })
    }

    // Returns a boolean of whether the given entity is in the Sparse Array, and the entity is of the correct generation.
    pub fn has(&self, entity: Entity) -> bool {
        self.sparse[entity.index()]
            .filter(|e| e.generation == entity.generation())
            .is_some()
    }

    // Pushes a new entity into the sparse array
    pub fn allocate(&mut self) {
        self.sparse.push(None)
    }

    pub fn remove(&mut self, entity: Entity) -> Result<(), ECSError> {
        if !self.has(entity) {
            return Err(ECSError::ComponentNotFound);
        }

        let sparse_entity_index = self.sparse[entity.index()].unwrap().index();

        self.components.swap_remove(sparse_entity_index);
        self.entities.swap_remove(sparse_entity_index);

        self.sparse[entity.index()] = None;

        match self.entities.last() {
            Some(last) => {
                self.sparse[last.index()] = Some(SparseEntity {
                    index: sparse_entity_index,
                    generation: last.generation(),
                });
            }
            _ => (),
        }

        Ok(())
    }
}
