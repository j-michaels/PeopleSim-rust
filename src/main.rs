use std::{io::stdout, vec};
use rand::prelude::*;

const HOBBIES: [&'static str; 8] = [
    "Knitting",
    "Baking Cookies",
    "Playing Bingo",
    "Gardening",
    "Arranging Furniture",
    "Dusting",
    "Flying Kites",
    "Playing Scrabble"
];

struct Person {
    easygoingness: i32,
    personality: i32,
    id: i32,
    hobbies: Vec<String>,
    friends: Vec<Box<Person>>,
    coords: Point,
    partying: bool,
}

struct Point {
    x: usize,
    y: usize,
}
type World<T> = Vec<Vec<T>>;
type Location = Vec<Box<Person>>;



fn make_world(width: i16, height: i16, n_people: i16) -> World<Location> {
    let mut world = Vec::new();

    for _i in 0..width {
        let mut row = Vec::new();

        for _j in 0..height {
            let mut location = Vec::new();

            row.append(location);
        }
        world.append(&mut row);
    }

    for i in 0..n_people {

    }

    return world;
}

fn is_compatible(person_a: &Person, person_b: &Person) -> bool {
    return (person_a.personality <= (person_b.personality + person_a.easygoingness))
        && (person_a.personality >= (person_b.personality - person_a.easygoingness))
        && (person_b.personality <= (person_a.personality + person_b.easygoingness))
        && (person_b.personality >= (person_a.personality - person_b.easygoingness));
}

fn in_bounds(world: &World<Location>, coords: &Point) -> bool {
    return coords.x >= 0 && coords.x <= world.len()
        && coords.y >= 0 && coords.y <= world.get(coords.x).unwrap().len()
}

fn find_adjacent_points(world: &World<Location>, coords: Point, allow_diagonal: bool) -> Vec<Point> {
    let mut adjacent_points = vec![
        Point { x: coords.x - 1, y: coords.y },
        Point { x: coords.x + 1, y: coords.y },
        Point { x: coords.x, y: coords.y - 1 },
        Point { x: coords.x, y: coords.y + 1 },
    ];

    if allow_diagonal {
        adjacent_points.extend(vec![
            Point { x: coords.x - 1, y: coords.y - 1},
            Point { x: coords.x - 1, y: coords.y + 1},
            Point { x: coords.x + 1, y: coords.y - 1},
            Point { x: coords.x + 1, y: coords.y + 1},
        ]);
    }

    adjacent_points
        .retain(|p| in_bounds(&world, &p));

    return adjacent_points;
}

fn make_person(id: i32, hobby: String) -> Person {
    let easygoingness: i32 = rand::random();
    let personality: i32 = rand::random();
    let mut hobbies: Vec<String> = Vec::new();
    hobbies.push(hobby);

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

fn move_person(world: &World<Location>, destination: Point) -> () {
    
}

fn find_adjecent_people(world: &World<Location>, )

fn get_at_world_coords(world: &World<Location>, coords: Point) -> Result<&Location, String> {
    if coords.y <= world.len() {
        let row = &world[coords.y];
        if coords.x <= row.len() {
            return Ok(&row[coords.x]);
        }
    }

    return Err("Out of bounds".to_string());
}

fn main() {
    let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());

    // say(&message, width, &mut writer).unwrap();

    let world = make_world(20, 20);


}


