Heuristic Tetris-playing bot, heavily based on our uni group project 'Project-1-1'.

There is an edge case left to be handled when applying gravity:

- if a piece is separated due to a row clear, where at least one of the tiles
- is in the bottom row and there at least two connected tiles of said piece
- separated from the bottom row, then they will be left floating.
