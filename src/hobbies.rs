use rand::seq::SliceRandom;

pub const HOBBIES: [&'static str; 8] = [
    "Knitting",
    "Baking Cookies",
    "Playing Bingo",
    "Gardening",
    "Arranging Furniture",
    "Dusting",
    "Flying Kites",
    "Playing Scrabble"
];

pub fn rand_hobby() -> String {
    return HOBBIES.choose(&mut rand::thread_rng()).unwrap().to_string();
}