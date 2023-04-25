use world::World;

mod entity;
mod entity_allocator;
mod component_storage;
mod world;
mod entity_builder;
mod error;
mod component;



fn physics(world: &mut World){

}

fn render(world: &mut World){

}

fn wfc(world: &mut World, ){
    
}

struct Position(f32);
struct Velocity(f32);


fn main() {
    let mut world = crate::world::World::new();

    let test = world.spawn().with(Position(5.0)).with(Velocity(5.0)).build();
    
    loop {
        physics(&mut world)
        render(&mut world)
        wfc(&mut world)
    }
}
