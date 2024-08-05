use consts::{Hangman, INFO_DICT};
use lazy_static::lazy_static;
use logic::game;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use termcolor::{Color, ColorChoice, StandardStream};
use tools::{clear, get_color, get_difficulty, random_color};
mod consts;
mod logic;
mod tools;

lazy_static! {
    pub static ref COLOR: Mutex<Color> = Mutex::new(random_color());
    pub static ref LANGUAGE: Mutex<bool> = Mutex::new(false);
    pub static ref DICTIONARY: Mutex<HashMap<u8, &'static str>> =
        Mutex::new(INFO_DICT.iter().cloned().collect());
    pub static ref DIFFICULTY: Mutex<u8> = Mutex::new(4);
    pub static ref NUM_PLAYERS: Mutex<u8> = Mutex::new(1);
}

fn main() {
    clear();
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let color = get_color();
    let mut hangman = Hangman {
        history: HashSet::new(),
        word: String::new(),
        hidden_letter: String::new(),
        attempt: None,
        lives: get_difficulty(),
        initial_lives: get_difficulty(),
    };

    game(&mut hangman, &mut stdout, color);
}
