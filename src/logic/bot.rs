use std::collections::{BinaryHeap, HashSet};

use crate::ui;

use crate::data::pentomino_db::PentominoDB;

use super::state::{self, State};

pub struct Bot {}

impl Bot {
    pub fn astar_search(state: &State, pent_db: &PentominoDB) -> Option<State> {
        let mut visited = HashSet::new();
        let mut queue = BinaryHeap::new();

        visited.insert(state.clone());

        let initial_state = state.clone();

        // println!("INITIAL STATE: {}", initial_state.clone());

        queue.push(initial_state);

        while let Some(current_state) = queue.pop() {
            if current_state.remaining_pieces.is_empty() {
                // with a lookahead (state::STACK_SIZE) of N, go back N-1 generations

                let mut final_state = current_state;
                let n = 5;

                for _ in 0..n - 1 {
                    match final_state.parent_state {
                        Some(parent_state) => final_state = *parent_state,
                        None => break,
                    }
                }

                // final_state.animate_clearing();

                return Some(final_state);
            }

            let children_states =
                Self::generate_states(&current_state, current_state.remaining_pieces[0], pent_db);

            for mut child in children_states {
                if visited.insert(child.clone()) {
                    child.heuristic = Self::heuristic(&mut child);
                    queue.push(child);
                }
            }
        }

        println!("No solution found");
        None
    }

    fn generate_states(parent_state: &State, piece: char, pent_db: &PentominoDB) -> Vec<State> {
        let mut states = Vec::new();
        let piece_id = Self::char_to_id(piece);

        for mutation in &pent_db.data[piece_id as usize] {
            for x in 0..=(state::FIELD_WIDTH as usize - mutation[0].len()) {
                for y in 0..=(state::FIELD_HEIGHT as usize - mutation.len()) {
                    let mut parent_clone = parent_state.clone();

                    if Self::try_place(&mut parent_clone, mutation, &piece_id, x, y) {
                        let child_state = State::new(
                            &parent_clone.field.clone(),
                            &parent_clone.uncleared_field.clone(),
                            Some(Box::new(parent_clone)),
                            // better approach ?
                            &parent_state
                                .remaining_pieces
                                .iter()
                                .filter(|&x| *x != piece)
                                .cloned()
                                .collect(),
                            &parent_state.used_ids.clone(),
                        );

                        states.push(child_state);
                    }
                }
            }
        }
        states
    }

    fn try_place(
        state: &mut State,
        piece: &Vec<Vec<u8>>,
        piece_id: &u8,
        x: usize,
        y: usize,
    ) -> bool {
        for delta_x in 0..piece[0].len() {
            for delta_y in 0..piece.len() {
                if piece[delta_y][delta_x] == 0 {
                    continue;
                }

                let field_x = x + delta_x;

                let field_y = y + delta_y;

                // if cell is out of bounds
                if field_x >= state.field[0].len() || field_y >= state.field.len() {
                    return false;
                }

                // if already occupied / overlap with other pieces
                if state.field[field_y][field_x] != state::EMPTY {
                    return false;
                }

                state.uncleared_field = state.field.clone();

                // actually place the piece
                state.field[field_y][field_x] = piece_id.clone();
            }
        }

        true
    }

    pub fn heuristic(state: &mut State) -> i32 {
        let mut penalty = 0;

        for row in 0..state::FIELD_HEIGHT {
            // penalty bias towards bottom rows
            let penalize_top = (12 * state::FIELD_HEIGHT as i32 / (row as i32 + 1)) << 13;

            for col in 0..state::FIELD_WIDTH {
                if state.field[row as usize][col as usize] != state::EMPTY {
                    penalty += penalize_top;
                } else {
                    penalty -= penalize_top;
                }
            }
        }

        // BUG: clear_full_rows clears the whole field ???

        let cleared_rows = state.get_full_rows();

        penalty -= (cleared_rows ^ 4 * 9000) as i32;

        penalty
    }

    fn char_to_id(c: char) -> u8 {
        match c {
            'X' => 0,
            'I' => 1,
            'Z' => 2,
            'T' => 3,
            'U' => 4,
            'V' => 5,
            'W' => 6,
            'Y' => 7,
            'L' => 8,
            'P' => 9,
            'N' => 10,
            'F' => 11,
            _ => 255,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use state::EMPTY;

    #[test]
    fn test_heuristic() {
        let mut state_a = State::initial_state(&vec![
            'X', 'I', 'Z', 'T', 'U', 'V', 'W', 'Y', 'L', 'P', 'N', 'F',
        ]);

        state_a.field = vec![
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let mut state_b = State::initial_state(&vec![
            'X', 'I', 'Z', 'T', 'U', 'V', 'W', 'Y', 'L', 'P', 'N', 'F',
        ]);

        state_b.field = vec![
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, 9, EMPTY, EMPTY, EMPTY],
            vec![9, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let heuristic_a = Bot::heuristic(&mut state_a);
        println!("HEURISTIC A: {}", heuristic_a);

        let heuristic_b = Bot::heuristic(&mut state_b);
        println!("HEURISTIC B: {}", heuristic_b);

        // heuristic_a should be greater than heuristic_b
        assert!(state_a.field.len() == 15);
        assert!(state_b.field.len() == 15);
        assert!(heuristic_a > heuristic_b);
    }

    #[test]
    fn test_try_place() {
        let mut state = State::initial_state(&vec![
            'X', 'I', 'Z', 'T', 'U', 'V', 'W', 'Y', 'L', 'P', 'N', 'F',
        ]);

        state.field = vec![
            vec![1, 1, EMPTY, EMPTY, EMPTY],
            vec![1, 1, EMPTY, EMPTY, EMPTY],
            vec![1, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
            vec![EMPTY, EMPTY, EMPTY, EMPTY, EMPTY],
        ];

        let piece_id = 10;
        let piece = vec![vec![1, 1], vec![1, 1], vec![1, 0]];

        let x = 0;
        let y = 0;

        let result = Bot::try_place(&mut state, &piece, &piece_id, x, y);

        assert_eq!(result, false);
    }
}
