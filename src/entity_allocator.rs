use crate::{entity::Entity, error::ECSError};

pub(crate) enum AllocState {
    Pushed,
    Filled,
}

#[derive(Clone, Copy)]
struct Generation(i16);

impl Generation {
    pub fn kill(self) -> Generation {
        Generation(-self.0)
    }

    pub fn raise(self) -> Generation {
        Generation(1 - self.0)
    }

    pub fn is_alive(&self) -> bool {
        self.0 >= 0
    }
}

pub(crate) struct EntityAllocator {
    generations: Vec<Generation>,
    free: Vec<usize>,
}

impl EntityAllocator {
    pub fn new() -> Self {
        Self {
            generations: Vec::new(),
            free: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.generations.len()
    }

    pub fn allocate(&mut self) -> (AllocState, Entity) {
        let index = self.free.pop();

        match index {
            Some(index) => {
                let generation = self.generations[index].raise();
                self.generations[index] = generation;

                return (AllocState::Filled, Entity::new(generation.0, index));
            }
            None => {
                self.generations.push(Generation(0));
                return (AllocState::Pushed, Entity::new(0, self.generations.len() - 1));
            }
        }
    }

    pub fn deallocate(&mut self, entity: Entity) -> Result<(), ECSError> {
        let index = entity.index();

        match self.generations.get(index) {
            Some(generation) => {
                if !generation.is_alive() || generation.0 != entity.generation() {
                    return Err(ECSError::UseAfterFree);
                }

                self.generations[index] = generation.kill();

                Ok(())
            }
            None => return Err(ECSError::EntityNotFound),
        }
    }
}
