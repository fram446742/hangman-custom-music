use rand::seq::SliceRandom;
use rand::Rng;
use rpassword::read_password;
use std::{
    io::{self, Write},
    process::exit,
};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use crate::{
    consts::{EASTEREGG, EASTEREGG2, INFO_DICT, INFO_DICTEN, WORDS_DICT, WORDS_DICTEN},
    hangman::Hangman,
    main,
    player::MusicPlayer,
    DICTIONARY, DIFFICULTY, LANGUAGE, NUM_PLAYERS,
};

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        eprintln!("Failed to read input");
        0
    });

    input
}

pub fn read_char() -> Option<char> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        input.trim().chars().next()
    } else {
        eprintln!("Failed to read input");
        None
    }
}

fn read_pass() -> String {
    read_password().unwrap_or_else(|_| {
        eprintln!("Failed to read password");
        String::new()
    })
}

pub fn clear() {
    print!("\x1B[2J\x1B[1;1H");
    io::stdout().flush().unwrap();
}

pub fn print_colored_text(stdout: &mut StandardStream, text: &str, color: Color) {
    stdout
        .set_color(ColorSpec::new().set_fg(Some(color)).set_bold(true))
        .unwrap();
    writeln!(stdout, "{}", text).unwrap();
    stdout.reset().unwrap();
}

fn set_language(stdout: &mut StandardStream, color: Color) {
    clear();
    print_colored_text(stdout, get_message(25), color);
    let lan = read_input().trim().to_uppercase();
    // Sets the language to Spanish if the user types "1" and to English if the user types "2"
    let new_language = lan == "2";

    let mut lang = LANGUAGE.lock().unwrap();
    *lang = new_language;

    // Update the dictionary with the new language
    let new_dict = if new_language {
        INFO_DICTEN.iter().cloned().collect()
    } else {
        INFO_DICT.iter().cloned().collect()
    };

    let mut dic = DICTIONARY.lock().unwrap();
    *dic = new_dict;
}

pub fn get_message(id: u8) -> &'static str {
    const DEFAULT_MESSAGE: &'static str = "ERROR! Message not found";
    let dic = DICTIONARY.lock().unwrap();
    dic.get(&id).unwrap_or(&DEFAULT_MESSAGE)
}

// Conversión de HSL a RGB
fn hsl_to_termcolor(h: f64, s: f64, l: f64) -> Color {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h {
        h if h < 60.0 => (c, x, 0.0),
        h if h < 120.0 => (x, c, 0.0),
        h if h < 180.0 => (0.0, c, x),
        h if h < 240.0 => (0.0, x, c),
        h if h < 300.0 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    let r = ((r + m) * 255.0).round() as u8;
    let g = ((g + m) * 255.0).round() as u8;
    let b = ((b + m) * 255.0).round() as u8;

    Color::Rgb(r, g, b)
}

pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let hue = rng.gen_range(0.0..360.0); // Tono
    let saturation = rng.gen_range(0.5..1.0); // Saturación
    let lightness = rng.gen_range(0.5..0.9); // Luminosidad

    hsl_to_termcolor(hue, saturation, lightness)
}

fn set_difficulty(lives: u8) {
    let mut dif = DIFFICULTY.lock().unwrap();
    *dif = lives;
}

pub fn get_difficulty() -> u8 {
    let dif = DIFFICULTY.lock().unwrap();
    *dif
}

fn select_difficulty(hangman: &mut Hangman, stdout: &mut StandardStream) {
    clear();
    print_colored_text(stdout, get_message(26), hangman.color);
    loop {
        let read = read_input();
        let input = read.trim();
        // Check if the input is a number between 1 and 4 and set the lives accordingly
        let lives = match input {
            "1" => 6,
            "2" => 4,
            "3" => 2,
            "4" => 1,
            _ => {
                print_colored_text(stdout, get_message(16), hangman.color);
                continue;
            }
        };
        hangman.lives = lives;
        hangman.initial_lives = lives;
        set_difficulty(lives);
        break;
    }
}

