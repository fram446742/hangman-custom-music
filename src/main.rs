use logic::game;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::process::exit;
use std::sync::Mutex;
use {consts::INFO_DICT, hangman::Hangman, player::MusicPlayer};
// use std::time::Duration;
use termcolor::{ColorChoice, StandardStream};
use tools::{clear, get_difficulty, get_message, get_word, print_colored_text, read_input};
mod consts;
mod hangman;
mod logic;
mod player;
mod tools;

static LANGUAGE: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
static DICTIONARY: Lazy<Mutex<HashMap<u8, &'static str>>> =
    Lazy::new(|| Mutex::new(INFO_DICT.iter().cloned().collect()));
static DIFFICULTY: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(4));
static NUM_PLAYERS: Lazy<Mutex<u8>> = Lazy::new(|| Mutex::new(1));

fn game_loop(music_player: MusicPlayer) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    loop {
        clear();

        let player1 = Mutex::new(Hangman::new(get_word(), get_difficulty()));

        // Play the game
        {
            let mut hangman = player1.lock().unwrap();
            game(&mut hangman, &mut stdout, &music_player);
        }

        // Ask the user if they want to restart or exit
        let hangman = player1.lock().unwrap();
        print_colored_text(&mut stdout, get_message(17), hangman.color);
        let input = read_input().trim().to_uppercase();

        if input != "S" {
            break; // Exit the loop if the user does not want to restart
        }
    }

    // Exit the game cleanly
    exit(0);
}

fn music_task() -> MusicPlayer {
    let mut music_player = MusicPlayer::new();
    music_player.init();
    music_player.start_music();

    music_player
}

fn main() {
    let player = music_task();
    game_loop(player); // Start the game loop
}
