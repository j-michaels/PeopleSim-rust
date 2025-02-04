use std::{cell::RefCell, cmp::Ordering, io::stdout, rc::Rc, vec};
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
    friends: Vec<Rc<RefCell<Person>>>,
    coords: Point,
    partying: bool,
}

#[derive(Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}

struct World {
    grid: Vec<Vec<Rc<RefCell<Location>>>>,
    people: Vec<Rc<RefCell<Person>>>,
}

type Location = Vec<Rc<RefCell<Person>>>;

fn make_world(width: usize, height: usize, n_people: u16) -> World {
    // let mut world = Vec::new();
    let mut world = World {
        grid: Vec::new(),
        people: Vec::new(),
    };

    for _i in 0..width {
        let mut row = Vec::new();

        for _j in 0..height {
            let location = Vec::new();

            row.push(Rc::new(RefCell::new(location)));
        }
        world.grid.push(row);
    }

    for i in 0..n_people {
        let mut person = make_person(i.into());
        
        person.hobbies.push(rand_hobby());
        person.coords = rand_location(&world);
        let coords = person.coords;
        let rc = Rc::new(RefCell::new(person));

        world.people.push(Rc::clone(&rc));
        world.grid[coords.x][coords.y].borrow_mut().push(Rc::clone(&rc));
    }

    return world;
}

fn rand_location(world: &World) -> Point {

    let x_coord = rand::thread_rng().gen_range(0..world.grid.len());
    let y_coord = rand::thread_rng().gen_range(0..world.grid.get(x_coord).unwrap().len());
    return Point {
        x: x_coord,
        y: y_coord,
    }
}

fn rand_hobby() -> String {
    return HOBBIES.choose(&mut rand::thread_rng()).unwrap().to_string();
}

fn is_compatible(person_a: &Person, person_b: &Person) -> bool {
    return (person_a.personality <= (person_b.personality + person_a.easygoingness))
        && (person_a.personality >= (person_b.personality - person_a.easygoingness))
        && (person_b.personality <= (person_a.personality + person_b.easygoingness))
        && (person_b.personality >= (person_a.personality - person_b.easygoingness));
}

fn in_bounds(world: &World, coords: &Point) -> bool {
    return coords.x <= world.grid.len()
        && coords.y <= world.grid.get(coords.x).unwrap().len()
}

fn find_adjacent_points(world: &World, coords: &Point, allow_diagonal: bool) -> Vec<Point> {
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

fn make_person(id: i32) -> Person {
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

fn move_person(world: &mut World, person: &mut Person, dest: &Point) {
    let origin = &person.coords;
    let origin_rc = &world.grid[origin.x][origin.y];
    let dest_rc = &world.grid[dest.x][dest.y];
    
    if let Some(index) = origin_rc.borrow().iter().position(|p| p.borrow().id == person.id) {
        if let Some(person_ref_cell) = origin_rc.borrow().get(index).cloned() {
            person.coords.x = dest.x;
            person.coords.y = dest.y;
            dest_rc.borrow_mut().push(person_ref_cell);
            origin_rc.borrow_mut().remove(index);
        }
    }
}

/* Returns the same value as FindAdjacents() except without any empty locations. */
fn find_adjacent_populated(world: &World, coords: &Point, allow_diagonal: bool) -> Vec<Point> {
    let mut adjacents = find_adjacent_points(world, coords, allow_diagonal);
    adjacents.retain(|p| world.grid[p.x][p.y].borrow().len() > 0);

    return adjacents;
}

fn iterate(world: &mut World, n_iterations: u8) {
    for person in &world.people {
        let adjacent_people = find_adjacent_populated(world, &person.borrow().coords, false);
        
        for adjacent_point in adjacent_people {
            let loc = &world.grid[adjacent_point.x][adjacent_point.y].borrow();

            for other_person in loc.iter() {
                if is_compatible(&person.borrow(), &other_person.borrow()) {
                    if let Some(other_person_rc) = world.people.iter().find(|&rc| Rc::ptr_eq(rc, person)) {
                        person.borrow_mut().friends.push(other_person_rc.clone());
                    }
                }
            }
        }
    }

    let people_len = world.people.len();
    for i in 0..people_len {
        // let adjacents = find_adjacent_points(world, &world.people[i].borrow().coords, false);

        // let destination = *adjacents.choose(&mut rand::thread_rng()).unwrap();
        let destination = Point {
            x: 0, y: 0
        };
        let person_rc = world.people[i].clone();
        move_person(world, &mut person_rc.borrow_mut(), &destination);
    }
}

fn main() {
    let stdout = stdout();
    // let message = String::from("Hello fellow Rustaceans!");
    // let width = message.chars().count();

    // let mut writer = BufWriter::new(stdout.lock());

    // say(&message, width, &mut writer).unwrap();

    let mut world = make_world(20, 20, 10);

    iterate(&mut world, 8);

}


