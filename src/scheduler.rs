
use crate::world::World;


pub struct Scheduler {
    systems: Vec<Box<dyn Fn(World) -> ()>>
}

impl Scheduler {
    pub fn new() -> Self {
        Self {
            systems: Vec::new()
        }
    }

    pub fn add_system<T: Fn(World) -> () + 'static>(&mut self, system: T) -> &mut Self {
        self.systems.push(Box::new(system));
        self
    }

    
}