// Each line in the CSV file has 4 values + 1D array piece to define one permutation of a pentomino.
// - First value is the ID for a pentomino, from 1 to 12.
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

        // Self::print_data(&data);

        // print first mutation of each piece, including ID
        // for id in 0..data.len() {
        //     println!("ID: {}", id);
        //     for i in 0..data[id][0].len() {
        //         for j in 0..data[id][0][0].len() {
        //             print!("{}", data[id][0][i][j]);
        //         }
        //         println!();
        //     }
        //     println!();
        // }

        PentominoDB { data }
    }

    fn print_data(data: &Vec<Vec<Vec<Vec<u8>>>>) {
        for id in 0..data.len() {
            for permutation in 0..data[id].len() {
                println!("ID: {}", id);
                println!("Permutation: {}", permutation);

                for y in 0..data[id][permutation][0].len() {
                    for x in 0..data[id][permutation].len() {
                        print!("{}", data[id][permutation][x][y]);
                    }
                    println!();
                }
                println!();
            }
        }
    }

    // returns 3D vec:
    // dimensions: 1- piece ID; 2- mutation; 3- 2D vec piece representation
    pub fn load_pentominoes() -> Vec<Vec<Vec<Vec<u8>>>> {
        // 12 pentominoes, 4 permutations each
        let mut pentominoes: Vec<Vec<Vec<Vec<u8>>>> = vec![vec![Vec::new(); 4]; 12];

        // 'include_str!':
        // small csv file, so no memory overhead
        // loaded at compile time, so no runtime file-reading overhead
        let csv = include_str!("pentomino_db.csv");

        for line in csv.lines() {
            let values: Vec<u8> = line.split(',').map(|s| s.parse().unwrap()).collect();

            let piece_id = values[0] as usize;
            let permutation = values[1] as usize;
            let x_size = values[2] as usize;
            let y_size = values[3] as usize;

            let mut piece = vec![vec![0u8; y_size]; x_size];

            // 1d array to 2d array
            for i in 0..(x_size * y_size) {
                piece[i / y_size][i % y_size] = values[4 + i];
            }

            pentominoes[piece_id][permutation] = piece;
        }

        // remove duplicate mutations
        for id in 0..pentominoes.len() {
            let mut seen = HashSet::new();
            pentominoes[id].retain(|mutation| seen.insert(mutation.clone()));
        }

        pentominoes
    }
}
