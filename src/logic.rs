use crate::{hangman::Hangman, player::MusicPlayer, tools::*};
use termcolor::StandardStream;

fn game_over(hangman: &Hangman, stdout: &mut StandardStream) {
    print_colored_text(
        stdout,
        &format!("{} {}", get_message(12), hangman.word),
        hangman.color,
    );
}

fn game_finish(hangman: &Hangman, stdout: &mut StandardStream) {
    print_colored_text(
        stdout,
        &format!("{} {}", get_message(13), hangman.word),
        hangman.color,
    );
}

pub fn game(hangman: &mut Hangman, stdout: &mut StandardStream, music_player: &MusicPlayer) {
    main_menu(stdout, hangman, music_player);
    clear();

    hangman.display(None);

    loop {
        let input = read_char();
        if let Some(c) = input {
            let guess = c.to_uppercase().next().unwrap_or(c);
            if hangman.guess(guess) {
                hangman.display(Some(get_message(11)));
            }
        } else {
            print_colored_text(stdout, &get_message(8), hangman.color);
            continue;
        }

        if hangman.is_lost() {
            game_over(hangman, stdout);
            break;
        } else if hangman.is_won() {
            game_finish(hangman, stdout);
            break;
        }
    }
}
