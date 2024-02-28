Heuristic Tetris-playing bot, heavily based on our uni group project 'Project-1-1'.

There is an edge case left to be handled when applying gravity:

- if a piece is separated due to a row clear, then if at least one of the original tiles of the piece
- is in the bottom row and there are 2 or more connected tiles of said piece separated from the bottom row
- tile, then they will be left floating. e.g.:

BEFORE ROW CLEAR + GRAVITY:
------------
|          |
|          |
|          |
|         X|
|         X|
|O O O O OX|
|         X|
|_________X|

AFTER ROW CLEAR + GRAVITY:
------------
|          |
|          |
|          |
|         X| // left floating
|         X| // left floating
|          |
|         X|
|_________X|

All 'X' share the same composite_id and bottom-row 'X' is marked as settled, then all 'X' above will be falsely settled.