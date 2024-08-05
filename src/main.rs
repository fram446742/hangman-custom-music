use consts::{Hangman, INFO_DICT};
use logic::game;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use termcolor::{Color, ColorChoice, StandardStream};
use tools::{clear, get_color, get_difficulty, random_color};
use once_cell::sync::Lazy;
mod consts;
mod logic;
mod tools;

static COLOR: Lazy<Mutex<Color>> = Lazy::new(|| Mutex::new(random_color()));
static LANGUAGE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static DICTIONARY: Lazy<Mutex<HashMap<u8, &'static str>>> =
    Lazy::new(|| Mutex::new(INFO_DICT.iter().cloned().collect()));
static DIFFICULTY: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(4));
static NUM_PLAYERS: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(1));

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
