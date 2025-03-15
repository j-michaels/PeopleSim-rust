mod world;
mod person;
mod hobbies;

use std::io::stdout;
use hobbies::HOBBIES;
use rand::prelude::*;

use person::Person;
use world::World;

fn main() {
    let stdout = stdout();

    let mut world = World::new(20, 20, 10);

    world.iterate(8);
}