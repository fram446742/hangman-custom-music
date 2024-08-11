use std::collections::HashSet;
use termcolor::{Color, StandardStream};

use crate::{
    consts::{STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6},
    tools::{clear, get_message, print_colored_text, random_color},
};

#[derive(Debug, Clone)]
pub struct Hangman {
    pub history: HashSet<char>,
    pub word: String,
    pub hidden_letter: String,
    pub attempt: Option<char>,
    pub lives: u8,
    pub initial_lives: u8,
    pub color: Color,
    pub stages: Vec<&'static str>,
}

#[allow(dead_code)]
impl Hangman {
    // Constructor to create a new Hangman game
    pub fn new(word: String, initial_lives: u8) -> Hangman {
        let hidden_letter = "_ ".repeat(word.len());
        Hangman {
            history: HashSet::new(),
            word,
            hidden_letter,
            attempt: None,
            lives: initial_lives,
            initial_lives,
            color: random_color(),
            stages: Hangman::initialize_stages(initial_lives),
        }
    }

    // Private method to initialize stages based on initial_lives
    fn initialize_stages(initial_lives: u8) -> Vec<&'static str> {
        match initial_lives {
            6 => vec![
                STAGE_0, STAGE_1, STAGE_2, STAGE_3, STAGE_4, STAGE_5, STAGE_6,
            ],
            4 => vec![STAGE_0, STAGE_2, STAGE_4, STAGE_5, STAGE_6],
            2 => vec![STAGE_0, STAGE_2, STAGE_6],
            1 => vec![STAGE_0, STAGE_6],
            _ => vec![STAGE_0], // Default stage if initial_lives is unknown
        }
    }

    // Make a guess and update the game state
    pub fn guess(&mut self, letter: char) -> bool {
        if self.history.contains(&letter) {
            self.display(Some(get_message(9)));
            return false; // Already guessed this letter
        }

        self.history.insert(letter);

        if self.word.contains(letter) {
            self.update_hidden_letter();
            true
        } else {
            self.lives = self.lives.saturating_sub(1);
            self.display(Some(get_message(10)));
            false
        }
    }

    // Check if the game is won
    pub fn is_won(&self) -> bool {
        self.hidden_letter == self.word
    }

    // Check if the game is lost
    pub fn is_lost(&self) -> bool {
        self.lives == 0
    }

    // Reveal the hidden letters based on the latest guess
    fn update_hidden_letter(&mut self) {
        self.hidden_letter = self
            .word
            .chars()
            .map(|c| if self.history.contains(&c) { c } else { '_' })
            .collect();
    }

    // Refresh the hidden letters based on the latest attempt
    pub fn refresh_line(&mut self) {
        if let Some(letter) = self.attempt {
            self.history.insert(letter);
            self.update_hidden_letter();
        }
    }

    // Change the word and reset the game state
    pub fn change_word(&mut self, new_word: String) {
        self.word = new_word.clone();
        self.hidden_letter = "_ ".repeat(new_word.len());
        self.history.clear();
        self.lives = self.initial_lives;
        self.stages = Hangman::initialize_stages(self.initial_lives);
    }

    // Display the current game state
    pub fn display(&self, message: Option<&str>) {
        let mut stdout = StandardStream::stdout(termcolor::ColorChoice::Always);
        clear();

        if let Some(stage) = self.stages.get(self.lives as usize) {
            print_colored_text(&mut stdout, stage, self.color);
        }

        // Conditionally print the message if it's Some
        if let Some(msg) = message {
            print_colored_text(&mut stdout, msg, self.color);
        }

        print_colored_text(
            &mut stdout,
            &format!("{} {}", get_message(30), self.hidden_letter),
            self.color,
        );
        print_colored_text(
            &mut stdout,
            &format!("{} {}", get_message(14), self.lives),
            self.color,
        );
        print_colored_text(
            &mut stdout,
            &format!("{} {:?}", get_message(31), self.history),
            self.color,
        );
    }
}
