use std::{cell::RefCell, rc::Rc};

use rand::{seq::SliceRandom, Rng};

use crate::{hobbies::rand_hobby, person::{Person, Point}};

pub type Location = Vec<Rc<RefCell<Person>>>;

pub struct World {
    grid: Vec<Vec<Rc<RefCell<Location>>>>,
    people: Vec<Rc<RefCell<Person>>>,
}

impl World {
    pub fn new(width: usize, height: usize, n_people: u16) -> Self {
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
            let mut person = Person::new(i.into());
            
            person.hobbies.push(rand_hobby());
            person.coords = world.rand_location();
            let coords = person.coords;
            let rc = Rc::new(RefCell::new(person));
    
            world.people.push(Rc::clone(&rc));
            world.grid[coords.x][coords.y].borrow_mut().push(Rc::clone(&rc));
        }
    
        return world;
    }

    pub fn move_person(&self, person: &mut Person, dest: &Point) {
        let origin = &person.coords;
        let origin_rc = &self.grid[origin.x][origin.y];
        let dest_rc = &self.grid[dest.x][dest.y];
        
        if let Some(index) = origin_rc.borrow().iter().position(|p| p.borrow().id == person.id) {
            if let Some(person_ref_cell) = origin_rc.borrow().get(index).cloned() {
                person.coords.x = dest.x;
                person.coords.y = dest.y;
                dest_rc.borrow_mut().push(person_ref_cell);
                origin_rc.borrow_mut().remove(index);
            }
        }
    }

    pub fn in_bounds(&self, coords: &Point) -> bool {
        return coords.x <= self.grid.len()
            && coords.y <= self.grid.get(coords.x).unwrap().len()
    }

    pub fn find_adjacent_points(&self, coords: &Point, allow_diagonal: bool) -> Vec<Point> {
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
            .retain(|p| self.in_bounds(&p));
    
        return adjacent_points;
    }

    /* Returns the same value as FindAdjacents() except without any empty locations. */
    pub fn find_adjacent_populated(&self, coords: &Point, allow_diagonal: bool) -> Vec<Point> {
        let mut adjacents = self.find_adjacent_points(coords, allow_diagonal);
        adjacents.retain(|p| self.grid[p.x][p.y].borrow().len() > 0);

        return adjacents;
    }

    pub fn iterate(&mut self, n_iterations: u8) {
        for person in &self.people {
            let adjacent_people = self.find_adjacent_populated(&person.borrow().coords, false);
            
            for adjacent_point in adjacent_people {
                let loc = &self.grid[adjacent_point.x][adjacent_point.y].borrow();
    
                for other_person in loc.iter() {
                    if (&person.borrow()).is_compatible(&other_person.borrow()) {
                        if let Some(other_person_rc) = self.people.iter().find(|&rc| Rc::ptr_eq(rc, person)) {
                            person.borrow_mut().friends.push(other_person_rc.clone());
                        }
                    }
                }
            }
        }
    
        let people = self.people.clone();
        for person in people {
            let destination = Point {
                x: 0, y: 0
            };
            self.move_person(&mut person.borrow_mut(), &destination);
        }
    }

    pub fn party(&self, n_parties: u16) {
        // let people = self.people.clone();
        let party_throwers = self.rand_people(n_parties);

        // first need to flag party_throwers as partying so they don't get teleported
        for party_thrower in &party_throwers {
            party_thrower.borrow_mut().partying = true;
        }

        for party_thrower in &party_throwers {
            let hobby = party_thrower.borrow().rand_hobby();

            let friends = party_thrower.borrow().friends.clone();
            for friend in friends {
                let diceroll: u8 = rand::thread_rng().gen_range(0..100);

                // 60% chance of attending
                if diceroll > 40 {
                    friend.borrow_mut().partying = true;
                    self.move_person(&mut friend.borrow_mut(), &party_thrower.borrow().coords);

                    
                }
            }
        }
    }

    pub fn rand_location(&self) -> Point {
        let x_coord = rand::thread_rng().gen_range(0..self.grid.len());
        let y_coord = rand::thread_rng().gen_range(0..self.grid.get(x_coord).unwrap().len());
        return Point {
            x: x_coord,
            y: y_coord,
        }
    }

    pub fn rand_person(&self) -> Rc<RefCell<Person>> {
        return Rc::clone(self.people.choose(&mut rand::thread_rng()).unwrap());
    }

    pub fn rand_people(&self, n_people: u16) -> Vec<Rc<RefCell<Person>>> {
        let mut people: Vec<Rc<RefCell<Person>>> = Vec::new();
        for _i in 0..n_people {
            people.push(self.rand_person());
        }
        return people;
    }
}