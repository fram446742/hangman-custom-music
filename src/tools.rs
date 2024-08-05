use rand::seq::SliceRandom;
use rand::Rng;
use rpassword::read_password;
use std::io::{self, Write};
use termcolor::{Color, ColorSpec, StandardStream, WriteColor};

use crate::{
    consts::{Hangman, EASTEREGG, EASTEREGG2, INFO_DICT, INFO_DICTEN, WORDS_DICT, WORDS_DICTEN},
    main, COLOR, DICTIONARY, DIFFICULTY, LANGUAGE, NUM_PLAYERS,
};

pub fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap_or_else(|_| {
        eprintln!("Failed to read input");
        0
    });

    input
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
    let new_language = lan == "2";

    let mut lang = LANGUAGE.lock().unwrap();
    *lang = new_language;

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

// fn set_color(new_color: Color) {
//     let mut color = COLOR.lock().unwrap();
//     *color = new_color;
// }

pub fn get_color() -> Color {
    let color = COLOR.lock().unwrap();
    *color
}

// pub fn random_color() -> Color {
//     *RAINBOW_COLORS.choose(&mut rand::thread_rng()).unwrap()
// }

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

    // Retornar un color de `termcolor` a partir de los valores RGB
    Color::Rgb(r, g, b)
}

// Función para generar un color aleatorio y claro
pub fn random_color() -> Color {
    let mut rng = rand::thread_rng();
    let hue = rng.gen_range(0.0..360.0); // Tono (0 a 360 grados)
    let saturation = rng.gen_range(0.5..1.0); // Saturación (0.5 a 1.0 para colores vibrantes)
    let lightness = rng.gen_range(0.5..0.9); // Luminosidad (0.5 a 0.9 para evitar colores oscuros)

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

fn select_difficulty(hangman: &mut Hangman, stdout: &mut StandardStream, color: Color) {
    clear();
    print_colored_text(stdout, get_message(26), color);
    loop {
        let input = read_input();
        match input.trim() {
            "1" | "2" | "3" | "4" => {
                let lives = match input.trim() {
                    "1" => 6,
                    "2" => 4,
                    "3" => 2,
                    "4" => 1,
                    _ => unreachable!(),
                };
                hangman.lives = lives;
                hangman.initial_lives = lives;
                set_difficulty(lives);
                break;
            }
            _ => {
                print_colored_text(stdout, get_message(16), color);
            }
        }
    }
}

pub fn restart(stdout: &mut StandardStream, color: Color) {
    print_colored_text(stdout, get_message(17), color);
    let input = read_input().trim().to_uppercase();
    if input == "S" {
        main();
    } else {
        std::process::exit(0);
    }
}

fn config(stdout: &mut StandardStream, color: Color, hangman: &mut Hangman) {
    clear();
    print_colored_text(stdout, get_message(24), color);
    let input = read_input().trim().to_uppercase();
    match input.as_str() {
        "1" => {
            config(stdout, change_color(), hangman);
        }
        "2" => {
            set_players(stdout, color);
            config(stdout, color, hangman);
        }
        "3" => {
            set_language(stdout, color);
            config(stdout, color, hangman);
        }
        "4" => {
            select_difficulty(hangman, stdout, color);
            config(stdout, color, hangman);
        }
        "EASTEREGG" => {
            hid(stdout, color);
        }
        "5" | _ => {
            clear();
            main();
        }
    }
}

pub fn main_menu(stdout: &mut StandardStream, color: Color, hangman: &mut Hangman) {
    clear();
    print_colored_text(stdout, get_message(22), color);
    print_colored_text(stdout, get_message(1), color);

    let input = read_input();
    if input.trim().to_uppercase() == "T" {
        clear();
        print_colored_text(stdout, &get_message(23), color);
        print_colored_text(stdout, &get_message(2), color);
        read_input();
        main();
    } else if input.trim().to_uppercase() == "C" {
        clear();
        config(stdout, color, hangman);
    }
}

pub fn check_word(hangman: &mut Hangman, stdout: &mut StandardStream, color: Color) {
    if hangman.word.len() < 2 {
        print_colored_text(stdout, get_message(4), color);
        restart(stdout, color);
    } else if hangman.word.len() > 15 {
        print_colored_text(stdout, get_message(5), color);
        restart(stdout, color);
    }
}

pub fn hid(stdout: &mut StandardStream, color: Color) {
    print_colored_text(stdout, get_message(18), color);
    let pass = read_pass();
    if pass == "HIDDEN" {
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
    } else {
        print_colored_text(stdout, get_message(20), color);
        print_colored_text(stdout, get_message(21), color);
        read_input();
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
    if !*LANGUAGE.lock().unwrap() {
        dic = WORDS_DICT.iter().cloned().collect();
    } else {
        dic = WORDS_DICTEN.iter().cloned().collect();
    };

    let word = dic.choose(&mut rand::thread_rng()).unwrap().to_uppercase();
    word
}

fn change_color() -> Color {
    let new_color = random_color();
    let mut color = COLOR.lock().unwrap();
    *color = new_color;
    new_color
}
