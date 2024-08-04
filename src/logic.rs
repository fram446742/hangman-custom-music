use termcolor::{Color, StandardStream};

use crate::{
    consts::{STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6},
    tools::*,
    Hangman,
};

fn game_over(hangman: &mut Hangman, stdout: &mut StandardStream, color: Color) {
    print_colored_text(
        stdout,
        &format!("{} {}", get_message(12), hangman.word),
        color,
    );
    restart(stdout, color);
}

fn game_finish(hangman: &mut Hangman, stdout: &mut StandardStream, color: Color) {
    print_colored_text(
        stdout,
        &format!("{} {}", get_message(13), hangman.word),
        color,
    );
    restart(stdout, color);
}

fn refresh_line(hangman: &mut Hangman, stdout: &mut StandardStream, color: Color) {
    let mut hidden_letter_vec: Vec<char> = hangman.hidden_letter.chars().collect();
    for (i, letter) in hangman.word.chars().enumerate() {
        if letter == hangman.attempt.unwrap() {
            hidden_letter_vec[i * 2] = letter;
        }
    }
    hangman.hidden_letter = hidden_letter_vec.iter().collect();
    constructor(get_message(11), hangman, stdout, color);
}

fn constructor(message: &str, hangman: &mut Hangman, stdout: &mut StandardStream, color: Color) {
    clear();
    print_colored_text(
        stdout,
        &format!("{} {}", get_message(14), hangman.lives),
        color,
    );

    let stages = match hangman.initial_lives {
        6 => vec![STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6],
        4 => vec![STAGE_0, STAGE_2, STAGE_4, STAGE_5, STAGE_6],
        2 => vec![STAGE_0, STAGE_2, STAGE_6],
        1 => vec![STAGE_0, STAGE_6],
        _ => return, // Early return for invalid state
    };

    if let Some(stage) = stages.get(hangman.lives as usize) {
        print_colored_text(stdout, stage, color);
    }

    print_colored_text(stdout, &hangman.hidden_letter, color);
    print_colored_text(stdout, message, color);
}

pub fn game(hangman: &mut Hangman, stdout: &mut StandardStream, color: Color) {
    loop {
        main_menu(stdout, color, hangman);
        clear();

        print_colored_text(stdout, &get_message(3), color);
        hangman.word = read_input().trim().to_uppercase();

        check_word(hangman, stdout, color);

        hangman.hidden_letter = "_ ".repeat(hangman.word.len());

        clear();
        constructor(&get_message(7), hangman, stdout, color);

        loop {
            let input = read_input().trim().chars().next();
            if let Some(c) = input {
                hangman.attempt = Some(c.to_uppercase().next().unwrap());
            } else {
                print_colored_text(stdout, &get_message(8), color);
                continue;
            }

            if hangman.history.contains(&hangman.attempt.unwrap()) {
                constructor(&get_message(9), hangman, stdout, color);
            } else {
                hangman.history.insert(hangman.attempt.unwrap());
                if hangman.word.contains(hangman.attempt.unwrap()) {
                    refresh_line(hangman, stdout, color);
                } else {
                    hangman.lives -= 1;
                    constructor(&get_message(10), hangman, stdout, color);
                }
            }

            if hangman.lives == 0 {
                game_over(hangman, stdout, color);
                break;
            } else if !hangman.hidden_letter.contains('_') {
                game_finish(hangman, stdout, color);
                break;
            }
        }
    }
}
