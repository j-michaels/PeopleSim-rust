use grid::*;
use ferris_says::say;
use std::io::{stdout, BufWriter};
use rand::prelude::*;

struct Point {
    x: i32,
    y: i32,
}

struct Person {
    easygoingness: i32,
    personality: i32,
    id: i32,
    hobbies: Vec<String>,
    friends: Vec<Box<Person>>,
    coords: Point,
    partying: bool,
}

fn make_world(width: i16, height: i16) -> Vec<Vec<Vec<Person>>> {
    let mut world = Vec::new();

    for _i in 0..width {
        let mut row = Vec::new();

        for _j in 0..height {
            row.append(&mut Vec::new());
        }
        world.append(&mut row);
    }

    return world;
}

fn make_person(id: i32, hobby: String) -> Person {
    let easygoingness: i32 = rand::random();
    let personality: i32 = rand::random();

    return Person {
        easygoingness: easygoingness % 10,
        personality: personality % 10,
        id,
        hobbies: Vec::new(),
        friends: Vec::new(),
        coords: Point { x: 0, y: 0 },
        partying: false,
    }
}

fn main() {
    let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());

    // say(&message, width, &mut writer).unwrap();

    let world = make_world(20, 20);


}


