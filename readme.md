Tetris-like game playing bot, referencing our uni project 'Project-1-1' specification. 

Employs a heuristic search, similar to the A* algorithm.

There is an edge case left to be handled when applying gravity:

- If a piece is separated due to a row clear, where at least one of the tiles
  is in the bottom row and there at least two connected tiles of said piece separated from the bottom row,
  then they will be left floating.
