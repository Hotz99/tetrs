## What ?

A heuristic-based pathfinding algorithm (`bot`) for resolving a valid pentomino piece
placement (`solution`), given the current game state and foresight on the next-up shapes (`lookahead`), provided by the game, with the aim of maximizing the number of rows-cleared.

## Why ?

To learn and practice Rust.

## System Component Overview

**Game**: a rough implementation of Tetris, where clearing and piece placement mechanics slightly deviate from the original. Pieces are pentominoes.

- **`bot` module**:
  - is provided with foresight of the next-up pieces resolved by the game mechanics
  - computes optimal moves using a priority queue and custom heuristic function:
  - simulates all placings, exploring the most promising states, derived from the heuristic

- **`egui` interface** for visualizing the game and bot's placements.

- **Performance testing mode**, with the metrics:
  - total solutions count
  - average solution time
  - solutions per second
  - average run time
  - failed runs count

## Getting Started

### Prerequisites

- Rust and Cargo installed on your system.
- A terminal or command prompt.

### Steps

1. **Clone the repository:**
    ```sh
    git clone https://github.com/Hotz99/tetrs.git
    cd tetrs
    ```

2. **Run the application:**
    ```sh
    cargo run --release
    ```

3. **Performance Testing:**
    ```sh
    cargo run --release -- --perf [n_runs] [n_searches] [lookahead_size]
    ```

    Defaults to `n_runs = 100`, `n_searches = 1000` and `lookahead_size = 4`, where:
    - `run` is a game starting from scratch
    - `search` is an attempt at finding the optimal piece placement, given the current game state
    - `lookahead_size` is the number of next-up shapes (provided by the game) the algorithm has foresight on

4. **Generate a Flamegraph:**
    To generate a flamegraph for performance analysis, use the following command:
    ```sh
    cargo flamegraph -- --perf [n_runs] [n_searches] [lookahead_size]
    ```

6. **Explore the Code:**
    - `src/main.rs`: Entry point of the application.
    - `src/app.rs`: `App` struct, performance testing and visualization logic.
    - `src/bot.rs`: Solution resolution logic and employed search algorithm.
    - `src/game.rs`: Game state management logic.
    - `src/ui.rs`: UI rendering logic.

## Further Work
  1. There's a substantial amount of refactoring, proper error handling and test cases lacking, though time is scarce.

  2. An edge case when applying gravity is causing invalid game behavior:
  - if a piece is separated due to a row clear, where at least one of the tiles
    is in the bottom row and there at least two connected tiles of said piece separated from the bottom row, they will be left floating.