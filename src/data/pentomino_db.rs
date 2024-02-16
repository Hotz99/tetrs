// Each line in the CSV file has 4 values + 1D array piece to define one permutation of a pentomino.
// - First value is the ID for a pentomino, from 0 to 11.
// - Second value is the index of the permutation (rotation, flip, etc.), between 0 to 7.
// - Third and fourth values are the X and Y sizes respectively.
// - The remaining values define an X*Y matrix, displaying the shape of the pentomino.

// This file does not contain a header.
// The pentominoes should be sorted by ID in increasing order

// EXAMPLE:

// 2,1,3,3,1,0,0,1,1,1,0,0,1

// ID: 2
// Permutation: 1
// X: 3 squares
// Y: 3 squares
// Shape:
// X 0 0
// X X X
// 0 0 X

use std::collections::HashSet;

pub struct PentominoDB {
    pub data: Vec<Vec<Vec<Vec<u8>>>>,
}

impl PentominoDB {
    pub fn new() -> PentominoDB {
        let data = PentominoDB::load_pentominoes();

        // Self::print_mutations(&data);

        PentominoDB { data }
    }

    fn print_mutations(mutations: &Vec<Vec<Vec<Vec<u8>>>>) {
        for pent_id in 0..mutations.len() {
            println!("{}", mutations[pent_id].len());

            for mutation in 0..mutations[pent_id].len() {
                println!("ID: {}", pent_id);
                println!("Permutation: {}", mutation);

                for y in 0..mutations[pent_id][mutation][0].len() {
                    for x in 0..mutations[pent_id][mutation].len() {
                        print!("{}", mutations[pent_id][mutation][x][y]);
                    }
                    println!();
                }
                println!();
            }
        }
    }

    // returns 3D vec:
    // 1D: piece ID, 2D: mutation, 3D: 2D vec piece representation
    pub fn load_pentominoes() -> Vec<Vec<Vec<Vec<u8>>>> {
        let mut pentominoes = Vec::<Vec<Vec<Vec<u8>>>>::new();

        // 'include_str!':
        // small csv file, so no memory overhead
        // loaded at compile time, so no runtime file-reading overhead
        let csv = include_str!("pentomino_db.csv");

        for line in csv.lines() {
            let mutation_data: Vec<u8> = line.split(',').map(|s| s.parse().unwrap()).collect();

            let pent_id = mutation_data[0] as usize;
            let permutation = mutation_data[1] as usize;
            let x_size = mutation_data[2] as usize;
            let y_size = mutation_data[3] as usize;

            let mut piece = vec![vec![0u8; y_size]; x_size];

            // 1d array to 2d array
            for i in 0..(x_size * y_size) {
                piece[i / y_size][i % y_size] = mutation_data[4 + i];
            }

            if pent_id >= pentominoes.len() {
                pentominoes.resize_with(pent_id + 1, Vec::new);
            }

            if permutation >= pentominoes[pent_id].len() {
                pentominoes[pent_id].resize_with(permutation + 1, Vec::new);
            }

            pentominoes[pent_id][permutation] = piece;
        }

        pentominoes
    }
}
