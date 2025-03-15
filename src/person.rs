use std::{cell::RefCell, rc::Rc};

use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

pub struct Person {
    pub easygoingness: i32,
    pub personality: i32,
    pub id: i32,
    pub hobbies: Vec<String>,
    pub friends: Vec<Rc<RefCell<Person>>>,
    pub coords: Point,
    pub partying: bool,
}

impl Person {
    pub fn new(id: i32) -> Self {
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

    pub fn rand_hobby(&self) -> String {
        return self.hobbies.choose(&mut rand::thread_rng()).unwrap().to_string();
    }

    pub fn is_compatible(&self, other_person: &Person) -> bool {
        return (self.personality <= (other_person.personality + self.easygoingness))
            && (self.personality >= (other_person.personality - self.easygoingness))
            && (other_person.personality <= (self.personality + other_person.easygoingness))
            && (other_person.personality >= (self.personality - other_person.easygoingness));
    }
}