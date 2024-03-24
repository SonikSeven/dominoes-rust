
# Dominoes

## Project Overview

This Rust project implements a simple yet entertaining game of Dominoes, where the player competes against a computer-controlled opponent. The game logic handles creating hands for both the player and the computer, initializing the game board (snake), and executing the gameplay, which includes players' moves and the game's end conditions.

The primary components of the game include:
- `Hand`: Manages the pieces that the player and computer hold.
- `Stock`: Represents the pool of pieces from which players draw.
- `Snake`: The line of pieces that form the game board.
- `PieceSet`: Utility for managing domino pieces.


## Requirements

- [Rust](https://www.rust-lang.org/tools/install)

## Installation

This application is written in Rust, so you'll need Rust installed on your computer to build it. If you don't have Rust installed, you can download it from [rust-lang.org](https://www.rust-lang.org/tools/install).

To install this project, follow these steps:

1. Clone the repository to your local machine:
   
   ```
   git clone https://github.com/yourusername/minigrep.git
   ```
   
2. Navigate to the cloned repository:
   
   ```
   cd minigrep
   ```

3. Build the project with Cargo:
   
   ```
   cargo build --release
   ```

## How to Run

Run the executable in `./target/release/`.


## Gameplay

Upon starting the game, you will be presented with a menu asking you to choose between playing the game and exiting. Choose "play" to start the game. The game's rules are simple and follow standard dominoes rules:

1. **Starting the Game**: The game automatically deals hands to the player and the computer and initializes the board with the appropriate starting piece.

2. **Your Turn**: During your turn, you'll see your pieces and the current state of the game board. Enter the number corresponding to the piece you wish to play. You can also draw a piece by entering `0`.

3. **Computer's Turn**: The computer automatically makes its move after your turn.

4. **End of Game**: The game ends when either player has no pieces left or no more moves can be made. The score is then updated, and you can choose to play again or exit.


## License

This project is licensed under the [MIT License](LICENSE.txt).
