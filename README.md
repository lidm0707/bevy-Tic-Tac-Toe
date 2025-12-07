# bevy-Tic-Tac-Toe

A simple Tic-Tac-Toe game built using the [Bevy](https://bevyengine.org/) game engine in Rust.

## Description

This project implements a classic Tic-Tac-Toe game with a graphical user interface. The game features:
- Interactive grid-based gameplay
- Turn-based mechanics (X and O players)
- Win detection
- Menu system
- Game-over screen with option to return to menu

## Requirements

- Rust 2024 edition or later
- Bevy engine 0.17.3 or later

## Installation and Running

1. Clone the repository:
```bash
git clone https://github.com/your-username/bevy-Tic-Tac-Toe.git
cd bevy-Tic-Tac-Toe
```

2. Run the game:
```bash
cargo run
```

## How to Play

1. Start the game from the main menu
2. Click on an empty cell to place your mark (X or O)
3. Players take turns marking cells
4. The game ends when a player gets three marks in a row (horizontal, vertical, or diagonal) or when all cells are filled
5. After the game ends, you can return to the main menu to start a new game

## Project Structure

- `src/main.rs` - Entry point and game initialization
- `src/state.rs` - Game state management
- `src/plugins/` - Game modules
  - `menu/` - Menu system
  - `game/` - Core game logic and systems

## Game Architecture

The game is built using Bevy's Entity-Component-System (ECS) architecture with the following components:
- `Turn` - Tracks current player turn
- `Board` - Represents the game board state
- `Winner` - Stores the game result

## License

This project is open source. Please see the LICENSE file for more information.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.