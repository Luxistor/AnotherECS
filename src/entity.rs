

use std::cmp::Eq;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub struct Entity {
    generation: i16,
    index: usize,
}

impl Entity {
    pub fn new(generation: i16, index: usize) -> Entity {
        Self { generation, index}
    }

    pub fn index(&self) -> usize {
        self.index
    }

    pub fn generation(&self) -> i16 {
        self.generation
    }
}

