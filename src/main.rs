use consts::INFO_DICT;
use lazy_static::lazy_static;
use logic::game;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use termcolor::{Color, ColorChoice, StandardStream};
use tools::{clear, get_color, get_difficulty, random_color};
mod consts;
mod logic;
mod tools;

pub struct Hangman {
    history: HashSet<char>,
    word: String,
    hidden_letter: String,
    attempt: Option<char>,
    lives: u8,
    initial_lives: u8,
}

lazy_static! {
    pub static ref COLOR: Mutex<Color> = Mutex::new(random_color());
    pub static ref COLOR2: Mutex<Color> = Mutex::new(random_color());
    pub static ref LANGUAGE: Mutex<bool> = Mutex::new(false);
    pub static ref DICTIONARY: Mutex<HashMap<u8, &'static str>> =
        Mutex::new(INFO_DICT.iter().cloned().collect());
    pub static ref DIFFICULTY: Mutex<u8> = Mutex::new(4);
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
