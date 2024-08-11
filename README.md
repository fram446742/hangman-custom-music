# Hangman Game

Welcome to **Hangman Game**! This is a console-based version of the classic hangman game, written in Rust. The game supports both English and Spanish languages, features a colorful console experience, and lets you choose your own background music to enhance the gaming experience.

## Features

- **Multilingual Support**: Play in either English or Spanish. You can switch languages seamlessly to enjoy the game in your preferred language.
- **Colorful Console**: The game leverages multicolored output to provide an engaging visual experience right in your terminal.
- **Customizable Music**: Choose your favorite background music from the available options to make your gameplay even more enjoyable.
- **Easter Egg**: Keep an eye out for a hidden surprise within the game!

## Getting Started

### Prerequisites

Ensure you have [Rust](https://www.rust-lang.org/) installed on your system. If not, you can install it using the following command:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Installation

Clone the repository to your local machine:

```bash
git clone https://github.com/yourusername/hangman-game.git
cd hangman-game
```

### Build the game

You can build the game by running the following command:

```bash
cargo build --release
```

### Running the Game

You can start the game by running the following command:

```bash
cargo run --release
```

## Gameplay

Upon starting the game, you will be prompted to choose a language (English or Spanish).
Next, select your preferred background music from the available options.
The game will then begin, and you will guess letters to try and figure out the hidden word.
Enjoy the colorful interface as you play!

## Customization

You can add your own music files to the music directory within the game’s root folder. Make sure they are in a supported format (e.g., .mp3, .ogg, .wav, .flac, .acc).

## Contributions

Contributions are welcome! Feel free to submit a pull request or open an issue for any bugs or features you would like to see.

## License

This project is licensed under the GNU License - see the [LICENSE] file for details.

## Acknowledgements

Thanks to the Rust community and all the open-source projects that made this game possible. Also, there might be a little surprise hidden in the code – happy hunting!