fn config(stdout: &mut StandardStream, hangman: &mut Hangman, music_player: &MusicPlayer) {
    loop {
        clear();
        print_colored_text(stdout, get_message(24), hangman.color);
        let input = read_input().trim().to_uppercase();
        // Check the user input and call the corresponding function
        match input.as_str() {
            "1" => {
                hangman.color = random_color();
            }
            "2" => {
                music_player.toggle_music();
            }
            "3" => {
                set_players(stdout, hangman.color);
            }
            "4" => {
                set_language(stdout, hangman.color);
                hangman.change_word(get_word());
            }
            "5" => {
                select_difficulty(hangman, stdout);
            }
            "EASTEREGG" => {
                hid(stdout, hangman.color);
            }
            "6" | _ => {
                // clear();
                break;
            }
        }
    }
    main_menu(stdout, hangman, music_player);
}

pub fn main_menu(stdout: &mut StandardStream, hangman: &mut Hangman, music_player: &MusicPlayer) {
    clear();
    print_colored_text(stdout, get_message(22), hangman.color);
    print_colored_text(stdout, get_message(1), hangman.color);

    let input = read_input().trim().to_uppercase();
    match input.as_str() {
        "I" => {
            clear();
            print_colored_text(stdout, &get_message(23), hangman.color);
            print_colored_text(stdout, &get_message(2), hangman.color);
            read_input();
        }
        "S" | "A" => {
            clear();
            config(stdout, hangman, music_player);
        }
        "E" => {
            exit(0);
        }
        _ => (),
    }
}

#[allow(dead_code)]
pub fn check_word(hangman: &Hangman, stdout: &mut StandardStream) -> bool {
    // Checks if the word is valid
    if hangman.word.len() < 2 {
        print_colored_text(stdout, get_message(4), hangman.color);
        false
    } else if hangman.word.len() > 15 {
        print_colored_text(stdout, get_message(5), hangman.color);
        false
    } else {
        true
    }
}

pub fn hid(stdout: &mut StandardStream, color: Color) {
    print_colored_text(stdout, get_message(18), color);
    let pass = read_pass();
    match pass == "HIDDEN" || pass == "hidden" || pass == "Hidden" || pass == "OCULTO" || pass == "oculto" || pass == "Oculto" {
        true => {
            print_colored_text(stdout, get_message(19), color);
            print_colored_text(stdout, get_message(6), color);
            print_colored_text(stdout, EASTEREGG, color);
            print_colored_text(stdout, get_message(21), color);
            read_input();
            if rand::thread_rng().gen_range(0..11) == 5 {
                print_colored_text(stdout, EASTEREGG2, color);
                print_colored_text(stdout, get_message(27), color);
                read_input();
            }
        }
        false => {
                    print_colored_text(stdout, get_message(20), color);
                    print_colored_text(stdout, get_message(21), color);
                    read_input();
                }
    }
    main();
}

fn set_players(stdout: &mut StandardStream, color: Color) {
    clear();
    print_colored_text(stdout, get_message(28), color);
    loop {
        let input = read_input().trim().to_uppercase();
        match input.as_str() {
            "1" => {
                *NUM_PLAYERS.lock().unwrap() = 1;
                break;
            }
            "2" => {
                *NUM_PLAYERS.lock().unwrap() = 2;
                break;
            }
            _ => {
                print_colored_text(stdout, get_message(16), color);
            }
        }
    }
}

pub fn get_word() -> String {
    let dic: Vec<&'static str>;
    let lan = *LANGUAGE.lock().unwrap();
    if !lan {
        dic = WORDS_DICT.iter().cloned().collect();
    } else {
        dic = WORDS_DICTEN.iter().cloned().collect();
    };

    let word = dic.choose(&mut rand::thread_rng()).unwrap().to_uppercase();
    word
}